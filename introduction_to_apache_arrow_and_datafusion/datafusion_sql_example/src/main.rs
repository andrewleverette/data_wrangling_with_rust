use datafusion::error::Result as DataFusionResult;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> DataFusionResult<()> {
    let mut ctx = ExecutionContext::new();

    // Register CSV in execution context
    ctx.register_csv("results", "../../data/StudentACTResults.csv", CsvReadOptions::new())?;

    // Returns a data frame that contains a logical plan
    // that is parsed from SQL syntax
    let df = ctx.sql("SELECT * FROM results WHERE group = 1")?;

    // Execute query and return results
    let results = df.collect().await?;

    println!("{:?}", results);

    Ok(())
}
