use std::{collections::HashMap, fs::File, io::Read};

use uuid::Uuid;

use self::models::{parse_command_line_arguments, Record};

mod models;

fn main() {
    println!("Hello, world!");
    let command_line_arguments = parse_command_line_arguments();

    println!("{:?}", command_line_arguments);

    let mut file = File::open(command_line_arguments.file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let data = data
        .lines()
        .map(|line| {
            let splitter = line.splitn(18, ",");
            let mut elements: Vec<&str> = splitter.collect();
            let note = elements[17].replace(",", "，");
            elements[17] = note.as_str();
            elements.join(",")
        })
        .collect::<Vec<String>>()
        .join("\n");

    let mut records = vec![];
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    for result in reader.deserialize() {
        let record: Record = result.unwrap();
        println!("{:?}", record);
        records.push(record);
    }

    let mut folders = HashMap::new();
    for record in records {
        if let Some(category) = record.category {
            if !folders.contains_key(&category) {
                folders.insert(category, Uuid::now_v7().to_string());
            }
        }
    }

    println!("{:?}", folders);
}
