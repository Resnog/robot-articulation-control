pub struct KNodeMsg {
    sender: u8,
    receiver: u8,
    kind: KNodeMsgKind,
    payload: KNodePayload,
}

pub enum KNodeMsgKind {
    Command(u8),
    Heartbeat,
    Err(KNodeErr),
}

pub enum KNodePayload {
    Err(KNodeErr),
    None,
}

pub enum KNodeErr {
    WrongState,
    Fault,
}
