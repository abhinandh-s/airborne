#![allow(dead_code)]

trait NonDiscountedCashFlowTechniques: IntoIterator<Item = f64> {
    fn payback_period(initial_investment: impl Into<f64>, annual_cf: impl Into<f64>) -> f64 {
        initial_investment.into() / annual_cf.into()
    }
    // fn payback_recipocal() -> f64;
    // fn payback_profitability() -> f64;
    // // a.k.a Accounting rate of return
    // fn avarage_rate_of_return() -> f64;
}

trait DiscountedCashFlowTechniques {
    fn npv() -> f64;
    fn pi() -> f64;
    fn irr() -> f64;
    fn dpp() -> f64;
    fn mnpv() -> f64;
    fn mirr() -> f64;
    fn adj_pv() -> f64;
}

// impl NonDiscountedCashFlowTechniques for T {}

// #[cfg(test)]
// mod test {
//     use super::NonDiscountedCashFlowTechniques;
//
//     #[test]
//     fn non_discounted_cash_flow_techniques_t() {
//         let cfs = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
//         let initial_investment = 1000;
//         Int
//     }
// }
