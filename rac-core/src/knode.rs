use crate::articulation::{CoreArticulationStatus, CoreArticulationVariant};
use crate::Status;
use heapless::binary_heap::{BinaryHeap, Max};
use rac_protocol::knode_protocol::{KNodeErr, KNodeMsg};

/// Description of an CoreArticulation in order to be represented in a Graph
pub struct KNode {
    id: usize,
    status: Status,
    art_status: Option<CoreArticulationStatus>,
    rx_queue: BinaryHeap<KNodeMsg, Max, 8>,
    tx_queue: BinaryHeap<KNodeMsg, Max, 8>,
}

impl KNode {
    pub fn new(new_id: usize) -> Self {
        Self {
            id: new_id,
            status: Status::Uninitialized,
            art_status: Option::None,
            rx_queue: BinaryHeap::new(),
            tx_queue: BinaryHeap::new(),
        }
    }

    pub fn init(&mut self) {
        self.status = Status::Active;
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
}
