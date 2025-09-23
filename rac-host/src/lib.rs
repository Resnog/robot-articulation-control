pub mod articulation;
pub mod kcontroller;

// TODO - default traits to send messages
trait SendMsg {
    fn send_to(&self);
}
