use rac_core::knode::KNode;
use rac_core::Status;
use rac_protocol::knode_protocol::{KNodeCommand, KNodeErr, KNodeMsg, KNodeResponse};
use std::collections::{BinaryHeap, HashMap};

struct KNodeInfo {
    id: u8,
    status: Status,
    timeout: usize,
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
    NodeIDInvalid,
    MsgOutBuffFull,
}

///  RAC Host KController
///
///  The KController is the RAC Host interface between the main controller and the nodes
///  in the network.
///
struct KController {
    id: u8,
    nodes: HashMap<u8, KNodeInfo>,
    status: Status,
    msgs_in: BinaryHeap<KNodeMsg>,
    msgs_out: BinaryHeap<KNodeMsg>,
}

impl KController {
    pub fn new() -> Self {
        KController {
            id: u8::MAX,
            nodes: HashMap::new(),
            status: Status::Uninitialized,
            msgs_in: BinaryHeap::new(),
            msgs_out: BinaryHeap::new(),
        }
    }

    // TODO - Change return value to Result<Status, KControllerErr>
    fn init_node(self, id: u8) -> Result<Status, KControllerErr> {
        match self.nodes.get(&id) {
            // Check what is the registered status of the KNode
            Some(v) => {
                // TODO - Check if the node is not an actual articulation, like
                // a sensor or a virtual articulation
                // Send the KNodeCommand::Initialize to the valid node
                self.send_init(v.id);
                // Set timout
                self.set_init_timeout(v.timeout);
                // Set status to KController::NodeInitializing
                Ok(Status::Initializing)
            }
            None => Err(KControllerErr::NodeIDInvalid),
        }
    }

    /// Send the init command to an Uninitialized node
    fn send_init(&self, id: u8) -> Result<Status, KControllerErr> {
        let cmd_init = KNodeMsg::command(KNodeCommand::Initialize);
        cmd_init.set_sender(self.id);
        cmd_init.set_receiver(id);
        if let Err(_) = self.msgs_out.push(cmd_init) {
            // Well crap, I forgot how to use the if let, I want
            // to check is there was an actual error here
            Err(KControllerErr::MsgOutBuffFull)
        }
        Ok(Status::MsgSendSuccessfully)
    }

    fn set_init_timeout(&self, timeout: usize) -> KControllerErr {
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
        let mut kcont = KController::new();
        let debug_data = [42u8; 32];
        let msgs: [KNodeMsg; 5] = [
            KNodeMsg::heartbeat(),
            KNodeMsg::command(KNodeCommand::Initialize),
            KNodeMsg::debug(0, 8, debug_data),
            KNodeMsg::error(KNodeErr::InitializationErr),
            KNodeMsg::response(KNodeResponse::Initilized),
        ];

        for i in 0..5 {
            let _ = kcont.msgs_out.push(msgs[i]);
        }

        assert_eq!(
            kcont.msgs_out.pop().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Err
        );

        assert_eq!(
            kcont.msgs_out.pop().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Heartbeat
        );

        assert_eq!(
            kcont.msgs_out.pop().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Command
        );

        assert_eq!(
            kcont.msgs_out.pop().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Response
        );

        assert_eq!(
            kcont.msgs_out.pop().expect("Expected Ok").get_priotiry(),
            KNodeMsgKind::Debug
        );
    }

    /// Check the KController sends a heartbeat to a KNode
    #[test]
    fn kcontroller_recv_hearthbeat() {
        todo!();
    }
}
