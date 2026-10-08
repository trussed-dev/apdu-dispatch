use apdu_app::{App, Result};
use apdu_dispatch::{
    dispatch::ApduDispatch,
    interchanges::{Channel, Data, Requester},
};
use heapless::VecView;
use hex_literal::hex;
use iso7816::{command::CommandView, Aid, Interface};

const AID_PIV: &[u8] = &hex!("A000000308000010000100");
const AID_OPENPGP: &[u8] = &hex!("D276000124010304000FFFFFFFFF0000");

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Calls {
    select: usize,
    deselect: usize,
    call: usize,
}

struct TestApp {
    aid: Aid,
    calls: Calls,
}

impl TestApp {
    fn new(aid: Aid) -> Self {
        Self {
            aid,
            calls: Calls::default(),
        }
    }
}

impl iso7816::App for TestApp {
    fn aid(&self) -> Aid {
        self.aid
    }
}

impl App for TestApp {
    fn select(
        &mut self,
        _interface: Interface,
        _apdu: CommandView<'_>,
        _reply: &mut VecView<u8>,
    ) -> Result {
        self.calls.select += 1;
        Ok(())
    }

    fn deselect(&mut self) {
        self.calls.deselect += 1;
    }

    fn call(
        &mut self,
        _interface: Interface,
        _apdu: CommandView<'_>,
        _reply: &mut VecView<u8>,
    ) -> Result {
        self.calls.call += 1;
        Ok(())
    }
}

struct Channels {
    contact: Channel,
    contactless: Channel,
}

impl Channels {
    fn new() -> Self {
        Self {
            contact: Channel::new(),
            contactless: Channel::new(),
        }
    }
}

struct Dispatch<'a, const N: usize> {
    contact_rq: Requester<'a>,
    contactless_rq: Requester<'a>,
    dispatch: ApduDispatch<'a>,
    apps: [TestApp; N],
}

impl<'a, const N: usize> Dispatch<'a, N> {
    fn new(channels: &'a Channels, aids: [Aid; N]) -> Self {
        let (contact_rq, contact_rp) = channels.contact.split().unwrap();
        let (contactless_rq, contactless_rp) = channels.contactless.split().unwrap();
        let dispatch = ApduDispatch::new(contact_rp, contactless_rp);
        let apps = aids.map(TestApp::new);
        Self {
            contact_rq,
            contactless_rq,
            dispatch,
            apps,
        }
    }

    fn send_command(&mut self, interface: Interface, command: &[u8]) -> Option<Data> {
        let rq = match interface {
            Interface::Contact => &mut self.contact_rq,
            Interface::Contactless => &mut self.contactless_rq,
        };
        rq.request(command.try_into().unwrap()).unwrap();
        let mut apps: Vec<&mut dyn App> = self.apps.iter_mut().map(|app| app as _).collect();
        self.dispatch.poll(&mut apps);
        rq.take_response()
    }

    fn calls(&self) -> [Calls; N] {
        self.apps.each_ref().map(|app| app.calls)
    }
}

fn select(aid: &[u8]) -> Vec<u8> {
    let mut cmd = Vec::from(hex!("00A40400"));
    cmd.push(aid.len().try_into().unwrap());
    cmd.extend(aid);
    cmd
}

#[test]
fn test_simple() {
    let aids = [Aid::try_new(AID_PIV).unwrap()];
    let mut calls = [Calls::default()];
    let channels = Channels::new();
    let mut dispatch = Dispatch::new(&channels, aids);
    assert_eq!(dispatch.calls(), calls);

    let dummy_cmd = hex!("00000000");
    let select_cmd = select(AID_PIV);

    let response = dispatch.send_command(Interface::Contact, &dummy_cmd);
    assert_eq!(response.unwrap(), &hex!("6A82"));
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &select_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[0].select += 1;
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &select_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[0].select += 1;
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &dummy_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[0].call += 1;
    assert_eq!(dispatch.calls(), calls);
}

#[test]
fn test_multi() {
    let aids = [
        Aid::try_new(AID_PIV).unwrap(),
        Aid::try_new(AID_OPENPGP).unwrap(),
    ];
    let mut calls = [Calls::default(), Calls::default()];
    let channels = Channels::new();
    let mut dispatch = Dispatch::new(&channels, aids);
    assert_eq!(dispatch.calls(), calls);

    let dummy_cmd = hex!("00000000");
    let select_piv_cmd = select(AID_PIV);
    let select_openpgp_cmd = select(AID_OPENPGP);

    let response = dispatch.send_command(Interface::Contact, &dummy_cmd);
    assert_eq!(response.unwrap(), &hex!("6A82"));
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &select_piv_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[0].select += 1;
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &select_piv_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[0].select += 1;
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &dummy_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[0].call += 1;
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &select_openpgp_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    // TODO: this is a bug -- the first application should be deselected, not the second
    calls[1].deselect += 1;
    calls[1].select += 1;
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &select_openpgp_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[1].select += 1;
    assert_eq!(dispatch.calls(), calls);

    let response = dispatch.send_command(Interface::Contact, &dummy_cmd);
    assert_eq!(response.unwrap(), &hex!("9000"));
    calls[1].call += 1;
    assert_eq!(dispatch.calls(), calls);
}
