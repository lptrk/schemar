use jsonschema::JSONSchema;
use serde_json::Value as JsonValue;
use std::fs;

fn compare(schema_path: &str, json_data_path: &str) -> Result<(), std::io::Error> {
    let schema_str = fs::read_to_string(schema_path)?;
    let schema_json: JsonValue = serde_json::from_str(&schema_str)?;

    let schema = JSONSchema::compile(&schema_json).expect("Valid Schema");

    let json_str = fs::read_to_string(json_data_path)?;
    let json_data: JsonValue = serde_json::from_str(&json_str)?;

    match schema.validate(&json_data) {
        Ok(_) => println!("JSON ==  Schema."),
        Err(errors) => {
            println!("JSON != Schema:");
            for error in errors {
                println!("- {}", error)
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = compare("./assets/json/schema.json", "./assets/json/index.json") {
        eprintln!("Error in comparison: {}", e);
    }
}
