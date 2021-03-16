use datafusion::error::Result as DataFusionResult;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> DataFusionResult<()> {
    // Initialize query interface
    let mut ctx = ExecutionContext::new();

    ctx.register_csv("scores", "../../data/StudentACTResults.csv", CsvReadOptions::new())?;

    let df = ctx.sql("SELECT group, AVG(english), AVG(reading), AVG(math), AVG(science)
                            FROM scores
                            GROUP BY group
                            ORDER BY group")?;

    let results = df.collect().await?;

    println!("{:?}", results);

    Ok(())
}

