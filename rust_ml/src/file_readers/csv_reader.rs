use fast_float::parse;
use std::error::Error;

pub fn read_csv() -> Result<(Vec<f64>, Vec<i32>), Box<dyn Error>> {
    // This line declares a mutable variable named reader. The mut keyword indicates that the variable can be changed. The type of the variable will be inferred by the compiler
    // This part is a function call to the from_path method of the Reader type in the csv module.
    // It creates a new Reader instance that reads CSV data from the specified file path.
    // The path provided is /home/arthur/workspace/ia/rust-machine-learning/datasets/kc_house_data.csv.
    let mut reader = csv::Reader::from_path(
        "/home/arthur/workspace/ia/rust-machine-learning/datasets/kc_house_data.csv",
    )?;
    // The ? operator is used to propagate errors in Rust. In this context, it indicates that the function call may return an error, and if an error occurs, it will be returned from the main() function.
    // This requires the main() function to have a return type that supports returning an error, such as Result<(), Box<dyn std::error::Error>>
    // Build the CSV reader and iterate over each record.
    let mut price: Vec<f64> = Vec::new();
    let mut sqft_living: Vec<i32> = Vec::new();

    // for result in rdr.records(): This line starts a loop that iterates over the records returned by the records() method of the rdr object.
    // reader is a csv::Reader object. Each iteration of the loop will assign the current record to the variable result.
    for result in reader.records() {
        // Here, result? is used to extract the value from the Result returned by the iterator.
        // If the Result is an Ok variant, the value is assigned to the variable record.
        // If the Result is an Err variant, the error is propagated up the call stack.
        // This line assumes that the Result returned by the iterator is of type Result<StringRecord, Error>.
        let record = result?;

        // match record.get(2) starts a match expression to handle the value returned by record.get(2).
        // get() method is used to retrieve the field at index 2 (zero-based index) from the record, which should be of type StringRecord.
        match record.get(2) {
            // If record.get(2) returns Some(i), meaning the field at index 2 exists, the code block within the braces will execute.
            // The value of the field is assigned to the variable i.
            Some(i) => {
                // let tmp: f64 = parse(i).unwrap() parses the value of i (which is assumed to be a string) into a f64 floating-point number using the parse() function.
                // The unwrap() method is used to extract the parsed value from the Result returned by parse().
                // It assumes that the parsing operation is successful.
                let tmp: f64 = parse(i).unwrap();

                // tmp value is divided by 1000.0 and then added to a price vector using the push() method.
                // The / operator performs the division operation.
                price.push(tmp / 1000.0);
            }
            // The underscore _ is a catch-all pattern that matches any value. In this case, if record.get(2) returns None (i.e., the field doesn't exist), the code block does nothing.
            _ => (),
        }

        // This line starts another match expression to handle the value returned by record.get(5), which retrieves the field at index 5 from the record.
        match record.get(5) {
            // If record.get(5) returns Some(i), the code block within the braces will execute. The value of the field is assigned to the variable i.
            Some(i) => {
                // This line parses the value of i into a floating-point number using the parse() function and adds it to the sqft_living vector using the push() method.
                // It assumes that the parsing operation is successful.
                sqft_living.push(i.parse::<i32>().unwrap());
            }
            // Similar to before, if record.get(5) returns None, the code block does nothing.
            _ => (),
        }
    }

    return Ok((price, sqft_living));
}
