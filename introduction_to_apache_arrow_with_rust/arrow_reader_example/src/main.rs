use std::fs::File;

use arrow::csv;
use arrow::error::Result;


fn main() -> Result<()> {
    let file = File::open("../../data/StudentACTResults.csv").unwrap();
    
    // Configure CSV builder that infers
    // the schema of the CSV file
    let csv_builder = csv::ReaderBuilder::new()
        .has_header(true)
        .infer_schema(Some(100));

    // Build CSV reader
    let mut csv_reader = csv_builder.build(file)?;

    // Get next batch record from CSV reader
    // CSV reader is an iterator over RecordBatch results
    let batch = csv_reader.next().unwrap().unwrap();

    println!("{:?}", batch);

    Ok(())
}
