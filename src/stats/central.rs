// # Avoid double convertion
//
// ```rust,ignore
//
// impl<T: Numeric, M: Marker> CentralTendency for DataSet<T, M> {
//     fn mean(&self) -> Result<f64> {
//         let v = self.to_n_vec()?; <- convertion 01
//         mean_n(v).map(|x| x.cf_to_f64())
//     }
// }
//
// impl<T: Numeric, M: Marker> DataSet<T, M> {
//     pub(crate) fn mean_n(&self) -> Result<N> {
//         let v = self.data.to_n_vec()?; <- convertion 02
//         mean_n(v)
//     }
// }
// ```

use crate::compute::{ComputeFloat, N};
use crate::error::Result;
use crate::{DataSet, Marker, Numeric};

pub trait CentralTendency {
    /// # Arithmetic mean
    ///
    /// **formula**: `(x̅ ) = Σxi/ n`. ie, (sum of values / number of values)
    ///
    /// ## Example
    ///
    /// ```rust
    /// use airborne::DataSet;
    /// use crate::airborne::CentralTendency;
    ///
    /// let data: DataSet<f64> = DataSet::new([10.0, 20.0, 30.0]).unwrap();
    /// let result = data.mean();
    ///
    /// assert_eq!(result, Ok(20.0));
    /// ```
    ///
    /// # Error 
    ///
    /// returns error if value in data set 
    ///     1. is finite
    ///     2. can't be convert to f64
    //
    // ref: https://en.wikipedia.org/wiki/Arithmetic_mean
    fn mean(&self) -> Result<f64>;
}

impl<T: Numeric, M: Marker> CentralTendency for DataSet<T, M> {
    fn mean(&self) -> Result<f64> {
        let v = self.to_n_vec()?; // <- only conversion in the whole call chain
        Ok(mean_n(&v).cf_to_f64())
    }
}

/// # Arithmetic mean for internal usage.
///
/// It take `&[N]` give `N`, all internal operations are on `N`
/// compling with the precision falg
///
/// formula: mean = total / n
///
/// putting `mean_n` as a top level fuction will help avoiding
/// double convertion.
pub(crate) fn mean_n(series: &[N]) -> N {
    let sum = N::cf_sum(series.iter().copied());
    sum / N::cf_from_usize(series.len())
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::ComputeFloat;
    use crate::marker::{Population, Sample};
    use crate::StatsError;

    #[test]
    fn test_mean_f64_population() {
        // Standard arithmetic mean: (1.0 + 2.0 + 3.0) / 3 = 2.0
        let data: DataSet<f64, Population> = DataSet::new(vec![1.0, 2.0, 3.0]).unwrap();
        let result = data.mean().unwrap();
        
        // Using n_assert_eq to respect the precision feature flag tolerance
        n_assert_eq!(n_from_f64!(result), 2.0);
    }

    #[test]
    fn test_mean_integer_sample() {
        // Testing Numeric trait: (10 + 20) / 2 = 15.0
        let data: DataSet<i32, Sample> = DataSet::new(vec![10, 20]).unwrap();
        let result = data.mean().unwrap();
        
        n_assert_eq!(n_from_f64!(result), 15.0);
    }

    #[test]
    fn test_mean_n_internal_precision() {
        // Direct test of the internal function using the N type
        let series = vec![
            n_from_f64!(1.1),
            n_from_f64!(2.2),
            n_from_f64!(3.3),
        ];
        let result = mean_n(&series);
        
        // 6.6 / 3 = 2.2
        n_assert_eq!(result, 2.2);
    }

    #[test]
    fn test_mean_invalid_values() {
        // DataSet::new succeeds, but to_n_vec inside mean() should catch NaN
        let data = vec![f64::NAN, 1.0, 2.0];
        let ds = DataSet::new(data).unwrap();
        
        let result = ds.mean();
        assert!(matches!(result, Err(StatsError::InvalidValue { index: 0 })));
    }

    #[test]
    fn test_mean_single_value() {
        let data = data_set![42.0].unwrap();
        assert_eq!(data.mean().unwrap(), 42.0);
    }

    #[test]
    fn test_mean_large_numbers() {
        // Verification that internal N type handles larger sums
        let data = data_set![1e10, 2e10].unwrap();
        assert_eq!(data.mean().unwrap(), 1.5e10);
    }
}
