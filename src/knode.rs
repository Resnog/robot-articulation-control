use crate::articulation::{ArticulationStatus, ArticulationVariant};
use crate::knode_protocol::{KNodeErr, KNodeMsg};
use crate::Status;
use heapless::mpmc::Q8;

/// Description of an Articulation in order to be represented in a Graph
struct KNode {
    id: usize,
    status: Status,
    art_status: Option<ArticulationStatus>,
    qin: Q8<KNodeMsg>,
    qout: Q8<KNodeMsg>,
}

impl KNode {
    pub fn new(new_id: usize) -> Self {
        Self {
            id: new_id,
            status: Status::Uninitialized,
            art_status: Option::None,
            qin: Q8::new(),
            qout: Q8::new(),
        }
    }

    pub fn init(&mut self) {
        self.status = Status::Active;
    }

    pub fn register_art(&mut self, art: &ArticulationVariant) {
        match art {
            ArticulationVariant::F32(a) => self.art_status = Some(a.get_status()),
            ArticulationVariant::F64(a) => self.art_status = Some(a.get_status()),
        }
    }

    pub fn send(&mut self, msg: KNodeMsg) -> KNodeErr {
        match &self.qout.enqueue(msg) {
            Ok(_) => KNodeErr::Ok,
            Err(_) => KNodeErr::BufferFull,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::knode_protocol::{KNodeMsg, KNodeMsgKind};
    #[test]
    fn send_heartbeat() {
        let mut node = KNode::new(1);

        // Fill the queue
        for _ in 0..7 {
            let msg = KNodeMsg::new(1, 2, KNodeMsgKind::Heartbeat, None);
            assert_eq!(node.send(msg), KNodeErr::Ok);
        }

        // Overflow the buffer
        let msg = KNodeMsg::new(1, 2, KNodeMsgKind::Heartbeat, None);
        assert_eq!(node.send(msg), KNodeErr::BufferFull);
    }
}
