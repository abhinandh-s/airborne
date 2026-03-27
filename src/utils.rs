
// /// # Variance
// ///
// /// The average of the squared differences from the Mean.
// ///
// ///  Symbol:  σ² ( sigma^2 )
// ///
// /// 1. calculate mean
// /// 2. Then for each number: subtract the Mean and square the result (the squared difference)
// /// 3. Then calculate the average of those squared differences.
// ///
// /// variance = for each + [ (actual - mean)^2 ] / count
// ///
// /// mean: Option<f64> => if None: function will calculate from series
// ///
// /// ref: https://en.wikipedia.org/wiki/Variance
// pub fn variance(series: &[f64], pre_computed_mean: Option<f64>) -> f64 {
//     let count = series.len() as f64;
//     // 01
//     let mean = match pre_computed_mean {
//         Some(m) => m,
//         None => mean!(series),
//     };
//     series
//         .iter()
//         .map(|x| {
//             // 02
//             let diff = x - mean;
//             diff * diff
//         })
//         .sum::<f64>()
//         / count // 03
// }
//
// /// # Variance Macro
// ///
// /// Calculates the variance of a given slice of `f64`.
// ///
// // ## Usage
// ///
// /// 1. `variance!(x)` - Calculates the mean automatically before computing variance.
// /// 2. `variance!(x, mean)` - Uses pre computed mean
// ///
// /// where,
// ///     x = `&[f64]`
// ///     mean = `f64`
// /// ## Usage
// /// ```rust
// /// use airborne::variance;
// /// use airborne::mean;
// ///
// /// let data = [10.0, 20.0, 30.0];
// /// let result_01 = variance!(&data);
// ///
// /// assert_eq!(result_01, 66.66666666666667);
// ///
// /// let mean = mean!(&data);
// /// let result_02 = variance!(&data, mean);
// ///
// /// assert_eq!(result_02, 66.66666666666667);
// /// ```
// #[macro_export]
// macro_rules! variance {
//     ($series: expr) => {
//         $crate::variance($series, None)
//     };
//     ($series: expr, $pre_computed_mean: expr) => {
//         $crate::variance($series, Some($pre_computed_mean))
//     };
// }
//
//
// /// # Covariance Macro
// ///
// /// Calculates the covariance of a given two slices of `f64`.
// ///
// // ## Usage
// ///
// /// 1. `covariance!(x, y)` - Calculates the mean automatically before computing covariance.
// /// 2. `covariance!(x, y, mean)` - Uses pre computed mean
// ///
// /// where,
// ///     x = `&[f64]`
// ///     y = `&[f64]`
// ///     mean = `(f64, f64)` // (mean_x, mean_y)
// ///
// /// ## Usage
// /// ```rust
// /// use airborne::covariance;
// /// use airborne::mean;
// ///
// /// let x = [3, 5, 2, 7, 4].map(|x| x as f64);
// /// let y = [70, 80, 60, 90, 75].map(|x| x as f64);
// ///
// /// let mean_x = mean!(&x);
// /// assert_eq!(mean_x, 4.2);
// ///
// /// let mean_y = mean!(&y);
// /// assert_eq!(mean_y, 75.0);
// ///
// /// let cv = covariance(&x, &y, Some((mean_x, mean_y)));
// ///
// /// assert_eq!(cv, 17.0);
// ///
// /// let m_01 = covariance!(&x, &y);
// /// let m_02 = covariance!(&x, &y, (mean_x, mean_y));
// /// assert_eq!(m_01, cv);
// /// assert_eq!(m_02, cv);
// /// ```
// #[macro_export]
// macro_rules! covariance {
//     ($x: expr, $y: expr) => {
//         $crate::covariance($x, $y, None)
//     };
//     ($x: expr, $y: expr, $pre_computed_mean: expr) => {
//         $crate::covariance($x, $y, Some($pre_computed_mean))
//     };
// }
//
// /// # Standard Deviation
// ///
// /// The Standard Deviation is a measure of how spread out numbers are.
// ///
// /// Symbol: σ (the greek letter sigma)
// ///
// /// Formula: standard deviation = sqrt of variance
// pub fn standard_deviation(variance: f64) -> f64 {
//     variance.sqrt()
// }
// /// # Standard Deviation Macro
// ///
// /// Calculates the standard deviation of a given slice of `f64`.
// ///
// // ## Usage
// ///
// /// 1. `sd!(x)`
// ///
// /// where,
// ///     x = `&[f64]`
// ///
// /// ## Usage
// /// ```rust
// /// use airborne::variance;
// /// use airborne::mean;
// /// use airborne::sd;
// ///
// /// let series = [4, 34, 18, 12, 2, 26].map(|x| x as f64);
// /// let mean = mean!(&series);
// /// let variance = variance!(&series, mean);
// /// let sd = sd!(variance);
// /// assert_eq!(mean, 16.0);
// /// assert_eq!(variance, 130.66666666666666); // 130.67
// /// assert_eq!(sd, 11.430952132988164); // 11.43
// /// ```
// #[macro_export]
// macro_rules! sd {
//     ($variance: expr) => {
//         $crate::standard_deviation($variance)
//     };
// }
//
// #[cfg(test)]
// mod test {
//     use crate::covariance;
//
//     #[test]
//     fn test_prelude() {
//         let series = [4, 34, 18, 12, 2, 26].map(|x| x as f64);
//         let mean = mean!(&series);
//         let variance = variance!(&series, mean);
//         let sd = sd!(variance);
//
//         assert_eq!(mean, 16.0);
//         assert_eq!(variance, 130.66666666666666); // 130.67
//         assert_eq!(sd, 11.430952132988164); // 11.43
//     }
//
//     #[test]
//     fn covariance_t() {
//         let hours = [3, 5, 2, 7, 4].map(|x| x as f64);
//         let score = [70, 80, 60, 90, 75].map(|x| x as f64);
//
//         let mean_x = mean!(&hours);
//         assert_eq!(mean_x, 4.2);
//
//         let mean_y = mean!(&score);
//         assert_eq!(mean_y, 75.0);
//
//         let cv = covariance(&hours, &score, Some((mean_x, mean_y)));
//
//         assert_eq!(cv, 17.0);
//
//         let m_01 = covariance!(&hours, &score);
//         let m_02 = covariance!(&hours, &score, (mean_x, mean_y));
//         assert_eq!(m_01, cv);
//         assert_eq!(m_02, cv);
//     }
// }
