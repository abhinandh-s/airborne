use std::iter::Sum;

use num_traits::{FromPrimitive, One, Zero};

use crate::error::Result;
use crate::numeric::{NumExt, NumOps};
use crate::stats::{mean, std_dev};
use crate::{Marker, StatsError};

/// # RiskMetrics
///
/// Provides methods related to portfolio risk for [`DataSet`].
/// eg: sharpe ratio, sortino ratio, treynor ratio, etc,.
pub trait RiskMetrics<T, M: Marker> {
    type Output;
    /// # Sharpe ratio
    ///
    /// ## Formula:
    ///
    /// sharpe = (mean risk - risk free return) / std deviation of mean risk
    ///
    /// ## Usage
    ///
    /// .sharpe_ratio(risk_free);
    ///
    /// where:
    ///     `risk_free: f64` - risk free return
    ///
    /// ## Grading Thresholds
    ///
    /// ```text
    ///     Less than 1: Bad
    ///     1 – 1.99: Adequate/good
    ///     2 – 2.99: Very good
    ///     Greater than 3: Excellent
    /// ```
    //
    // ref: [Sharpe Ratio](https://corporatefinanceinstitute.com/resources/career-map/sell-side/risk-management/sharpe-ratio-definition-formula/)
    fn sharpe_ratio(&self, risk_free: f64) -> Result<f64>;
    /// The Sortino ratio measures the risk-adjusted return of an investment asset, portfolio, or strategy. It is a modification of the Sharpe ratio but penalizes only those returns falling below a user-specified target or required rate of return, while the Sharpe ratio penalizes both upside and downside volatility equally. Though both ratios measure an investment's risk-adjusted return, they do so in significantly different ways that will frequently lead to differing conclusions as to the true nature of the investment's return-generating efficiency.
    ///
    /// The Sortino ratio is used as a way to compare the risk-adjusted performance of programs with differing risk and return profiles. In general, risk-adjusted returns seek to normalize the risk across programs and then see which has the higher return unit per risk.
    fn sortino_ratio(&self, rf: T) -> Result<Self::Output>;
    /// .
    fn downside_deviation(&self, mar: T) -> Result<Self::Output>;
    /// .
    fn beta(&self, benchmark: &[T]) -> Result<Self::Output>;
    /// .
    fn treynor_ratio(&self, benchmark: &[T], risk_free: T) -> Result<Self::Output>;
    /// .
    fn tracking_error(&self, benchmark: &[T]) -> Result<Self::Output>;
}

// impl<T: Numeric, M: Marker> RiskMetrics<T, M> for DataSet<T, M> {
pub fn sharpe_ratio<T, M>(data: &[T], risk_free: T) -> Result<T>
where
    M: Marker,
    T: NumOps + NumExt + Zero + One + FromPrimitive + Sum,
{
    // if !risk_free.is_finite() {
    //     return Err(crate::error::StatsError::InvalidRiskFreeRate { rate: risk_free });
    // }

    let sd = std_dev::<T, M>(data)?;
    if sd == T::zero() {
        return Err(StatsError::ZeroVariance);
    }

    // computation only via type N
    let portfolio_ret = mean(data)?;
    let sharpe = (portfolio_ret - risk_free) / sd;

    Ok(sharpe)
}
//
//     fn sortino_ratio(&self, rf: f64) -> Result<f64> {
//         let d_dev = self.downside_deviation(rf)?;
//         if d_dev == 0.0 {
//             return Ok(0.0);
//         }
//
//         let m = N::cf_from_f64(self.mean()?);
//         let d_dev = N::cf_from_f64(d_dev);
//         let rf = N::cf_from_f64(rf);
//
//         let sortino = (m - rf) / d_dev;
//
//         Ok(sortino.cf_to_f64())
//     }
//
//     // number of observation / number of year are same as the x.len()
//     fn downside_deviation(&self, mar: f64) -> Result<f64> {
//         let v = to_n_vec(&self.data)?;
//         let n = self.len_n();
//         let mar = N::cf_from_f64(mar);
//
//         let result = v
//             .iter()
//             .map(|xi| {
//                 let diff = xi - mar;
//                 if diff.is_sign_negative() {
//                     diff * diff
//                 } else {
//                     N::cf_from_f64(0.0)
//                 }
//             })
//             .sum::<N>();
//
//         let re = (result / n).cf_sqrt();
//
//         Ok(re.cf_to_f64())
//     }
//
//     fn treynor_ratio(&self, benchmark: &DataSet<T, M>, risk_free: f64) -> Result<f64> {
//         if !risk_free.is_finite() {
//             return Err(StatsError::InvalidRiskFreeRate { rate: risk_free });
//         }
//
//         let b = self.beta_n(benchmark)?;
//         if b == N::cf_zero() {
//             return Err(StatsError::ZeroVariance);
//         }
//
//         let n = self.to_n_vec()?;
//         let mu = mean(&n)?;
//         let rf = N::cf_from_f64(risk_free);
//         let t = (mu - rf) / b;
//
//         Ok(t.cf_to_f64())
//     }
//
//     fn beta(&self, benchmark: &DataSet<T, M>) -> Result<f64> {
//         self.beta_n(benchmark).map(ComputeFloat::cf_to_f64)
//     }
//
//     fn tracking_error(&self, benchmark: &DataSet<T, M>) -> Result<f64> {
//         if self.len() != benchmark.len() || self.is_empty() {
//             return Err(StatsError::InvalidBenchmark);
//         }
//
//         let active = self
//             .to_n_iter()
//             .zip(benchmark.to_n_iter())
//             .map(|(p, b)| p - b)
//             .collect::<Vec<N>>();
//
//         let active_ds: DataSet<N> = DataSet::from_iter(active);
//
//         std_dev_n(&active_ds, active_ds.dof_denominator_n()?).map(|(_, _, sd)| sd.cf_to_f64())
//     }
// }
//
// impl<T: Numeric, M: Marker> DataSet<T, M> {
//     fn beta_n(&self, benchmark: &DataSet<T, M>) -> Result<N> {
//         let dof = self.dof_denominator_n()?;
//         let self_n = self.to_n_vec()?;
//         let benchmark_n = benchmark.to_n_vec()?;
//         if self.len() != benchmark.len() {
//             return Err(StatsError::InvalidBenchmark);
//         }
//         // Fully reuses existing Correlation + Dispersion trait impls.
//         // M::DOF_OFFSET propagates through both cov and var and cancels.
//         let cov = covariance(&self_n, &benchmark_n, dof)?;
//         let (_, var_b) = variance_n(&benchmark_n, dof)?;
//         if var_b == N::cf_zero() {
//             return Err(StatsError::ZeroVariance);
//         }
//         Ok(cov / var_b)
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use crate::DataSet;
//     use crate::error::Result;
//
//     use super::RiskMetrics;
//
//     // in the month of Jan 2026
//     const _NIFTY_50: [f64; 19] = [
//         0.006960765170238605,
//         -0.002972058760474052,
//         -0.002727647317136396,
//         -0.0014496220164682432,
//         -0.01009536415844993,
//         -0.007479613285493556,
//         0.004164153963733427,
//         -0.0022469428853927357,
//         -0.0025921184600641006,
//         0.0011201764399651254,
//         -0.0042363247573810724,
//         -0.013796877137441129,
//         -0.002972357079163777,
//         0.0052628596094604,
//         -0.009539381186706124,
//         0.005060152863462813,
//         0.006647346488174181,
//         0.003004819548983437,
//         -0.003865234077404724,
//     ];
//     // in the month of Jan 2026
//     const ITC: [f64; 19] = [
//         -0.03792780822846855,
//         -0.0009997348459663343,
//         -0.02073202469727801,
//         -0.0035042128799998833,
//         -0.001025698054907989,
//         -0.011000239733699091,
//         0.003707530300019869,
//         -0.0109337303722129,
//         0.000149316018136497,
//         -0.016579515057863883,
//         0.012150627231730476,
//         -0.02070828665292772,
//         -0.00475024862225316,
//         0.00030797372763436743,
//         -0.004463668706180607,
//         -0.014687699127850123,
//         0.007845658409425468,
//         -0.007940199219514013,
//         0.011142447553835816,
//     ];
//
//     // #[test]
//     // fn beta_t() {
//     // let series: DataSet<f64> = DataSet::new(ITC.to_vec()).unwrap();
//     // let market: DataSet<f64> = DataSet::new(NIFTY_50.to_vec()).unwrap();
//     //     let beta: f64 = Beta::new(series, market).into();
//     //     assert_eq!(beta, -0.13098715705340794);
//     // }
//     macro_rules! assertion {
//         ($a:expr, $b:expr, $c:expr) => {{
//             assert!(matches!($a, $b | $c));
//         }};
//     }
//
//     #[test]
//     fn sharpe_t() -> Result<()> {
//         let rf = 0.03; // risk free return = 3%
//         let series: DataSet<f64> = DataSet::from_iter(ITC.to_vec());
//         let s1 = series.sharpe_ratio(rf)?;
//         assertion!(
//             s1,
//             -3.024907069875915,  // f64
//             -3.0249070698759137  // rust_decimal::Decimal
//         );
//
//         Ok(())
//     }
//
//     // ref: https://www.investopedia.com/terms/d/downside-deviation.asp
//     // #[test]
//     // fn downside_t() {
//     //     // downside deviation input data
//     //     let _years = [2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018, 2019];
//     //
//     //     let returns = [-0.02, 0.16, 0.31, 0.17, -0.11, 0.21, 0.26, -0.03, 0.38];
//     //     let mar = 0.01;
//     //
//     //     let dd = downside_deviation(&returns, mar);
//     //    // TODO: this test
//     //     // assert_eq!(dd.round(), 0.0433)
//     // }
// }
//
// use crate::{DataSet, Marker, Numeric, covariance, mean, sd, variance};
//
// pub struct Beta(f64);
//
// impl Beta {
//     pub fn new<T: Numeric, M: Marker>(series: DataSet<T, M>, market: DataSet<T, M>) -> Self {
//         // If they don't match, the math is technically invalid for a specific timeframe
//         assert_eq!(
//             series.len(),
//             market.len(),
//             "series[{}] and market[{}] must have the same mumber of data points",
//             series.len(),
//             market.len()
//         );
//
//         if series.is_empty() {
//             return Self(0.0);
//         }
//         let s = to_f64_vec(&series.data).unwrap();
//         let m = to_f64_vec(&market.data).unwrap();
//         let beta = crate::covariance!(&s, &m) / crate::variance!(&m);
//         Self(beta)
//     }
//
//     // if Beta > 1.0 => The fund is more volatile than the market
//     /// If true, it means the fund moves exactly with the market.
//     pub const fn is_one(&self) -> bool {
//         // self.0 == 1.00 - this is risky due to precision errors
//         // check if the difference is within a tiny margin (epsilon)
//         (self.0 - 1.0).abs() < 1e-6 // or f64::EPSILON? isn't EPSILON way too small for
//         // financial calculations
//     }
//
//     /// The fund is "defensive" and moves less than the market
//     pub const fn is_negative(&self) -> bool {
//         self.0.is_sign_negative()
//     }
//
//     pub const fn is_positive(&self) -> bool {
//         self.0.is_sign_positive()
//     }
//
//     pub const fn value(&self) -> f64 {
//         self.0
//     }
// }
//
// impl From<f64> for Beta {
//     fn from(value: f64) -> Self {
//         Self(value)
//     }
// }
//
// impl From<Beta> for f64 {
//     fn from(value: Beta) -> Self {
//         value.0
//     }
// }
//

//
// /// # Sharpe ratio
// ///
// /// ## Formula:
// ///
// /// sharpe = (mean risk - risk free return) / std deviation of mean risk
// ///
// /// ## Usage
// ///
// /// 1. sharpe!(series, rf);
// /// 2. sharpe!(rp, rf, sd);
// ///
// /// where:
// ///     `series: &[f64]` - portfolio return as slice Item = % return (not absolute return)
// ///     `rf: f64` - risk free return
// ///     `rp: f64` - portfolio return
// ///     `sd: f64` - standard deviation
// ///
// /// ## Grading Thresholds
// /// ```text
// ///     Less than 1: Bad
// ///     1 – 1.99: Adequate/good
// ///     2 – 2.99: Very good
// ///     Greater than 3: Excellent
// /// ```
// /// ref: [Sharpe Ratio](https://corporatefinanceinstitute.com/resources/career-map/sell-side/risk-management/sharpe-ratio-definition-formula/)
// #[macro_export]
// macro_rules! sharpe {
//     ($series: expr, $rf: expr) => {
//         $crate::risk_metrics::internal_sharpe(Some($series), $rf, None, None)
//     };
//     ($rp: expr, $rf: expr, $sd: expr) => {
//         $crate::risk_metrics::internal_sharpe(None, $rf, Some($rp), Some($sd))
//     };
// }
// pub struct SharpeResult {
//     pub(crate) value: f64,
// }
//
// impl SharpeResult {
//     pub fn value(&self) -> f64 {
//         self.value
//     }
// }

// impl Display for SharpeResult {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "result: {}\n\n## Grading Thresholds\n\nLess than 1: Bad\n1 – 1.99: Adequate/good\n2 – 2.99: Very good\nGreater than 3: Excellent",
//             self.value()
//         )
//     }
// }
//
// impl Deref for SharpeResult {
//     type Target = f64;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
//
// macro_rules! impl_result {
//     ($name:ident, $fmt:literal) => {
//         pub struct $name {
//             pub(crate) value: f64,
//         }
//
//         impl $name {
//             pub fn value(&self) -> f64 {
//                 self.value
//             }
//         }
//
//         impl std::fmt::Display for $name {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 write!(f, "result: {}\n\n\n{}", self.value(), $fmt)
//             }
//         }
//
//         impl std::ops::Deref for $name {
//             type Target = f64;
//
//             fn deref(&self) -> &Self::Target {
//                 &self.value
//             }
//         }
//     };
// }
//
// impl_result!(SortinoResult, "this");
