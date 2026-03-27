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
