use crate::articulation::{CoreArticulationStatus, CoreArticulationVariant};
use crate::Status;
use heapless::binary_heap::{BinaryHeap, Max};
use rac_protocol::knode_protocol::{KNodeCommand, KNodeErr, KNodeMsg, KNodeMsgKind, KNodePayload};

/// Description of an CoreArticulation in order to be represented in a Graph
pub struct KNode {
    pub id: u8,
    pub status: Status,
    art_status: Option<CoreArticulationStatus>,
    rx_queue: BinaryHeap<KNodeMsg, Max, 8>,
    tx_queue: BinaryHeap<KNodeMsg, Max, 8>,
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

        self.tx_enqueue(
            KNodeMsg::heartbeat()
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

    // For RX Queue
    pub fn rx_enqueue(&mut self, msg: KNodeMsg) -> KNodeErr {
        match self.rx_queue.push(msg) {
            Ok(_) => KNodeErr::Ok,
            Err(_) => KNodeErr::BufferFull,
        }
    }

    pub fn rx_dequeue(&mut self) -> Result<KNodeMsg, KNodeErr> {
        match self.rx_queue.pop() {
            Some(msg) => Ok(msg),
            None => Err(KNodeErr::BufferFull),
        }
    }

    // For TX Queue
    pub fn tx_enqueue(&mut self, msg: KNodeMsg) -> KNodeErr {
        match self.tx_queue.push(msg) {
            Ok(_) => KNodeErr::Ok,
            Err(_) => KNodeErr::BufferFull,
        }
    }

    pub fn tx_dequeue(&mut self) -> Result<KNodeMsg, KNodeErr> {
        match self.tx_queue.pop() {
            Some(msg) => Ok(msg),
            None => Err(KNodeErr::BufferFull),
        }
    }

    /// Get a message sent by the KController and process it
    pub fn process(&mut self) {
        let msg = self.rx_dequeue().unwrap();
        // Check what message the KController sent and update state accoringly
        match msg.get_priotiry() {
            KNodeMsgKind::Command => self.commands_handler(msg),
            _ => return,
        }
        // For the init message change status to Initializing
        // Branch to the actual Initializing
        // Push to the TX queue a Initialized messaged if successful
        // Push to the TX queue an Err
    }

    /// KNode command handler
    fn commands_handler(&mut self, msg: KNodeMsg) {
        if let KNodePayload::Command(cmd) = msg.payload {
            // Check the command type
            match cmd {
                KNodeCommand::Initialize { kcont_id, timeout } => self.init(kcont_id, timeout),
                _ => (),
            };
        }
    }
}
