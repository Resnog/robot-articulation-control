pub mod articulation;
pub mod kcontroller;
pub mod kgraph;

// TODO - default traits to send messages
trait SendMsg {
    fn send_to(&self);
}
