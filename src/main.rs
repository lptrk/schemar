use jsonschema::JSONSchema;
use serde_json::Value as JsonValue;
use std::fs;

fn compare(schema_path: &str, json_data_path: &str) -> Result<(), std::io::Error> {
    let schema_str = fs::read_to_string(schema_path)?;
    let schema_json: JsonValue = serde_json::from_str(&schema_str)?;

    let schema = JSONSchema::compile(&schema_json).expect("Valides Schema");

    let json_str = fs::read_to_string(json_data_path)?;
    let json_data: JsonValue = serde_json::from_str(&json_str)?;

    match schema.validate(&json_data) {
        Ok(_) => println!("Die JSON-Datei entspricht dem Schema."),
        Err(errors) => {
            println!("Die JSON-Datei entspricht nicht dem Schema:");
            for error in errors {
                println!("- {}", error)
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = compare(
        "/Users/lptrk/environment/repos/schemar/assets/json/schema.json",
        "/Users/lptrk/environment/repos/schemar/assets/json/index.json",
    ) {
        eprintln!("Fehler beim Vergleichen: {}", e);
    }
}
