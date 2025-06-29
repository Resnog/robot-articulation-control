pub struct KNodeMsg {
    sender: u8,
    receiver: u8,
    kind: KNodeMsgKind,
    payload: Option<KNodePayload>,
}

impl KNodeMsg {
    pub fn new(s: u8, r: u8, k: KNodeMsgKind, p: Option<KNodePayload>) -> Self {
        Self {
            sender: s,
            receiver: r,
            kind: k,
            payload: p,
        }
    }
}

pub enum KNodeMsgKind {
    Command(KNodeCommand),
    Response(KNodeResponse),
    Heartbeat,
    Err(KNodeErr),
}

pub enum KNodePayload {
    Info {
        id: usize,
        data: [u8; 32],
        len: usize,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KNodeErr {
    InitializationErr,
    BufferFull,
    Ok,
}

pub enum KNodeCommand {
    Initialize,
    GetData,
}

pub enum KNodeResponse {
    Initilized,
    DataSent,
}
