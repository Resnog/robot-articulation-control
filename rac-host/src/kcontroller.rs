/// KNode unit testing
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
    fn send_heartbeat() {
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
}
