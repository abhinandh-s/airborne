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
    use crate::marker::Population; // Import a concrete marker

    #[test]
    fn test_mean_basic() {
        let data = vec![1.0, 2.0, 3.0];
        // Explicitly type the DataSet to help the compiler
        let ds: DataSet<f64, Population> = DataSet::new(data).unwrap();
        
        let result = ds.mean().unwrap();
        n_assert_eq!(n_from_f64!(result), 2.0);
    }

    #[test]
    fn test_mean_empty() {
        let data: Vec<f64> = vec![];
        // Even for errors, the compiler needs to know what M would have been
        let result = DataSet::<f64, Population>::new(data);
        assert!(result.is_err());
    }
}
