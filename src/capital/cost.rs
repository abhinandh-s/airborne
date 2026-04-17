use crate::compute::ComputeFloat;
use crate::compute::N;
use crate::types::Percentage;

// TODO: write docs
pub fn cost_of_credit(d: Percentage, dd: u32, ad: u32) -> f64 {
    let d = d.as_decimal();
    let t = (ad - dd) as f64;
    let lhs = nf64!(d) / (nf64!(100.0) - nf64!(d));
    let rhs = nf64!( 365.0) / nf64!(t);
    (lhs * rhs).cf_to_f64()
} 

#[derive(Debug, Default)]
pub struct DebtCapital {
    pub amount: f64,
    pub market_price: Option<f64>,
    /// interest a.k.a coupon
    pub interest: Percentage,
    pub floatation_cost: Option<f64>,
    pub issued_at: IssuedAt,
    pub type_is: Type,
    pub tax_rate: Option<Percentage>,
}

impl DebtCapital {
    pub fn cost_before_tax(&self) -> f64 {
        let int = self.amount * self.interest.as_decimal();
        let denom = match self.market_price {
            Some(mv) => mv,
            None => self.net_proceeds(),
        };
        (int / denom) * 100.0
    }

    pub fn cost_after_tax(&self) -> f64 {
        let rate = match self.tax_rate {
            Some(r) => r.as_decimal(),
            None => 0.0,
        };

        (1.0 - rate) * self.cost_before_tax()
    }

    pub fn net_proceeds(&self) -> f64 {
        // NP = face value - floatation cost -/+ Discount/Premium
        let mut face_value = self.amount;
        if let Some(l) = self.floatation_cost {
            face_value -= l
        }

        // - Discount allowed at the time of issue (if any)
        // + Premium charged at the time of issue (if any)
        match self.issued_at {
            IssuedAt::Par => face_value,
            IssuedAt::Premium(p) => face_value + (p.as_decimal() * self.amount),
            IssuedAt::Discount(d) => face_value - (d.as_decimal() * self.amount),
        }
    }

    pub fn set_issued_at(&mut self, issued_at: IssuedAt) {
        self.issued_at = issued_at;
    }

    pub fn set_typ(&mut self, typ: Type) {
        self.type_is = typ;
    }

    pub fn set_tax_rate(&mut self, tax_rate: Option<Percentage>) {
        self.tax_rate = tax_rate;
    }

    pub fn set_floatation_k(&mut self, floatation_k: Option<f64>) {
        self.floatation_cost = floatation_k;
    }

    pub fn set_interest(&mut self, interest: Percentage) {
        self.interest = interest;
    }
}

#[derive(Debug, Default)]
pub enum Type {
    #[default]
    // We can't issue Irredeemable perf. share in India. Since this enum is suppose to be
    // used for PrefCapital Redeemable is set to default.
    Redeemable,
    /// Irredeemable or Perpetual
    Irredeemable,
}

/// stores Premium/Discount percentage
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum IssuedAt {
    #[default]
    Par,
    Premium(Percentage),
    Discount(Percentage),
}

#[cfg(test)]
mod test {
    use super::*;
    /// Illustration 01 - form official CMA study material sylubus 2022 (icmai)
    #[test]
    fn kd_t_01() {
        let mut debt = DebtCapital {
            interest: Percentage::new_unchecked(10),
            amount: 100_000.0,
            type_is: Type::Irredeemable,
            tax_rate: Some(Percentage::new_unchecked(35)),
            ..Default::default()
        };

        let cost = debt.cost_before_tax();
        assert_eq!(debt.net_proceeds(), 100000.0);
        assert_eq!(cost, 10.00);
        assert_eq!(debt.cost_after_tax(), 6.5);

        debt.set_issued_at(IssuedAt::Discount(10.0.into()));
        let cost = debt.cost_before_tax();
        assert_eq!(debt.net_proceeds(), 90000.0);
        assert_eq!(cost, 11.11111111111111);
        assert_eq!(debt.cost_after_tax(), 7.222222222222222);

        debt.set_issued_at(IssuedAt::Premium(10.0.into()));
        let cost = debt.cost_before_tax();
        assert_eq!(debt.net_proceeds(), 110000.0);
        assert_eq!(cost, 9.090909090909092);
        assert_eq!(debt.cost_after_tax(), 5.90909090909091);
    }

    /// Illustration 02 - form official CMA study material sylubus 2022 (icmai)
    #[test]
    fn kd_t_02() {
        let debt = DebtCapital {
            interest: Percentage::new_unchecked(12),
            amount: 100.0,
            market_price: Some(95.0),
            type_is: Type::Irredeemable,
            tax_rate: Some(Percentage::new_unchecked(35)),
            issued_at: IssuedAt::Premium(Percentage::new_unchecked(5)),
            ..Default::default()
        };

        assert_eq!(debt.cost_after_tax(), 8.210526315789474);
    }

    /// Illustration 03 - form official CMA study material sylubus 2022 (icmai)
    #[test]
    fn kd_t_03() {
        let mut debt = DebtCapital {
            interest: Percentage::new_unchecked(15),
            amount: 100.0,
            type_is: Type::Irredeemable,
            tax_rate: Some(Percentage::new_unchecked(35)),
            ..Default::default()
        };

        assert_eq!(debt.cost_after_tax(), 9.75);

        debt.set_issued_at(IssuedAt::Discount(5.0.into()));
        assert_eq!(debt.net_proceeds(), 95.0);
        assert_eq!(debt.cost_after_tax(), 10.263157894736842);

        debt.set_issued_at(IssuedAt::Premium(5.0.into()));
        assert_eq!(debt.net_proceeds(), 105.0);
        // In textbook its 9.25 (wrong in textbook)
        assert_eq!(debt.cost_after_tax(), 9.285714285714285);
    }
}
