#![no_std]

pub use iso7816::{command::CommandView, Data, Interface, Status};

pub type Result = iso7816::Result<()>;

/// An App can receive and respond APDUs at behest of the ApduDispatch.
pub trait App<const R: usize>: iso7816::App {
    /// Given parsed APDU for select command.
    /// Write response data back to buf, and return length of payload.  Return APDU Error code on error.
    /// Alternatively, the app can defer the response until later by returning it in `poll()`.
    fn select(
        &mut self,
        interface: Interface,
        apdu: CommandView<'_>,
        reply: &mut Data<R>,
    ) -> Result;

    /// Deselects the app. This is the result of another app getting selected.
    /// App should clear any sensitive state and reset security indicators.
    fn deselect(&mut self);

    /// Given parsed APDU for app when selected.
    /// Write response data back to buf, and return length of payload.  Return APDU Error code on error.
    fn call(&mut self, interface: Interface, apdu: CommandView<'_>, reply: &mut Data<R>) -> Result;
}
