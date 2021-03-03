use std::fs::File;

use arrow::array::{Array, ArrayRef, BooleanArray, Int64Array};
use arrow::compute::kernels::filter;
use arrow::csv;
use arrow::datatypes::DataType;
use arrow::error::Result as ArrowResult;
use arrow::record_batch::RecordBatch;

fn main() -> ArrowResult<()> {
    let file = File::open("../../data/StudentACTResults.csv").unwrap();

    // Configure CSV builder
    let csv_builder = csv::ReaderBuilder::new()
        .has_header(true)
        .infer_schema(Some(100));

    // Build CSV reader
    let mut csv_reader = csv_builder.build(file)?;

    // Get next batch record from CSV reader
    // CSV reader is an iterator over RecordBatch results
    let batch = csv_reader.next().unwrap().unwrap();

    let filtered_batch = filter_by_group(1, &batch)?;

    println!("{:?}", filtered_batch);

    Ok(())
}

fn filter_by_group(group: i64, batch: &RecordBatch) -> ArrowResult<RecordBatch> {
    // Create a boolean array that determines which values are filtered out
    let filter_array = batch
        .column(2)
        .as_any()
        .downcast_ref::<Int64Array>()
        .unwrap()
        .iter()
        .map(|value| Some(value == Some(group)))
        .collect::<BooleanArray>();

    let mut arrays: Vec<ArrayRef> = Vec::new();

    // Iterate over the columns and apply filter
    for idx in 0..batch.num_columns() {
        let array = batch.column(idx);

        // Downcast array ref to specific array implementation based on type
        let array = match array.data_type() {
            &DataType::Int64 => array.as_any().downcast_ref::<Int64Array>().unwrap() as &dyn Array,
            &DataType::Boolean => {
                array.as_any().downcast_ref::<BooleanArray>().unwrap() as &dyn Array
            }
            _ => panic!("Data type not supported"),
        };

        // Apply filter to column;
        let filtered = filter::filter(array, &filter_array)?;
        
        arrays.push(filtered);
    }

    // Create a new record batch from filtered results
    RecordBatch::try_new(batch.schema(), arrays)
}
