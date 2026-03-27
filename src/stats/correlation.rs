use crate::Numeric;
use crate::compute::{ComputeFloat, N};
use crate::{DataSet, Marker, Result, mean_n};

use super::std_dev_n;

pub trait Correlation<T: Numeric, M: Marker> {
    fn covariance(&self, other: &DataSet<T, M>) -> Result<f64>;
    fn pearson(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn spearman_r(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn autocorrelation(&self, lag: usize) -> Result<f64>;
    //   fn r_squared(&self, other: &DataSet<T, M>) -> Result<f64>;
}

impl<T: Numeric, M: Marker> Correlation<T, M> for DataSet<T, M> {
    fn covariance(&self, other: &DataSet<T, M>) -> Result<f64> {
        let dof = self.dof_denominator_n()?;
        let xs = self.to_n_vec()?;
        let ys = other.to_n_vec()?;
        Ok(covariance_n(&xs, &ys, dof).cf_to_f64())
    }

    fn pearson(&self, other: &DataSet<T, M>) -> Result<f64> {
        let dof = self.dof_denominator_n()?;
        let xs = self.to_n_vec()?;
        let ys = other.to_n_vec()?;

        let cov = covariance_n(&xs, &ys, dof);
        let sx = std_dev_n(&xs, dof);
        let sy = std_dev_n(&ys, dof);
        let res = cov / (sx * sy);
        Ok(res.cf_to_f64())
    }
}

pub(crate) fn covariance_n(xs: &[N], ys: &[N], dof: N) -> N {
    let mu_x = mean_n(xs);
    let mu_y = mean_n(ys);
    let sum = N::cf_sum(xs.iter().zip(ys).map(|(&x, &y)| (x - mu_x) * (y - mu_y)));
    sum / dof
}
