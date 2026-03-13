use airborne::{DataSet, RiskMetrics, SharpeResult};

fn main() -> airborne::Result<()> {
    let data = [1, 2, 3, 4, 5];
    let set: DataSet<i32> = DataSet::new(data)?;
    let sharpe: SharpeResult = set.sharpe_ratio(0.03)?;
    println!("=== sharpe ratio ===");
    println!("\n{}", sharpe);
    Ok(())
}
