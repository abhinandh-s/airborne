use thiserror::Error;

use crate::compute::N;

/// All errors that can arise from statistical or financial computations.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum StatsError {
    #[error("iterator is empty")]
    EmptyIterator,

    #[error("insufficient data: need at least {needed} elements, got {got}")]
    InsufficientData { needed: usize, got: usize },

    #[error("dataset length mismatch: {len_a} vs {len_b}")]
    LengthMismatch { len_a: usize, len_b: usize },

    #[error("value at index {index} cannot be represented as f64")]
    ConversionError { index: usize },

    #[error("encountered NaN or infinite value at index {index}")]
    InvalidValue { index: usize },

    #[error("percentile p={0} is out of range [0.0, 100.0]")]
    InvalidPercentile(f64),

    #[error("zero variance: data is constant; z-scores and correlation are undefined")]
    ZeroVariance,

    #[error("no mode exists for this dataset")]
    NoMode,

    #[error("trim_percent {0} must be in [0.0, 50.0)")]
    InvalidTrimPercent(f64),

    #[error(
        "all values must be strictly positive for geometric mean; got non-positive at index {index}"
    )]
    NonPositiveValue { index: usize },

    #[error("all values must be non-zero for harmonic mean; got zero at index {index}")]
    ZeroValue { index: usize },

    #[error("division by zero: weight sum is zero")]
    ZeroWeightSum,

    #[error("lag {lag} is too large for dataset of length {n}")]
    LagTooLarge { lag: usize, n: usize },

    #[error("IRR did not converge after {max_iter} iterations (last residual: {residual:.6e})")]
    IrrNoConvergence { max_iter: usize, residual: f64 },

    #[error("IRR is undefined: all cash flows have the same sign")]
    IrrNoSignChange,

    #[error("discount rate {rate} is invalid; must be > -1.0")]
    InvalidDiscountRate { rate: f64 },

    #[error("confidence level {level} is invalid; must be in (0.0, 1.0)")]
    InvalidConfidenceLevel { level: f64 },

    #[error("downside deviation is zero: no returns fall below the minimum acceptable return")]
    ZeroDownsideDeviation,

    #[error("benchmark returns are required and must match the length of portfolio returns")]
    InvalidBenchmark,

    #[error("risk-free rate is non-finite: {rate}")]
    InvalidRiskFreeRate { rate: f64 },

    #[error("cash flows contain no initial investment (no negative value)")]
    NoCashFlowInvestment,

    #[error("equity curve is flat; max drawdown is undefined")]
    FlatEquityCurve,

    #[error("argument must be in between [{lower_bound}..{upper_bound}]: got {n}")]
    InvalidRange {
        n: f64,
        lower_bound: f64,
        upper_bound: f64,
    },
}

pub type Result<T> = std::result::Result<T, StatsError>;

pub(crate) fn check_empty_set(s: &[N]) -> Result<()> {
    if s.is_empty() {
        return Err(StatsError::EmptyIterator);
    }
    Ok(())
}

// return true: if passes
pub(crate) fn check_bound(n: f64, lower_bound: f64, upper_bound: f64) -> Result<()> {
    if n == lower_bound || n == upper_bound {
        return Ok(());
    }
    let rng = lower_bound..upper_bound;
    if !rng.contains(&n) {
        return Err(crate::StatsError::InvalidRange {
            n,
            lower_bound,
            upper_bound,
        });
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bound_t() {
        check_bound(0.0, 0.0, 1.0).unwrap();
        check_bound(0.5, 0.0, 1.0).unwrap();
        check_bound(1.0, 0.0, 1.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn bound_shoud_fail_t_01() {
        check_bound(-0.2, 0.0, 1.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn bound_shoud_fail_t_02() {
        check_bound(1.1, 0.0, 1.0).unwrap();
    }
}
