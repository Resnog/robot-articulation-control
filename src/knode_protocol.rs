use core::cmp::Ord;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KNodeMsg {
    sender: u8,
    receiver: u8,
    kind: KNodeMsgKind,
    payload: Option<KNodePayload>,
    priority: u8,
}

impl KNodeMsg {
    pub fn new(s: u8, r: u8, k: KNodeMsgKind, p: Option<KNodePayload>) -> Self {
        Self {
            sender: s,
            receiver: r,
            kind: k,
            payload: p,
            priority: 0,
        }
    }

    pub fn get_priotiry(&self) -> KNodeMsgKind {
        self.kind
    }
}

impl Ord for KNodeMsg {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.kind.cmp(&other.kind)
    }
}

impl PartialOrd for KNodeMsg {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum KNodeMsgKind {
    Heartbeat = 10,
    Command = 8,
    Response = 6,
    Debug = 2,
    Err = 255,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum KNodePayload {
    Command(KNodeCommand),
    Response(KNodeResponse),
    Heartbeat,
    Err(KNodeErr),
    Info {
        id: usize,
        data: [u8; 32],
        len: usize,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum KNodeErr {
    InitializationErr,
    BufferFull,
    Ok,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum KNodeCommand {
    Initialize,
    GetData,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum KNodeResponse {
    Initilized,
    DataSent,
}
