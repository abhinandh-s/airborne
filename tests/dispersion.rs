use airborne::stats::Dispersion;

#[test]
fn name() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let _v = data.variance().unwrap();
    //    assert_eq!(v, N!(8.25));
}
