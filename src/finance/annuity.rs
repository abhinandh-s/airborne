use crate::Result;
use crate::StatsError;
use crate::compute::ComputeFloat;
use crate::compute::N;
use crate::error::check_bound;

fn is_finite(v: f64, index: usize) -> Result<N> {
    if !v.is_finite() {
        return Err(StatsError::InvalidValue { index });
    }
    Ok(nf64!(v))
}

pub fn fv_single(pv: f64, rate: f64, periods: u32) -> Result<f64> {
    check_bound(rate, 0.0, 1.0)?;
    let fv = nf64!(pv) * (N::cf_one() + nf64!(rate)).cf_powf(periods as f64);
    Ok(fv.cf_to_f64())
}

pub fn fv_single_unchecked(pv: f64, rate: f64, periods: u32) -> f64 {
    let fv = nf64!(pv) * (N::cf_one() + nf64!(rate)).cf_powf(periods as f64);
    fv.cf_to_f64()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn fv_single_t() {
        let (p, r, n) = (150_000.0, 0.12, 10_u32);
        let fv_01 = fv_single(p, r, n).unwrap();
        let fv_02 = fv_single_unchecked(p, r, n);
        assert_eq!(fv_01, 465877.2312516318);
        assert_eq!(fv_02, 465877.2312516318);
        fv_single(10.0, -12.8, 10).unwrap();
    }
}

// use crate::{Numeric, compute::N};
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum AnnuityKind {
//     Ordinary,
//     Due,
// }
//
// impl AnnuityKind {
//     #[inline]
//     fn timing_factor(&self, rate: f64) -> f64 {
//         match self {
//             AnnuityKind::Ordinary => 1.0,
//             AnnuityKind::Due => 1.0 + rate,
//         }
//     }
// }
//
// pub struct Annuity {
//     cashflow: Vec<N>,
//     rate: N,
// }
//
// impl Annuity {
//     pub fn new_regular(cashflow: impl IntoIterator<Item = impl Into<f64>>, rate: impl Into<f64>) -> Self {
//         let s = cashflow
//         .into_iter()
//         .enumerate()
//         .map(|(i, v)| {
//             let f = v.into();
//             if !f.is_finite() {
//                 return Err(crate::StatsError::InvalidValue { index: i });
//             }
//             Ok(nf64!(f))
//         })
//         .collect();
//         if cashflow.is_empty() {
//             return Err(crate::StatsError::EmptyIterator);
//         }
//         Self { cashflow, rate }
//     }
// }
