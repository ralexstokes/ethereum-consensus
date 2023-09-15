use crate::bellatrix::execution_engine as bellatrix;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid block hash")]
    InvalidBlockHash,
    #[error("invalid payload")]
    InvalidPayload,
}

#[derive(Default)]
pub struct ExecutionEngine {
    bellatrix: Option<Box<dyn bellatrix::ExecutionEngine>>,
}
