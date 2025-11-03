use core::time;
use rac_core::knode::KNode;
use rac_core::Status;
use rac_protocol::knode_protocol::{KNodeCommand, KNodeErr, KNodeMsg, KNodeResponse};
use std::{
    collections::{BinaryHeap, HashMap},
    usize,
};

/// Default KNode timeout in ms
static KNODE_DEF_TIMEOUT: u32 = 500;

struct KNodeInfo {
    status: Status,
    timeout: u32,
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
    htimeout: usize,
}

impl KController {
    pub fn new() -> Self {
        KController {
            id: u8::MAX,
            nodes: HashMap::new(),
            status: Status::Uninitialized,
            msgs_in: BinaryHeap::new(),
            msgs_out: BinaryHeap::new(),
            htimeout: usize::MAX,
        }
    }

    fn init(&mut self) {
        // Send an init command to all the KNodes in the network
        for (id, node_info) in &self.nodes {
            let cmd_init = KNodeMsg::command(KNodeCommand::Initialize {
                kcont_id: self.id,
                timeout: node_info.timeout,
            })
            .set_sender(self.id)
            .set_receiver(id.clone());
            self.msgs_out.push(cmd_init);
            // Stablish a timeout to track the node initialization process - TODO
        }

        self.status = Status::Initializing;
    }

    pub fn add_node(&mut self, node: &KNode, timeout: Option<u32>) {
        let ntimeout = match timeout {
            None => KNODE_DEF_TIMEOUT,
            Some(num) => num,
        };

        let node_info = KNodeInfo {
            status: Status::Uninitialized,
            timeout: ntimeout,
            last_cmd: KNodeCommand::InvalidCommand,
            last_rsp: KNodeResponse::InvalidResponse,
        };

        self.nodes.insert(node.id, node_info);
    }

    // TODO - read articulation
}

/// The KMonitor keeps track of the state of the nodes on the KController
/// and their respective timeouts, so that the KController reacts accordingly

// TODO - Check if it is worth to implement an actual KMonitor

/// KNode unit testin
#[cfg(test)]
mod test {
    use super::*;
    use rac_core::knode::KNode;
    use rac_protocol::knode_protocol::{
        KNodeCommand, KNodeErr, KNodeMsg, KNodeMsgKind, KNodeResponse,
    };

    /// Virtual channel between nodes for KNode priority checks
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

    /// Check the KNodeMsg priotity when emptying a KNode queue
    #[test]
    fn check_msg_priority() {
        let mut knode = KNode::new(1);
        let debug_data = [42u8; 32];

        let msgs: [KNodeMsg; 5] = [
            KNodeMsg::heartbeat(),
            KNodeMsg::command(KNodeCommand::Initialize {
                kcont_id: 255,
                timeout: KNODE_DEF_TIMEOUT,
            }),
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
            KNodeMsg::command(KNodeCommand::Initialize {
                kcont_id: 255,
                timeout: KNODE_DEF_TIMEOUT,
            }),
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
    /// This test is a bit lacking since the data is has not been serialized. We need to serialize the data correctly and then send it correctly.
    ///
    /// This test might have to be modified when we add an abstraction layer to send messages through an interface.
    #[test]
    fn kcontroller_send_heartbeat() {
        let mut kcont = KController::new();

        let mut knode = KNode::new(1);

        kcont.add_node(&knode, Some(KNODE_DEF_TIMEOUT));

        kcont.init();

        assert_eq!(kcont.status, Status::Initializing);

        // Pop the KController messages and insert these into the KNode
        knode.rx_enqueue(kcont.msgs_out.pop().unwrap());

        // Check that the messages are processed
        knode.process();
        assert_eq!(knode.status, Status::Active);
        assert_eq!(knode.controller_id, kcont.id);
        assert_eq!(knode.heartbeat_timeout, KNODE_DEF_TIMEOUT);
    }
}
