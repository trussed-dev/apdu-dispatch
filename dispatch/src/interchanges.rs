pub const SIZE: usize = 3072;
pub type Data = iso7816::Data<SIZE>;
pub type Responder<'pipe> = interchange::Responder<'pipe, Data, Data>;
pub type Requester<'pipe> = interchange::Requester<'pipe, Data, Data>;
pub type Channel = interchange::Channel<Data, Data>;
