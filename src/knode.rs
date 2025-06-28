use crate::articulation::{ArticulationStatus, ArticulationVariant};
use crate::knode_protocol::{KNodeErr, KNodeMsg};
use crate::Status;
use heapless::binary_heap::{BinaryHeap, Max};

/// Description of an Articulation in order to be represented in a Graph
struct KNode {
    id: usize,
    status: Status,
    art_status: Option<ArticulationStatus>,
    qin: BinaryHeap<KNodeMsg, Max, 8>,
    qout: BinaryHeap<KNodeMsg, Max, 8>,
}

impl KNode {
    pub fn new(new_id: usize) -> Self {
        Self {
            id: new_id,
            status: Status::Uninitialized,
            art_status: Option::None,
            qin: BinaryHeap::new(),
            qout: BinaryHeap::new(),
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
        match &self.qout.push(msg) {
            Ok(_) => KNodeErr::Ok,
            Err(_) => KNodeErr::BufferFull,
        }
    }

    pub fn read(&mut self) -> Result<KNodeMsg, KNodeErr> {
        match &self.qin.pop() {
            Some(msg) => Ok(*msg),
            None => Err(KNodeErr::BufferFull),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::knode_protocol::{KNodeMsg, KNodeMsgKind};

    fn channel_send_knodemsg(sender: &mut KNode, receiver: &mut KNode) {
        while let Some(sent_msg) = sender.qout.pop() {
            receiver.qin.push(sent_msg).unwrap();
        }
    }

    #[test]
    fn send_heartbeat() {
        let mut sender = KNode::new(1);
        let mut receiver = KNode::new(2);

        // Fill the sender queue
        for _ in 0..8 {
            let msg = KNodeMsg::new(1, 2, KNodeMsgKind::Heartbeat, None);
            assert_eq!(&sender.send(msg), &KNodeErr::Ok);
        }

        // Overflow the buffer sending one extra message
        let msg = KNodeMsg::new(1, 2, KNodeMsgKind::Heartbeat, None);
        assert_eq!(sender.send(msg), KNodeErr::BufferFull);

        // Send the msgs to the receiver
        channel_send_knodemsg(&mut sender, &mut receiver);

        // Empty the receiver queue
        for _ in 0..8 {
            let msg = KNodeMsg::new(1, 2, KNodeMsgKind::Heartbeat, None);
            assert_eq!(&receiver.send(msg), &KNodeErr::Ok);
        }
    }

    #[test]
    fn check_msg_priority() {
        let mut knode = KNode::new(1);

        // TODO: Fix the msg kind to be consistent with the msg payload
        let msgs: [KNodeMsg; 5] = [
            KNodeMsg::new(1, 2, KNodeMsgKind::Heartbeat, None),
            KNodeMsg::new(1, 2, KNodeMsgKind::Command, None),
            KNodeMsg::new(1, 2, KNodeMsgKind::Debug, None),
            KNodeMsg::new(1, 2, KNodeMsgKind::Err, None),
            KNodeMsg::new(1, 2, KNodeMsgKind::Response, None),
        ];

        for i in 0..5 {
            let _ = knode.qin.push(msgs[i]);
        }

        assert_eq!(
            knode.read().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Err
        );

        assert_eq!(
            knode.read().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Heartbeat
        );

        assert_eq!(
            knode.read().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Command
        );

        assert_eq!(
            knode.read().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Response
        );

        assert_eq!(
            knode.read().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Debug
        );
    }
}
