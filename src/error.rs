use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum StatsError {
    #[error("dataset is empty")]
    EmptyDataSet,

    #[error("insufficient data: need at least {needed} elements, got {got}")]
    InsufficientData { needed: usize, got: usize },

    #[error("value at index {index} cannot be represented as f64")]
    ConversionError { index: usize },

    #[error("encountered NaN or infinite value at index {index}")]
    InvalidValue { index: usize },

    // risk 
    #[error("risk-free rate is non-finite: {rate}")]
    InvalidRiskFreeRate { rate: f64 },
}

pub type Result<T> = std::result::Result<T, StatsError>;
