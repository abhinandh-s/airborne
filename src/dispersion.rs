use crate::DataSet;
use crate::error::Result;
use crate::marker::Marker;
use crate::numeric::{Numeric, to_f64_vec};

pub trait Dispersion {
    fn mean(&self) -> Result<f64>;
    fn variance(&self) -> Result<f64>;
   // fn covariance(&self, other: &[f64]) -> Result<f64>;
}

impl<T: Numeric, M: Marker> Dispersion for DataSet<T, M> {
    fn variance(&self) -> Result<f64> {
        let v = to_f64_vec(&self.data)?;
        let n = v.len() as f64;
        let denom = self.dof_denominator()? as f64;
        let mean = v.iter().sum::<f64>() / n;
        let ss = v.iter().map(|&x| (x - mean).powi(2)).sum::<f64>();
        Ok(ss / denom)
    }

    fn mean(&self) -> Result<f64> {
        let count = self.data.len() as f64;
        Ok(to_f64_vec(&self.data)?.iter().sum::<f64>() / count)
    }
}
