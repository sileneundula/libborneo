pub struct BorneoContainerSystem {
    containers: Vec<Box<dyn BorneoContainer>>,
}

pub mod metadata;
pub mod pivot;