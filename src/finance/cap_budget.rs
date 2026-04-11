pub trait NonDiscountedCashFlowTechniques {
    fn payback_period() -> f64;
    fn payback_recipocal() -> f64;
    fn payback_profitability() -> f64;
    // a.k.a Accounting rate of return
    fn avarage_rate_of_return() -> f64;
}
pub trait DiscountedCashFlowTechniques {
    fn npv() -> f64;
    fn pi() -> f64;
    fn irr() -> f64;
    fn dpp() -> f64;
    fn mnpv() -> f64;
    fn mirr() -> f64;
    fn adj_pv() -> f64;
}
