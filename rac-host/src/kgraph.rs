use crate::articulation::Articulation;

struct KGraph<T> {
    /// The theorerical state of the articulations is stored for the
    /// KController to use as a reference
    articulations: Vec<Articulation<T>>,
}

impl<T> KGraph<T> {
    pub fn new() -> Self {
        KGraph {
            articulations: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Articulation<T>) {
        self.articulations.push(node);
    }
}
