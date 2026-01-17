use crate::articulation::{CoreArticulationStatus, CoreArticulationVariant};
use crate::Status;
use heapless::binary_heap::{BinaryHeap, Max};

/// Description of an CoreArticulation in order to be represented in a Graph
pub struct KNode {
    pub id: u8,
    pub status: Status,
    art_status: Option<CoreArticulationStatus>,
    pub rx_queue: BinaryHeap<Msg, Max, 8>,
    pub tx_queue: BinaryHeap<Msg, Max, 8>,
    pub controller_id: u8,
    pub heartbeat_timeout: u32,
}

impl KNode {
    pub fn new(new_id: u8) -> Self {
        Self {
            id: new_id,
            status: Status::Uninitialized,
            art_status: Option::None,
            rx_queue: BinaryHeap::new(),
            tx_queue: BinaryHeap::new(),
            controller_id: 0,
            heartbeat_timeout: 0,
        }
    }

    pub fn init(&mut self, controller_id: u8, timeout: u32) {
        self.status = Status::Active;
        self.controller_id = controller_id;
        self.heartbeat_timeout = timeout;

        // At this point the KNode is Initializing so we
        // can safely assume that the queues are not full
        // TODO - Send a response instead of a heartbeat
        let _ = self.tx_queue.push(
            Msg::heartbeat()
                .set_sender(self.id)
                .set_receiver(self.controller_id),
        );
    }

    pub fn register_art(&mut self, art: &CoreArticulationVariant) {
        match art {
            CoreArticulationVariant::F32(a) => self.art_status = Some(a.get_status()),
            CoreArticulationVariant::F64(a) => self.art_status = Some(a.get_status()),
        }
    }

    /// Get a message sent by the KController and process it
    pub fn process(&mut self) {
        let msg = self.rx_queue.pop().unwrap();
        // Check what message the KController sent and update state accoringly
        match msg.get_priotiry() {
            MsgKind::Command => self.commands_handler(msg),
            _ => return,
        }
        // For the init message change status to Initializing
        // Branch to the actual Initializing
        // Push to the TX queue a Initialized messaged if successful
        // Push to the TX queue an Err
    }

    /// KNode command handler
    fn commands_handler(&mut self, msg: Msg) {
        if let Payload::Command(cmd) = msg.payload {
            // Check the command type
            match cmd {
                Command::Initialize { kcont_id, timeout } => self.init(kcont_id, timeout),
                _ => (),
            };
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Msg {
    sender: u8,
    receiver: u8,
    pub kind: MsgKind,
    pub payload: Payload,
}

impl Msg {
    pub fn get_priotiry(&self) -> MsgKind {
        self.kind
    }

    pub fn set_sender(mut self, s: u8) -> Self {
        self.sender = s;
        self
    }

    pub fn set_receiver(mut self, r: u8) -> Self {
        self.receiver = r;
        self
    }

    pub fn heartbeat() -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: MsgKind::Heartbeat,
            payload: Payload::Heartbeat,
        }
    }

    pub fn command(cmd: Command) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: MsgKind::Command,
            payload: Payload::Command(cmd),
        }
    }

    pub fn response(rsp: Response) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: MsgKind::Response,
            payload: Payload::Response(rsp),
        }
    }

    pub fn error(err: Err) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: MsgKind::Err,
            payload: Payload::Err(err),
        }
    }

    pub fn debug(id: usize, len: usize, data: [u8; 32]) -> Self {
        Self {
            sender: 0,
            receiver: 0,
            kind: MsgKind::Debug,
            payload: Payload::Info { id, len, data },
        }
    }
}

impl Ord for Msg {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.kind.cmp(&other.kind)
    }
}

impl PartialOrd for Msg {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum MsgKind {
    Heartbeat = 10,
    Command = 8,
    Response = 6,
    Debug = 2,
    Err = 255,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Payload {
    Command(Command),
    Response(Response),
    Heartbeat,
    Err(Err),
    Info {
        id: usize,
        len: usize,
        data: [u8; 32],
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Err {
    InitializationErr,
    BufferEmpty,
    BufferFull,
    Ok,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Command {
    InvalidCommand,
    Initialize { kcont_id: u8, timeout: u32 },
    GetData,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Response {
    InvalidResponse,
    Initilized,
    DataSent,
}
