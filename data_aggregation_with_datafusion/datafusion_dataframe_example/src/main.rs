use datafusion::error::Result as DataFusionResult;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> DataFusionResult<()> {
    let mut ctx = ExecutionContext::new();

    let df = ctx
        .read_csv("../../data/StudentACTResults.csv", CsvReadOptions::new())?
        .aggregate(vec![col("group")], vec![avg(col("english"))])?;

    let results = df.collect().await?;

    println!("{:?}", results);

    Ok(())
}
