use crate::articulation::{ArticulationStatus, ArticulationVariant};
use crate::knode_protocol::KNodeMsg;
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
    pub fn new(self, new_id: usize) -> Self {
        Self {
            id: new_id,
            status: Status::Uninitialized,
            art_status: Option::None,
            qin: Q8::new(),
            qout: Q8::new(),
        }
    }

    pub fn register_art(&mut self, art: &ArticulationVariant) {
        match art {
            ArticulationVariant::F32(a) => self.art_status = Some(a.get_status()),
            ArticulationVariant::F64(a) => self.art_status = Some(a.get_status()),
        }
    }
}
