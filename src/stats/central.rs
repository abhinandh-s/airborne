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

use std::iter::Sum;
use std::ops::Div;

use num_traits::Float;
use num_traits::FromPrimitive;

use crate::StatsError;
use crate::compute::{ComputeFloat, N};
use crate::error::Result;
use crate::{DataSet, Marker, Numeric};

pub trait CentralTendency {
    type Output;
    /// # Arithmetic mean
    ///
    /// **formula**: `(x̅ ) = Σxi/ n`. ie, (sum of values / number of values)
    ///
    /// ## Example
    ///
    /// ```rust
    /// use airborne::DataSet;
    /// use airborne::stats::CentralTendency;
    ///
    /// let data: DataSet<f64> = DataSet::try_from([10.0, 20.0, 30.0]).unwrap();
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
    fn mean(&self) -> Result<Self::Output>;
}

impl<T: Numeric, M: Marker> CentralTendency for DataSet<T, M> {
    type Output = f64;

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

/// impl `CentralTendency` for `Vec`, `&[T]`, `[T; N]`, `Box<[T]>`, `Box<[T; N]>`
///
/// if a type impls deref to slice this should work on those too.
///
/// # Example
///
/// ```
///  use airborne::stats::CentralTendency;
///
/// let test_series_01: [f64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| x as f64);
/// let test_series_02: &[f64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| x as f64);
/// let test_series_03 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
/// let test_series_04: Box<[f64; 9]> = Box::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
/// let test_series_05: Box<[f64]> = Box::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
///
/// let res_01 = test_series_01.mean().unwrap();
/// let res_02 = test_series_02.mean().unwrap();
/// let res_03 = test_series_03.mean().unwrap();
/// let res_04 = test_series_04.mean().unwrap();
/// let res_05 = test_series_05.mean().unwrap();
///
/// assert_eq!(res_01, res_02);
/// assert_eq!(res_02, res_03);
/// assert_eq!(res_03, res_04);
/// assert_eq!(res_04, res_05);
/// assert_eq!(res_05, res_01);
/// ```
impl<T> CentralTendency for [T]
where
    T: Div<Output = T> + Copy + Sum + FromPrimitive,
{
    type Output = T;

    fn mean(&self) -> Result<Self::Output> {
        let count = self.len();
        // If the iterator is empty, return `Err` to avoid division by zero
        if count == 0 {
            return Err(StatsError::EmptyIterator);
        }

        let sum: T = self.iter().copied().sum();
        // safety: I checked its not `0` & its of same type.
        // so, this must not panic!
        Ok(sum / T::from_usize(count).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::marker::Population; // Import a concrete marker

    #[test]
    fn test_mean_basic() {
        let data = vec![1.0, 2.0, 3.0];
        // Explicitly type the DataSet to help the compiler
        let ds: DataSet<f64, Population> = DataSet::from_iter(data);

        let result = ds.mean().unwrap();
        assert_n_eq!(N::cf_from_f64(result), 2.0);
    }

    #[test]
    fn test_mean_empty() -> Result<()> {
        let test_series_01: [N; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| nusize!(x));
        let res_01 = test_series_01.mean().unwrap();
        assert_eq!(res_01, nf64!(5.0));
        Ok(())
    }
}
