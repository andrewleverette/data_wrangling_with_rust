use datafusion::error::Result as DataFusionResult;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> DataFusionResult<()> { 
    let mut ctx = ExecutionContext::new();

    // Creates a data frame that scans a CSV file
    let df = ctx.read_csv("../../data/StudentACTResults.csv", CsvReadOptions::new())?;

    // Transforms logical plan to be a scan and filter
    let df = df.filter(col("group").eq(lit(1)))?;

    // Executes the logical plan and returns results
    // as an array of RecordBatch
    let results = df.collect().await?;

    println!("{:?}", results);

    Ok(())
}
