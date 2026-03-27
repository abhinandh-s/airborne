use crate::compute::{ComputeFloat, N, NExt};
use crate::error::Result;
use crate::{DataSet, Marker, Numeric, StatsError};

pub trait CentralTendency {
    // ref: https://en.wikipedia.org/wiki/Arithmetic_mean
    fn mean(&self) -> Result<f64>;
}

impl<T: Numeric, M: Marker> CentralTendency for DataSet<T, M> {
    fn mean(&self) -> Result<f64> {
        let v = self.to_n_vec()?;
        mean_n(v).map(|x| x.cf_to_f64())
    }
}

impl<T: Numeric, M: Marker> DataSet<T, M> {
    pub(crate) fn mean_n(&self) -> Result<N> {
        let v = self.data.to_n_vec()?;
        mean_n(v)
    }
}

/// # Arithmetic mean for internal usage.
///
/// It take `N` give `N`, all internal operations are on `N`
/// compling with the precision falg
///
/// formula: mean = total / n
pub(crate) fn mean_n<I>(series: I) -> Result<N>
where
    I: IntoIterator<Item = N>,
    I::IntoIter: ExactSizeIterator,
{
    let v = series.into_iter();
    let count = v.len_n();
    let sum = n_sum!(v);
    Ok(sum / count)
}

/// # Arithmetic mean
///
/// **formula**: `(x̅ ) = Σxi/ n`. ie, (sum of values / number of values)
///
/// ## Usage
///
/// ```rust
/// use airborne::mean_f64;
///
/// let data = &[10.0, 20.0, 30.0];
/// let result = mean_f64(data);
///
/// assert_eq!(result, Ok(20.0));
/// ```
pub fn mean_f64<'a, I, T>(series: I) -> Result<f64>
where
    T: Copy + 'a,
    f64: From<T>,
    I: IntoIterator<Item = &'a T>,
    I::IntoIter: ExactSizeIterator,
{
    let iter = series.into_iter();
    let count = iter.len();
    if count == 0 {
        return Err(StatsError::EmptyIterator);
    }
    let sum = iter.map(|x| f64::from(*x)).sum::<f64>();
    Ok(sum / count as f64)
}

#[test]
fn generic_mean_t() {
    let data = &[10.0, 20.0, 30.0];
    let data_3 = &vec![10.0, 20.0, 30.0];
    let result = mean_f64(data);
    let result_3 = mean_f64(data_3);

    assert_eq!(result, Ok(20.0));
    assert_eq!(result_3, Ok(20.0));
}
