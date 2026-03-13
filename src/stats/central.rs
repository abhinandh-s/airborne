use crate::compute::{ComputeFloat, N, to_n_vec};
use crate::error::Result;
use crate::{DataSet, Marker, Numeric, n_from_usize, n_sum};

pub trait CentralTendency {
    // ref: https://en.wikipedia.org/wiki/Arithmetic_mean
    fn mean(&self) -> Result<f64>;
}

impl<T: Numeric, M: Marker> CentralTendency for DataSet<T, M> {
    fn mean(&self) -> Result<f64> {
        self.mean_n().map(|mu| mu.cf_to_f64())
    }
}

impl<T: Numeric, M: Marker> DataSet<T, M> {
    pub(crate) fn mean_n(&self) -> Result<N> {
        let v = to_n_vec(&self.data)?;
        let count = n_from_usize!(v.len());
        let sum = n_sum!(v.into_iter());
        Ok(sum / count)
    }
}
