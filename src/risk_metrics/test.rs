use super::*; // Pulls SharpeResult, RiskMetrics, etc. from ratios.rs
use crate::compute::N;
use crate::marker::Sample;
use crate::{DataSet, StatsError};

/// Helper to wrap data into the DataSet using Sample marker
fn create_test_ds(data: Vec<f64>) -> DataSet<f64, Sample> {
    DataSet::new(data).expect("Failed to create test dataset")
}

#[test]
fn test_sharpe_ratio_basic() {
    let ds = create_test_ds(vec![0.10, 0.12, 0.08, 0.10]);
    let rf = 0.02;

    let result = ds.sharpe_ratio(rf).expect("Calculation failed");

    // Use *result to get the f64 value from SharpeResult via Deref
    assert_n_eq!(N::cf_from_f64(*result), 4.89897948556636);
}

#[test]
fn test_downside_deviation_logic() {
    let ds = create_test_ds(vec![0.05, 0.02, 0.04, 0.01]);
    let mar = 0.03;

    let d_dev = ds.downside_deviation(mar).expect("Downside dev failed");

    // Use the full precision value or the literal calculation
    // sqrt(0.000125) is approx 0.011180339887498949
    assert_n_eq!(N::cf_from_f64(d_dev), 0.011180339887498949);
}

#[test]
fn test_beta_and_treynor() {
    let asset = create_test_ds(vec![0.10, 0.20, 0.30]);
    let market = create_test_ds(vec![0.05, 0.10, 0.15]);
    let rf = 0.05;

    let b = asset.beta(&market).expect("Beta failed");
    assert_n_eq!(N::cf_from_f64(b), 2.0);

    let treynor = asset.treynor_ratio(&market, rf).expect("Treynor failed");
    assert_n_eq!(N::cf_from_f64(treynor), 0.075);
}

#[test]
fn test_error_on_invalid_rf() {
    let ds = create_test_ds(vec![0.1, 0.2]);
    let result = ds.sharpe_ratio(f64::INFINITY);

    assert!(matches!(
        result,
        Err(StatsError::InvalidRiskFreeRate { .. })
    ));
}

#[test]
fn test_display_implementation() {
    let result = SharpeResult { value: 2.5 };
    let output = format!("{}", result);

    assert!(output.contains("result: 2.5"));
    assert!(output.contains("Very good"));
}
