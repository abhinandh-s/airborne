use crate::compute::{ComputeFloat, to_n_vec};
use crate::error::Result;
use crate::marker::Marker;
use crate::numeric::Numeric;
use crate::{DataSet, n_from_f64, n_from_usize, n_sum};

pub trait Dispersion {
    fn mean(&self) -> Result<f64>;
    fn variance(&self) -> Result<f64>;
    fn std_dev(&self) -> Result<f64>;
    // fn covariance(&self, other: &[f64]) -> Result<f64>;
}

impl<T: Numeric, M: Marker> Dispersion for DataSet<T, M> {
    fn mean(&self) -> Result<f64> {
        let v = to_n_vec(&self.data)?;
        let count = n_from_usize!(self.dof_denominator()?);
        let sum = n_sum!(v.into_iter());
        Ok((sum / count).cf_to_f64())
    }

    fn variance(&self) -> Result<f64> {
        let v = to_n_vec(&self.data)?;
        let denom = self.dof_denominator()?;
        let mu = n_from_f64!(self.mean()?);
        let ss = n_sum!(v.iter().map(|&x| (x - mu).cf_powi(2)));
        Ok((ss / n_from_usize!(denom)).cf_to_f64())
    }

    fn std_dev(&self) -> Result<f64> {
        let v = n_from_f64!(self.variance()?);
        Ok(v.cf_sqrt().cf_to_f64())
    }
}

#[cfg(test)]
mod test {
    use crate::DataSet;

    use super::Dispersion;

    // https://statisticsbyjim.com/calculators/variance-calculator/
    #[test]
    fn variance_t() {
        let data: DataSet<i32> = DataSet::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]).unwrap();
        let variance = data.variance().unwrap();
        assert_eq!(variance, 8.250000);
    }
}
