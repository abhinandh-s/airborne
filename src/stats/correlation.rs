use crate::compute::{ComputeFloat, N, to_n_vec};
use crate::{DataSet, Marker, Numeric, Result, StatsError};

pub trait Correlation<T: Numeric, M: Marker> {
    fn covariance(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn pearson_r(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn spearman_r(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn autocorrelation(&self, lag: usize) -> Result<f64>;
    //   fn r_squared(&self, other: &DataSet<T, M>) -> Result<f64>;
}

impl<T: Numeric, M: Marker> Correlation<T, M> for DataSet<T, M> {
    fn covariance(&self, other: &DataSet<T, M>) -> Result<f64> {
        self.covariance_n(other).map(|cv| cv.cf_to_f64())
    }
}

impl<T: Numeric, M: Marker> DataSet<T, M> {
    // ref: https://statisticsbyjim.com/basics/covariance/
    // required in beta
    pub(crate) fn covariance_n(&self, other: &DataSet<T, M>) -> Result<N> {
        let n = self.len();
        if other.len() != n {
            return Err(StatsError::LengthMismatch {
                len_a: n,
                len_b: other.len(),
            });
        }
        let x = to_n_vec(&self.data)?;
        let y = to_n_vec(&other.data)?;

        let mu_x = self.mean_n()?;
        let mu_y = other.mean_n()?;
        let total = x
            .iter()
            .zip(y)
            .map(|(xi, yi)| (xi - mu_x) * (yi - mu_y))
            .sum::<N>();
        let denom = self.dof_denominator()?;
        Ok(total / n_from_usize!(denom))
    }
}
