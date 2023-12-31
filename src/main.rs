use serde_json::Value;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufReader, Read},
    path::Path, collections::HashSet,
};

fn main() {
    println!("convert json to csv");

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).unwrap();

    let path = Path::new(&fname);
    let json_value = read_json_from_file(path).unwrap();

    let array = json_value.as_array().unwrap();

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
        .flat_map(|obj| obj.as_object().unwrap().iter().enumerate().map(|(idx, (key, _value))| (key, idx)))
        .collect::<HashSet<_>>();

    let mut sorted_keys: Vec<_> = unique_keys.iter().collect();

    sorted_keys.sort_by(|a, b| a.1.cmp(&b.1));

    let sorted_keys = sorted_keys.iter().map(|(key, _idx)| key).collect::<Vec<_>>();

    wtr.write_record(&sorted_keys).unwrap();

    array
        .iter()
        .map(|obj| {
            sorted_keys
                .iter()
                .map(|key| {
                    let item = &obj.get(key);

                    match item {
                        Some(Value::String(s)) => s.clone(),
                        Some(Value::Number(n)) => n.to_string(),
                        Some(Value::Null) => "".to_string(),
                        None => "".to_string(),
                        _ => {
                            panic!("unexpected type")
                        }
                    }
                })
                .collect::<Vec<String>>()
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

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Could not open file");

    let reader = BufReader::new(file);

    let v = serde_json::from_reader(reader)?;

    Ok(v)
}
