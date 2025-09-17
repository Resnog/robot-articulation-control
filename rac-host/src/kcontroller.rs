use rac_core::knode::KNode;
use rac_core::Status;
use rac_protocol::knode_protocol::{KNodeCommand, KNodeErr, KNodeMsg, KNodeResponse};
use std::{
    collections::{BinaryHeap, HashMap},
    os::linux::raw::stat,
};

struct KNodeInfo {
    name: usize,
    status: Status,
    last_cmd: KNodeCommand,
    last_rsp: KNodeResponse,
}

impl KNodeInfo {
    pub fn is_node_active(&self) -> bool {
        match self.status {
            Status::Active => true,
            _ => false,
        }
    }
}

enum KControllerErr {
    NodeSilent,
    NodeInitializing,
    NodeIDInvalid,
    Ok,
}

///  RAC Host KController
///
///  The KController is the RAC Host interface between the main controller and the nodes
///  in the network.
///
struct KController {
    nodes: HashMap<usize, KNodeInfo>,
    status: Status,
    msgs_in: BinaryHeap<KNodeMsg>,
    msgs_out: BinaryHeap<KNodeMsg>,
}

impl KController {
    pub fn new(self) -> Self {
        KController {
            nodes: HashMap::new(),
            status: Status::Uninitialized,
            msgs_in: BinaryHeap::new(),
            msgs_out: BinaryHeap::new(),
        }
    }

    fn init_node(self, id: usize) -> KControllerErr {
        let mut status = KControllerErr::NodeSilent;

        match self.nodes.get(&id) {
            // Check what is the registered status of the KNode
            Some(v) => {
                // Send the KNodeCommand::Initialize to the valid node
                // Set status to KController::NodeInitializing
                status = KControllerErr::NodeInitializing;
                // Set timout
            }
            None => {
                status = KControllerErr::NodeIDInvalid;
            }
        };
        status
    }

    /// Send the init command to an Uninitialized node
    /// What happens if the node is Initilized ?
    pub fn send_init(self, id: usize) -> KControllerErr {
        todo!()
    }

    pub fn set_timeout(timeout: u32) -> KControllerErr {
        todo!()
    }
}

/// KNode unit testin
#[cfg(test)]
mod test {
    use super::*;
    use rac_core::knode::KNode;
    use rac_protocol::knode_protocol::{
        KNodeCommand, KNodeErr, KNodeMsg, KNodeMsgKind, KNodeResponse,
    };

    fn channel_send_knodemsg(sender: &mut KNode, receiver: &mut KNode) {
        while let Ok(sent_msg) = sender.tx_dequeue() {
            receiver.rx_enqueue(sent_msg);
        }
    }

    #[test]
    fn knode_send_heartbeat() {
        let mut sender = KNode::new(1);
        let mut receiver = KNode::new(2);

        // Fill the sender queue
        for _ in 0..8 {
            let msg = KNodeMsg::heartbeat();
            assert_eq!(sender.tx_enqueue(msg), KNodeErr::Ok);
        }

        // Overflow the buffer sending one extra message
        let msg = KNodeMsg::heartbeat();
        assert_eq!(sender.tx_enqueue(msg), KNodeErr::BufferFull);

        // Send the msgs to the receiver
        channel_send_knodemsg(&mut sender, &mut receiver);

        // Empty the receiver queue
        for _ in 0..8 {
            assert_eq!(receiver.rx_dequeue(), Ok(KNodeMsg::heartbeat()));
        }
    }

    #[test]
    /// Check the KNodeMsg priotity when emptying a KNode queue
    fn check_msg_priority() {
        let mut knode = KNode::new(1);
        let debug_data = [42u8; 32];

        let msgs: [KNodeMsg; 5] = [
            KNodeMsg::heartbeat(),
            KNodeMsg::command(KNodeCommand::Initialize),
            KNodeMsg::debug(0, 8, debug_data),
            KNodeMsg::error(KNodeErr::InitializationErr),
            KNodeMsg::response(KNodeResponse::Initilized),
        ];

        for i in 0..5 {
            let _ = knode.tx_enqueue(msgs[i]);
        }

        assert_eq!(
            knode.tx_dequeue().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Err
        );

        assert_eq!(
            knode.tx_dequeue().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Heartbeat
        );

        assert_eq!(
            knode.tx_dequeue().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Command
        );

        assert_eq!(
            knode.tx_dequeue().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Response
        );

        assert_eq!(
            knode.tx_dequeue().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Debug
        );
    }

    #[test]
    /// Check the KController message priority
    fn kcontroller_msg_priority() {
        todo!();
    }

    #[test]
    fn kcontroller_recv_hearthbeat() {
        todo!();
    }
}
