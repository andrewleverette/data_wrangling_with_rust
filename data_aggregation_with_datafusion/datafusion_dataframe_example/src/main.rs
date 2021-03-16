use datafusion::error::Result as DataFusionResult;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> DataFusionResult<()> {
    // Initialize query interface
    let mut ctx = ExecutionContext::new();

    // Creates a data frame that describes a query to scan a  CSV file,
    // calculate the average of each score by group,
    // then finally sort by group.
    let df = ctx
        .read_csv("../../data/StudentACTResults.csv", CsvReadOptions::new())?
        .aggregate(
            vec![col("group")],
            vec![
                avg(col("english")),
                avg(col("reading")),
                avg(col("math")),
                avg(col("science")),
            ],
        )?
        .sort(vec![col("group").sort(true, true)])?;

    // Execute the query defined by the data frame
    let results = df.collect().await?;

    println!("{:?}", results);

    Ok(())
}
