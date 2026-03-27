
    use super::*;
    // Assuming these are accessible for creating test DataSets
    use crate::{DataSet, Marker}; 

    // Mock Marker for testing
    #[derive(Clone, Copy, Debug, PartialEq, Default)]
    pub struct TestMarker;
    impl Marker for TestMarker {
        const DOF_OFFSET: usize = 1; 
    }

    // Helper to wrap data into the DataSet
    fn create_test_ds(data: Vec<f64>) -> DataSet<f64, TestMarker> {
        DataSet::new(data)
    }

    #[test]
    fn test_sharpe_ratio_basic() {
        // Returns: [10%, 12%, 8%, 10%] 
        // Mean = 0.10, Sample StdDev ≈ 0.01632993
        let ds = create_test_ds(vec![0.10, 0.12, 0.08, 0.10]);
        let rf = 0.02;
        
        let result = ds.sharpe_ratio(rf).expect("Calculation failed");
        
        // Expected: (0.10 - 0.02) / 0.01632993 ≈ 4.898979
        // Using your internal n_assert_eq for consistency
        n_assert_eq!(*result, 4.8989794855);
    }

    #[test]
    fn test_downside_deviation_logic() {
        // MAR = 0.03
        // Data: [0.05, 0.02, 0.04, 0.01]
        // Diffs: [0, -0.01, 0, -0.02]
        // Squares: [0, 0.0001, 0, 0.0004] -> Sum = 0.0005
        // Mean Sq = 0.0005 / 4 = 0.000125
        // Root = 0.0111803
        let ds = create_test_ds(vec![0.05, 0.02, 0.04, 0.01]);
        let mar = 0.03;
        
        let d_dev = ds.downside_deviation(mar).unwrap();
        
        n_assert_eq!(n_from_f64!(d_dev), 0.0111803398);
    }

    #[test]
    fn test_beta_and_treynor() {
        // Portfolio perfectly correlated but 2x volatile
        let asset = create_test_ds(vec![0.10, 0.20, 0.30]);
        let market = create_test_ds(vec![0.05, 0.10, 0.15]);
        let rf = 0.05;

        // Beta should be 2.0
        let b = asset.beta(&market).unwrap();
        n_assert_eq!(n_from_f64!(b), 2.0);

        // Treynor = (Mean Asset - RF) / Beta
        // (0.20 - 0.05) / 2.0 = 0.075
        let treynor = asset.treynor_ratio(&market, rf).unwrap();
        n_assert_eq!(n_from_f64!(treynor), 0.075);
    }

    #[test]
    fn test_error_on_invalid_rf() {
        let ds = create_test_ds(vec![0.1, 0.2]);
        let result = ds.sharpe_ratio(f64::INFINITY);
        
        match result {
            Err(StatsError::InvalidRiskFreeRate { .. }) => (),
            _ => panic!("Should have returned InvalidRiskFreeRate error"),
        }
    }

    #[test]
    fn test_zero_variance_handling() {
        // Flat line data has 0 std dev / 0 beta
        let ds = create_test_ds(vec![0.1, 0.1, 0.1]);
        let market = create_test_ds(vec![0.05, 0.1, 0.15]);

        assert!(ds.sharpe_ratio(0.02).is_err());
        // Treynor fails because beta is 0
        assert!(ds.treynor_ratio(&market, 0.02).is_err());
    }

    #[test]
    fn test_display_implementation() {
        let result = SharpeResult { value: 2.5 };
        let output = format!("{}", result);
        
        assert!(output.contains("result: 2.5"));
        assert!(output.contains("Very good")); // Part of your grading thresholds
    }
