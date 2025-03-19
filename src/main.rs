use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write as _},
};

use serde_json::json;
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

    let mut items = Vec::new();
    let mut categories = HashMap::new();
    for record in records {
        if let Some(category) = record.category.clone() {
            if !categories.contains_key(&category) {
                categories.insert(category, Uuid::now_v7().to_string());
            }
        }

        let mut fields = Vec::new();
        if let Some(identifier2) = record.identifier2.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "識別子2",
                "type": 0,
                "value": identifier2
            }));
        }
        if let Some(identifier3) = record.identifier3.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "識別子3",
                "type": 0,
                "value": identifier3
            }));
        }
        if let Some(identifier4) = record.identifier4.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "識別子4",
                "type": 0,
                "value": identifier4
            }));
        }
        if let Some(identifier5) = record.identifier5.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "識別子5",
                "type": 0,
                "value": identifier5
            }));
        }
        if let Some(password2) = record.password2.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "パスワード2",
                "type": 1,
                "value": password2
            }));
        }
        if let Some(password3) = record.password3.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "パスワード3",
                "type": 1,
                "value": password3
            }));
        }
        if let Some(password4) = record.password4.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "パスワード4",
                "type": 1,
                "value": password4
            }));
        }
        if let Some(password5) = record.password5.clone() {
            fields.push(json!({
                "linkedId": null,
                "name": "パスワード5",
                "type": 1,
                "value": password5
            }));
        }

        let mut uris = Vec::new();
        if let Some(url1) = record.url1.clone() {
            uris.push(json!({ "match": null, "uri": url1 }));
        }
        if let Some(url2) = record.url2.clone() {
            uris.push(json!({ "match": null, "uri": url2 }));
        }
        if let Some(url3) = record.url3.clone() {
            uris.push(json!({ "match": null, "uri": url3 }));
        }
        if let Some(url4) = record.url4.clone() {
            uris.push(json!({ "match": null, "uri": url4 }));
        }
        if let Some(url5) = record.url5.clone() {
            uris.push(json!({ "match": null, "uri": url5 }));
        }

        items.push(json!({
            "passwordHistory": null,
            "revisionDate": chrono::Local::now().to_rfc3339(),
            "creationDate": chrono::Local::now().to_rfc3339(),
            "deletedDate": null,
            "id": Uuid::now_v7().to_string(),
            "organizationId": null,
            "folderId": categories.get(&record.category.unwrap()).unwrap().to_string(),
            "type": 1,
            "reprompt": 0,
            "name": record.title,
            "notes": record.remarks,
            "favorite": false,
            "fields": fields,
            "login": {
                "fido2Credentials": [],
                "uris": uris,
                "username": record.identifier1.unwrap_or("".to_string()),
                "password": record.password1.unwrap_or("".to_string()),
                "totp": null
            }

        }));
    }

    let mut folders = Vec::new();
    for (category, id) in categories {
        folders.push(json!({
            "id": id,
            "name": category
        }));
    }

    let output = json!({
        "items": items,
        "folders": folders
    });

    println!("{}", output.to_string());

    let mut output_file = File::create("bitwarden_export_sis_pass_mgr_csv.json").unwrap();
    output_file
        .write_all(output.to_string().as_bytes())
        .unwrap();
}
