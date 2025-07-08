use core::cmp::Ord;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KNodeMsg {
    sender: u8,
    receiver: u8,
    kind: KNodeMsgKind,
    payload: KNodePayload,
}

impl KNodeMsg {
    pub fn get_priotiry(&self) -> KNodeMsgKind {
        self.kind
    }

    pub fn set_sender(&mut self, s: u8) {
        self.sender = s;
    }

    pub fn set_receiver(&mut self, r: u8) {
        self.receiver = r;
    }

    pub fn hearbeat() -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: KNodeMsgKind::Heartbeat,
            payload: KNodePayload::Heartbeat,
        }
    }

    pub fn command(cmd: KNodeCommand) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: KNodeMsgKind::Command,
            payload: KNodePayload::Command(cmd),
        }
    }

    pub fn response(rsp: KNodeResponse) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: KNodeMsgKind::Response,
            payload: KNodePayload::Response(rsp),
        }
    }

    pub fn error(err: KNodeErr) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: KNodeMsgKind::Err,
            payload: KNodePayload::Err(err),
        }
    }

    pub fn debug(id: usize, len: usize, data: [u8; 32]) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: KNodeMsgKind::Debug,
            payload: KNodePayload::Info { id, len, data },
        }
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
        len: usize,
        data: [u8; 32],
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
