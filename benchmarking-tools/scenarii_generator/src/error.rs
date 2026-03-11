use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerationError {
    #[error(
        "Weight configuration \"{0}\" assumes {1} client types and cannot be applied to scenario with {2} client types"
    )]
    InvalidWeights(String, usize, usize),
}
