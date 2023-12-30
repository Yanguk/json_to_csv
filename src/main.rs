use serde_json::Value;
use std::{collections::HashSet, error::Error, fs::{OpenOptions, File}, io::{BufReader, Read}, path::Path};

const TEST_PATH: &str = "test.json";

fn main() {
    println!("convert json to csv");

    let args: Vec<String> = std::env::args().collect();
    let default_path = TEST_PATH.to_string();
    let fname = args.get(1).unwrap_or(&default_path);
    let path = Path::new(&fname);

    let v = read_user_from_file(path).unwrap();

    let array = v.as_array().unwrap();

    let output_path = path.with_extension("csv");

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .unwrap();

    let mut wtr = csv::Writer::from_writer(file);

    let unique_keys = array
        .iter()
        .flat_map(|obj| obj.as_object().unwrap().iter().map(|(key, _value)| key))
        .collect::<HashSet<_>>();

    let mut sorted_keys: Vec<_> = unique_keys.into_iter().collect();
    sorted_keys.sort();

    wtr.write_record(&sorted_keys).unwrap();

    array
        .iter()
        .map(|obj| {
            let row: Vec<String> = sorted_keys
                .iter()
                .map(|key| {
                    match &obj[key] {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => "".to_string(), // Handle other types as needed
                    }
                })
                .collect();

            row
        })
        .for_each(|row| {
            wtr.write_record(&row).unwrap();
            wtr.flush().unwrap();
        });

    println!("success");

    let mut file_content = String::new();
    let mut file = File::open(&output_path).unwrap();
    file.read_to_string(&mut file_content).unwrap();

    println!("<Reading CSV file>");
    println!("{}", file_content);
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Could not open file");

    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let v = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(v)
}
