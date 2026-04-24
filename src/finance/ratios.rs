// use chrono::{NaiveDate};
use crate::compute::ComputeFloat;
use crate::compute::N;
// use std::f64::consts::E;
// use statrs::distribution::{Normal, ContinuousCDF};

// adding detailed BSM output with all greeks soon
// adding binomial tree model soon

// Ratios

// Ratios - Liquidity
pub fn current_r(current_assets: impl Into<f64>, current_liabilities: impl Into<f64>) -> f64 {
    let current = nf64!(current_assets.into()) / nf64!(current_liabilities.into());
    current.cf_to_f64()
}

pub fn quick_r(
    current_assets: impl Into<f64>,
    inventory: impl Into<f64>,
    current_liabilities: impl Into<f64>,
) -> f64 {
    ((nf64!(current_assets.into()) - nf64!(inventory.into())) / nf64!(current_liabilities.into()))
        .cf_to_f64()
}

// fn acid_r(cash: impl Into<f64>, inventory: impl Into<f64>, accounts_recievable: impl Into<f64>, current_liabilities: impl Into<f64>) -> f64 {
//     (cash + inventory + accounts_recievable) / current_liabilities
// }
//
// fn cash_r(cash_and_equivalents: impl Into<f64>, current_liabilities: impl Into<f64>) -> f64 {
//     cash_and_equivalents / current_liabilities
// }
//
// // Ratios - Profitability
// fn gross_m(gross_profit: impl Into<f64>, revenue: impl Into<f64>) -> f64 {
//     gross_profit / revenue
// }
//
// fn operating_m(operating_income: impl Into<f64>, revenue: impl Into<f64>) -> f64 {
//     operating_income / revenue
// }
//
// fn net_m(net_income: impl Into<f64>, revenue: impl Into<f64>) -> f64 {
//     net_income / revenue
// }
//
// fn r_o_a(net_income: impl Into<f64>, total_assets: impl Into<f64>) -> f64 {
//     net_income / total_assets
// }
//
// fn r_o_e(net_income: impl Into<f64>, shareholders_equity: impl Into<f64>) -> f64 {
//     net_income / shareholders_equity
// }
//
//
// // Ratios - Leverage
// fn d_t_e(total_debt: impl Into<f64>, shareholders_equity: impl Into<f64>) -> f64 {
//     total_debt / shareholders_equity
// }
//
// fn d_r(total_debt: impl Into<f64>, total_assets: impl Into<f64>) -> f64 {
//     total_debt / total_assets
// }
//
// fn ebit_i_c(ebit: impl Into<f64>, interest_expense: impl Into<f64>) -> f64 {
//     ebit / interest_expense
// }
//
// // Ratios - Activity
// fn inv_t(cost_of_goods_sold: impl Into<f64>, average_inventory: impl Into<f64>) -> f64 {
//     cost_of_goods_sold / average_inventory
// }
//
// fn rec_t(revenue: impl Into<f64>, average_accounts_receivable: impl Into<f64>) -> f64 {
//     revenue / average_accounts_receivable
// }
//
// fn a_t(revenue: impl Into<f64>, total_assets: impl Into<f64>) -> f64 {
//     revenue / total_assets
// }
//
//
// // Ratios - Valuation
// fn p_t_e(share_price: impl Into<f64>, earnings_per_share: impl Into<f64>) -> f64 {
//     share_price / earnings_per_share
// }
//
// fn p_t_b(share_price: impl Into<f64>, book_value_per_share: impl Into<f64>) -> f64 {
//     share_price / book_value_per_share
// }
//
// fn div_y(annual_dividends_per_share: impl Into<f64>, share_price: impl Into<f64>) -> f64 {
//     annual_dividends_per_share / share_price
// }
//
//
// // Build Ups
//
// // Build Up - FCFF
//
// fn fcff_ni(net_income: impl Into<f64>, non_cash_charges: impl Into<f64>, interest: impl Into<f64>, tax_rate: impl Into<f64>, capex: impl Into<f64>, change_in_working_capital: impl Into<f64>,) -> f64 {
//     net_income + non_cash_charges + (interest * (1.0 - tax_rate)) - capex - change_in_working_capital
// }
//
// fn fcff_cfo(cfo: impl Into<f64>, interest_expense: impl Into<f64>, tax_rate: impl Into<f64>, capex: impl Into<f64>) -> f64 {
//     cfo + interest_expense * (1.0 - tax_rate) - capex
// }
//
// fn fcff_ebit(ebit: impl Into<f64>, tax_rate: impl Into<f64>, depreciation: impl Into<f64>, capex: impl Into<f64>, change_in_working_capital: impl Into<f64>) -> f64 {
//     ebit * (1.0 - tax_rate) + depreciation - capex - change_in_working_capital
// }
//
// fn fcff_ebitda(ebitda: impl Into<f64>, tax_rate: impl Into<f64>, depreciation: impl Into<f64>, capex: impl Into<f64>, change_in_working_capital: impl Into<f64>) -> f64 {
//      (ebitda * (1.0 - tax_rate)) + (depreciation * (1.0 - tax_rate)) - capex - change_in_working_capital
// }
//
// // Build Up - WACC
// fn wacc_coe(coe: impl Into<f64>, we: impl Into<f64>, tax_rate: impl Into<f64>, cod: impl Into<f64>, wd: impl Into<f64>, cop: impl Into<f64>, wp: impl Into<f64>) -> f64 {
//     (coe * we) + (cop * wp) + ((1.0 - tax_rate) * cod * wd)
// }
//
// fn coe(rfr: impl Into<f64>, equity_beta: impl Into<f64>, mrp: impl Into<f64>) -> f64 {
//     rfr + (equity_beta * mrp)
// }
//
// fn wacc_beta(equity_beta: impl Into<f64>, rfr: impl Into<f64>, mrp: impl Into<f64>, we: impl Into<f64>, tax_rate: impl Into<f64>, cod: impl Into<f64>, wd: impl Into<f64>, cop: impl Into<f64>, wp: impl Into<f64>) -> f64 {
//     ((rfr + (equity_beta * mrp)) * we) + (cop * wp) + ((1.0 - tax_rate) * cod * wd)
// }
//
// fn mrp(equity_market_return: impl Into<f64>, rfr: impl Into<f64>) -> f64 {
//     equity_market_return - rfr
// }
//
// fn equity_beta(equity: impl Into<f64>, debt: impl Into<f64>, asset_beta: impl Into<f64>, tax_rate: impl Into<f64>) -> f64 {
//     asset_beta * ( 1.0 + ((debt / equity) * ( 1.0 - tax_rate)))
// }
//
// fn asset_beta(equity: impl Into<f64>, debt: impl Into<f64>, equity_beta: impl Into<f64>, tax_rate: impl Into<f64>) -> f64 {
//     equity_beta / ( 1.0 + ((debt / equity) * ( 1.0 - tax_rate)))
// }
//
// // Time Value of Money
//
//
// // Time Value of Money - XNPV
// // assumes that the first outflow is at t=0
// fn xnpv(cashflows: Vec<(f64, &str)>, discount_rate: impl Into<f64>) -> f64 {
//     let mut present_value = 0.0;
//
//     // Parse the date of the first cash flow to use as the start date
//     let start_date = NaiveDate::parse_from_str(cashflows[0].1, "%Y-%m-%d").expect("Invalid date format");
//
//     for (cashflow, date_str) in cashflows {
//         let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("Invalid date format");
//         let days = (date - start_date).num_days() as f64;
//         let discount_factor = (1.0 + discount_rate).powf(days / 365.0);
//         present_value += cashflow / discount_factor;
//     }
//
//     present_value
// }
//
// // Time Value of Money - XIRR
// fn xirr(cashflows: Vec<(f64, &str)>) -> f64 {
//     let mut rate = 0.10; // Initial guess of 10%
//     let tolerance = 1e-6;
//     let max_iterations = 10000;
//
//     for _ in 0..max_iterations {
//         let npv = xnpv(cashflows.clone(), rate);
//         let npv_derivative = (xnpv(cashflows.clone(), rate + tolerance) - npv) / tolerance;
//
//         let new_rate = rate - npv / npv_derivative;
//
//         if (new_rate - rate).abs() < tolerance {
//             return new_rate;
//         }
//
//         rate = new_rate;
//     }
//
//     panic!("IRR did not converge");
// }
//
// // Valuation Models
//
// // Valuation Models - Gordon Growth Model - One Phase
// fn ggm_p1(cashflow_0: impl Into<f64>, required_rate_of_return: impl Into<f64>, growth_rate: impl Into<f64>) -> Option<f64> {
//     if required_rate_of_return <= growth_rate {
//         // Return None if the required rate of return is not greater than the growth rate to avoid division by zero or negative denominator
//         return None;
//     }
//
//     let value = (cashflow_0 * (1.0 + growth_rate)) / (required_rate_of_return - growth_rate);
//     Some(value)
// }
//
// // Valuation Models - Gordon Growth Model - Two Phase
// fn ggm_p2(cashflow_0: impl Into<f64>, required_rate_of_return: impl Into<f64>, growth_rate_1: impl Into<f64>, growth_rate_2: impl Into<f64>, periods: u32) -> Option<f64> {
//     if required_rate_of_return <= growth_rate_2 {
//         // Return None if the required rate of return is not greater than the growth rate to avoid division by zero or negative denominator
//         return None;
//     }
//
//     let mut pv_cashflow = 0.0;
//     for t in 1..=periods {
//         let cashflow_t = cashflow_0 * (1.0 + growth_rate_1).powi(t as i32);
//         pv_cashflow += cashflow_t / (1.0 + required_rate_of_return).powi(t as i32);
//     }
//
//     // Calculate the terminal value at the end of the period
//     let terminal_cashflow = cashflow_0 * (1.0 + growth_rate_1).powi((periods) as i32) * (1.0 + growth_rate_2);
//     let terminal_value = terminal_cashflow / (required_rate_of_return - growth_rate_2);
//     let pv_terminal_value = terminal_value / (1.0 + required_rate_of_return).powi(periods as i32);
//
//     Some(pv_cashflow + pv_terminal_value)
// }
//
//
//
// // Black-Sholes-Merton
//
// // Black-Sholes-Merton - function to calculate N(d1) and N(d2)
// fn calc_nd(d: impl Into<f64>) -> f64 {
//     let normal = Normal::new(0.0, 1.0).unwrap();
//     normal.cdf(d)
// }
//
// // Black-Scholes-Merton - function for call and put values
// fn bsm(
//     s: impl Into<f64>,       // Current stock price
//     k: impl Into<f64>,       // Option strike price
//     t: impl Into<f64>,       // Time to expiration in years
//     r: impl Into<f64>,       // Annual Risk-free interest rate
//     sigma: impl Into<f64>,   // Annual Volatility
//     q: impl Into<f64>
// ) -> (f64, f64, f64, f64) {
//     let d1 = (s.ln() - k.ln() + (r - q + sigma.powi(2) / 2.0) * t) / (sigma * t.sqrt());
//     let d2 = d1 - sigma * t.sqrt();
//
//     let nd1 = calc_nd(d1);
//     let nd2 = calc_nd(d2);
//
//     let call_price = s * E.powf(-q * t) * nd1 - k * E.powf(-r * t) * nd2;
//     let put_price = k * E.powf(-r * t) * calc_nd(-d2) - s * E.powf(-q * t) * calc_nd(-d1);
//
//     (call_price, put_price, nd1, nd2)
// }
//
//
//
//
// // tests
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // ratios
//     #[test]
//     fn test_current_ratio() {
//         assert_eq!(current_r(200.0, 100.0), 2.0);
//     }
//
//     // TMV
//     #[test]
//     fn test_present_value_of_cashflows() {
//         let cashflows = vec![
//             (1000.0, "2026-01-01"),
//             (1500.0, "2027-01-01"),
//             (2000.0, "2028-01-01"),
//         ];
//         let discount_rate = 0.05; // 5% annual discount rate
//
//         let pv = xnpv(cashflows, discount_rate);
//
//         // Expected present value calculated manually or with a reliable tool
//         let expected_pv = 4242.63;
//
//         assert!((pv - expected_pv).abs() < 0.01); // Allowing a small margin for floating-point precision
//     }
//     #[test]
//     fn test_irr() {
//         // Define a set of cash flows (amount, date)
//         let cashflows = vec![
//             (-1000.0, "2025-01-01"),
//             (200.0, "2025-12-31"),
//             (300.0, "2026-12-31"),
//             (400.0, "2027-12-31"),
//             (500.0, "2028-12-31"),
//             (600.0, "2029-12-31"),
//         ];
//
//         // Calculate the IRR
//         let calculated_irr = xirr(cashflows);
//
//         // Expected IRR value (approximately)
//         let expected_irr = 0.23300;
//
//         // Assert that the calculated IRR is close to the expected value
//         assert!((calculated_irr - expected_irr).abs() < 0.001, "IRR calculation is incorrect");
//     }
//
//     // Valuation Models
//     #[test]
//     fn test_ggm_p1_basic() {
//         let result = ggm_p1(100.0, 0.1, 0.05);
//         assert_eq!(result, Some(2100.0));
//     }
//
//     #[test]
//     fn test_ggm_p1_zero_growth() {
//         let result = ggm_p1(100.0, 0.1, 0.0);
//         assert_eq!(result, Some(1000.0));
//     }
//
//     #[test]
//     fn test_ggm_p1_high_growth() {
//         let result = ggm_p1(100.0, 0.05, 0.1);
//         assert_eq!(result, None);
//     }
//
//     #[test]
//     fn test_ggm_p1_negative_growth() {
//         let result = ggm_p1(100.0, 0.1, -0.05);
//         assert_eq!(result, Some(633.3333333333333));
//     }
//
//     #[test]
//     fn test_ggm_p2_basic() {
//         let cashflow_0 = 100.0;
//         let required_rate_of_return = 0.1;
//         let growth_rate_1 = 0.05;
//         let growth_rate_2 = 0.03;
//         let periods = 5;
//         let result = ggm_p2(cashflow_0, required_rate_of_return, growth_rate_1, growth_rate_2, periods);
//         assert!(result.is_some());
//         let expected_value = 1601.8757; // Replace with the expected value calculated manually or using a reliable source
//         assert!((result.unwrap() - expected_value).abs() < 1.0);
//     }
//
//
//     #[test]
//     fn test_ggm_p2_required_rate_not_greater_than_growth_rate_2() {
//         let cashflow_0 = 100.0;
//         let required_rate_of_return = 0.02;
//         let growth_rate_1 = 0.05;
//         let growth_rate_2 = 0.03;
//         let periods = 5;
//         let result = ggm_p2(cashflow_0, required_rate_of_return, growth_rate_1, growth_rate_2, periods);
//         assert!(result.is_none());
//     }
//
//     #[test]
//     fn test_ggm_p2_zero_periods() {
//         let cashflow_0 = 100.0;
//         let required_rate_of_return = 0.1;
//         let growth_rate_1 = 0.05;
//         let growth_rate_2 = 0.03;
//         let periods = 0;
//         let result = ggm_p2(cashflow_0, required_rate_of_return, growth_rate_1, growth_rate_2, periods);
//         assert!(result.is_some());
//         let terminal_value = (cashflow_0 * (1.0 + growth_rate_1).powi((periods) as i32) * (1.0 + growth_rate_2)) / (required_rate_of_return - growth_rate_2);
//         let expected_value = terminal_value / (1.0 + required_rate_of_return).powi(periods as i32);
//         assert!((result.unwrap() - expected_value).abs() < 1.0);
//     }
//
//     // BSM
//     #[test]
//     fn test_black_scholes() {
//         // Define test parameters
//         let s = 100.0;      // Current stock price
//         let k = 100.0;      // Option strike price
//         let t = 1.0;        // Time to expiration in years
//         let r = 0.05;       // Risk-free interest rate
//         let sigma = 0.2;    // Volatility
//         let q = 0.03;       // Dividend Yield
//
//         // Expected results for call and put options
//         // These should be calculated using a reliable source or tool
//         let expected_call_price = 8.653; // Example value for call
//         let expected_put_price = 6.731;   // Example value for put
//         let expected_nd1 = 0.579;         // Example value
//         let expected_nd2 = 0.500;         // Example value
//
//         // Call the function
//         let (call_price, put_price, nd1, nd2) = bsm(s, k, t, r, sigma, q);
//         println!("debug:d1 ={}",nd1);
//         // Assert results with a small tolerance for floating-point comparisons
//         let tolerance = 1e-3;
//         assert!((call_price - expected_call_price).abs() < tolerance, "Call price mismatch");
//         assert!((put_price - expected_put_price).abs() < tolerance, "Put price mismatch");
//         assert!((nd1 - expected_nd1).abs() < tolerance, "N(d1) mismatch");
//         assert!((nd2 - expected_nd2).abs() < tolerance, "N(d2) mismatch");
//     }
// }
