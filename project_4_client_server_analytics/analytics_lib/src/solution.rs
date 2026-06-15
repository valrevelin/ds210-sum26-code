use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

fn row_matches(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(column_name, target_value) => {
            let index = dataset.column_index(column_name);
            row.get_value(index) == target_value
        }
        Condition::Not(inner) => {
            !row_matches(row, dataset, inner)
        }
        Condition::And(left, right) => {
            row_matches(row, dataset, left) && row_matches(row, dataset, right)
        }
        Condition::Or(left, right) => {
            row_matches(row, dataset, left) || row_matches(row, dataset, right)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut result = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if row_matches(row, dataset, filter) {
            result.add_row(row.clone());
        }
    }
    result
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let col_idx = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    let mut map_values = HashMap::new();
    for row in dataset.into_iter() {
        let key = row.get_value(col_idx).clone();
        map_values.entry(key)
            .or_insert_with(|| Dataset::new(columns.clone()))
            .add_row(row);
    }
    map_values
}


fn summing(group_dataset: &Dataset, column_name: &String) -> i32 {
    let col_idx = group_dataset.column_index(column_name);
    let mut sum = 0;
    for row in group_dataset.iter() {
        if let Value::Integer(n) = row.get_value(col_idx) {
            sum += n;
        }
    }
    sum
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();
    match aggregation {
        Aggregation::Count(_column_name) => {
            for (group_value, group_dataset) in &dataset {
                let count = group_dataset.len();
                result.insert(group_value.clone(),Value::Integer(count as i32));
            }
        }
        Aggregation::Sum(column_name) => {
            for (group_value, group_dataset) in &dataset {
                result.insert(group_value.clone(), Value::Integer(summing(group_dataset, column_name))); 
            }
        }
        Aggregation::Average(column_name) => {
            for (group_value, group_dataset) in &dataset {
                let count = group_dataset.len() as i32;
                let avg = summing(group_dataset, column_name)/count;
                result.insert(group_value.clone(), Value::Integer(avg)); 
            }
        }
    }
    result 
}
    

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}