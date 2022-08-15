#![no_main]
use apdu_dispatch::app::Result as AppResult;
use apdu_dispatch::{dispatch::Interface, interchanges, iso7816, App};
use arbitrary::{Arbitrary, Unstructured};
use interchange::Interchange;
use libfuzzer_sys::fuzz_target;

use std::convert::TryFrom;

#[derive(Debug)]
struct StatusWrapper(iso7816::Status);

impl<'a> Arbitrary<'a> for StatusWrapper {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<StatusWrapper, arbitrary::Error> {
        let (tag1, tag2) = u.arbitrary()?;
        Ok(StatusWrapper(
            iso7816::Status::try_from((tag1, tag2))
                .map_err(|_| arbitrary::Error::IncorrectFormat)?,
        ))
    }
}

#[derive(Debug, Arbitrary)]
struct Input {
    apps: Vec<FuzzApp>,
    apdus: Vec<(Vec<u8>, bool)>,
}

#[derive(Debug, Arbitrary)]
struct FuzzApp {
    responses: Vec<(Vec<u8>, Option<StatusWrapper>)>,
}

struct FuzzAppImpl {
    id: usize,
    responses: Vec<(Vec<u8>, Option<iso7816::Status>)>,
    count: usize,
}

impl FuzzAppImpl {
    fn new(id: usize, app: FuzzApp) -> Self {
        FuzzAppImpl {
            responses: app
                .responses
                .into_iter()
                .map(|(data, maybe_status)| (data, maybe_status.map(|s| s.0)))
                .collect(),
            id,
            count: 0,
        }
    }
}

impl iso7816::App for FuzzAppImpl {
    fn aid(&self) -> iso7816::Aid {
        let [b1, b2] = (self.id as u16).to_be_bytes();
        iso7816::Aid::new(&[0x0Au8, 1, 0, b1, b2])
    }
}

impl App<{ apdu_dispatch::command::SIZE }, { apdu_dispatch::response::SIZE }> for FuzzAppImpl {
    fn select(
        &mut self,
        _apdu: &apdu_dispatch::Command,
        _reply: &mut apdu_dispatch::response::Data,
    ) -> AppResult {
        Ok(Default::default())
    }

    fn deselect(&mut self) {}

    fn call(
        &mut self,
        _: Interface,
        _apdu: &apdu_dispatch::Command,
        reply: &mut apdu_dispatch::response::Data,
    ) -> AppResult {
        let (ref data, status) = &self.responses[self.count];
        reply.extend_from_slice(&data).ok();
        self.count += 1;
        self.count = self.count % self.responses.len();
        match status {
            Some(s) => Err(s.clone()),
            None => Ok(()),
        }
    }
}

fuzz_target!(|input: Input| {
    unsafe { interchanges::Contact::reset_claims() };
    unsafe { interchanges::Contactless::reset_claims() };
    let mut apps: Vec<_> = input
        .apps
        .into_iter()
        .enumerate()
        .map(|(idx, app)| FuzzAppImpl::new(idx, app))
        .collect();
    let mut dyn_apps: Vec<_> = apps
        .iter_mut()
        .map(|s| (s as &mut dyn apdu_dispatch::App<7609, 7609>))
        .collect();

    let (mut contact_requester, contact_responder) =
        interchanges::Contact::claim().expect("could not setup ccid ApduInterchange");

    let (mut contactless_requester, contactless_responder) =
        interchanges::Contactless::claim().expect("could not setup iso14443 ApduInterchange");

    let mut apdu_dispatch =
        apdu_dispatch::dispatch::ApduDispatch::new(contact_responder, contactless_responder);

    for (mut apdu, requester) in input.apdus {
        apdu.truncate(interchanges::SIZE);
        if requester {
            contact_requester
                .request(&interchanges::Data::from_slice(&apdu).unwrap())
                .expect("could not deposit command");
        } else {
            contactless_requester
                .request(&interchanges::Data::from_slice(&apdu).unwrap())
                .expect("could not deposit command");
        }
        apdu_dispatch.poll(&mut dyn_apps);

        if requester {
            contact_requester.take_response().unwrap();
        } else {
            contactless_requester.take_response().unwrap();
        }
    }
});
