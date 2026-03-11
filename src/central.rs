use crate::compute::{ComputeFloat, to_n_vec};
use crate::error::Result;
use crate::{DataSet, Marker, Numeric, n_from_usize, n_sum};

pub trait CentralTendency {
    // ref: https://en.wikipedia.org/wiki/Arithmetic_mean
    fn mean(&self) -> Result<f64>;
}

impl<T: Numeric, M: Marker> CentralTendency for DataSet<T, M> {
    fn mean(&self) -> Result<f64> {
        let v = to_n_vec(&self.data)?;
        let count = n_from_usize!(self.dof_denominator()?);
        let sum = n_sum!(v.into_iter());
        Ok((sum / count).cf_to_f64())
    }
}
