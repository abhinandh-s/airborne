
    use super::*;
    // Import the traits and types needed for the macros and logic
    use crate::compute::ComputeFloat;
    use crate::{DataSet, StatsError};
    // Use the existing markers since the Marker trait is sealed
    use crate::marker::Sample; 

    /// Helper to wrap data into the DataSet using Sample marker
    fn create_test_ds(data: Vec<f64>) -> DataSet<f64, Sample> {
        // DataSet::new returns a Result, we unwrap it for tests
        DataSet::new(data).expect("Failed to create test dataset")
    }

    #[test]
    fn test_sharpe_ratio_basic() {
        // Returns: [10%, 12%, 8%, 10%] 
        // Mean = 0.10, Sample StdDev ≈ 0.01632993
        let ds = create_test_ds(vec![0.10, 0.12, 0.08, 0.10]);
        let rf = 0.02;
        
        let result = ds.sharpe_ratio(rf).expect("Calculation failed");
        
        // Deref SharpeResult to get the value for n_assert_eq
        n_assert_eq!(n_from_f64!(*result), 4.8989794855);
    }

    #[test]
    fn test_downside_deviation_logic() {
        let ds = create_test_ds(vec![0.05, 0.02, 0.04, 0.01]);
        let mar = 0.03;
        
        let d_dev = ds.downside_deviation(mar).unwrap();
        
        n_assert_eq!(n_from_f64!(d_dev), 0.0111803398);
    }

    #[test]
    fn test_beta_and_treynor() {
        let asset = create_test_ds(vec![0.10, 0.20, 0.30]);
        let market = create_test_ds(vec![0.05, 0.10, 0.15]);
        let rf = 0.05;

        let b = asset.beta(&market).unwrap();
        n_assert_eq!(n_from_f64!(b), 2.0);

        let treynor = asset.treynor_ratio(&market, rf).unwrap();
        n_assert_eq!(n_from_f64!(treynor), 0.075);
    }

    #[test]
    fn test_error_on_invalid_rf() {
        let ds = create_test_ds(vec![0.1, 0.2]);
        let result = ds.sharpe_ratio(f64::INFINITY);
        
        assert!(matches!(result, Err(StatsError::InvalidRiskFreeRate { .. })));
    }

    #[test]
    fn test_zero_variance_handling() {
        let ds = create_test_ds(vec![0.1, 0.1, 0.1]);
        let market = create_test_ds(vec![0.05, 0.1, 0.15]);

        assert!(ds.sharpe_ratio(0.02).is_err());
        assert!(ds.treynor_ratio(&market, 0.02).is_err());
    }

    #[test]
    fn test_display_implementation() {
        let result = SharpeResult { value: 2.5 };
        let output = format!("{}", result);
        
        assert!(output.contains("result: 2.5"));
        assert!(output.contains("Very good"));
    }
