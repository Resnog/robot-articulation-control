use crate::articulation::Articulation;

struct KGraph<T> {
    /// The theorerical state of the articulations is stored for the
    /// KController to use as a reference
    articulations: Vec<Articulation<T>>,
}
