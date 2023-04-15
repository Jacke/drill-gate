use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use jsonschema::{ JSONSchema, Draft };
use serde::{ Deserialize, Serialize };
use serde_json::{ json, Value };

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entity {
    id: String,
    data: HashMap<String, Value>,
}

fn main() {
    // Load the JSON schema from disk
    let mut file = File::open("entity_schema.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let schema: Value = serde_json::from_str(&contents).unwrap();

    // Compile the JSON schema into a JSONSchema instance
    let compiled_schema = JSONSchema::compile(&schema).unwrap();

    // Create an in-memory storage for the entities
    let mut entities: HashMap<String, Entity> = HashMap::new();

    // Add some entities to the storage
    let entity1 = Entity {
        id: "1".to_string(),
        data: [
            ("name".to_string(), json!("John Doe")),
            ("age".to_string(), json!(30)),
            ("address".to_string(), json!("123 Main St")),
        ]
            .iter()
            .cloned()
            .collect(),
    };
    add_entity(&entity1, &compiled_schema, &mut entities);

    let entity2 = Entity {
        id: "2".to_string(),
        data: [
            ("name".to_string(), json!("Jane Smith")),
            ("age".to_string(), json!(25)),
            ("address".to_string(), json!("456 Elm St")),
        ]
            .iter()
            .cloned()
            .collect(),
    };
    add_entity(&entity2, &compiled_schema, &mut entities);

    // Example usage: add a new entity to the storage
    let new_entity = Entity {
        id: "3".to_string(),
        data: [
            ("name".to_string(), json!("Bob Johnson")),
            ("age".to_string(), json!(40)),
            ("address".to_string(), json!("789 Oak St")),
        ]
            .iter()
            .cloned()
            .collect(),
    };
    add_entity(&new_entity, &compiled_schema, &mut entities);

    // Example usage: update an existing entity in the storage
    let updated_entity = Entity {
        id: "1".to_string(),
        data: [
            ("name".to_string(), json!("John Doe Jr.")),
            ("age".to_string(), json!(1)),
            ("address".to_string(), json!("123 Main St")),
        ]
            .iter()
            .cloned()
            .collect(),
    };
    // update_entity(&updated_entity, &compiled_schema, &mut entities);

    // Example usage: remove an existing entity from the storage
    // remove_entity("2", &mut entities);

    // Example usage: get all entities from the storage
    //let all_entities = get_entities(None, &entities);
    //println!("All entities: {:?}", all_entities);

    // Example usage: get entities by name from the storage
    //let entities_by_name = get_entities(Some(json!("John Doe Jr.".to_string())), &entities);
    //println!("Entities by name: {:?}", entities_by_name);
}

// Add a new entity to the storage
fn add_entity(entity: &Entity, schema: &JSONSchema, entities: &mut HashMap<String, Entity>) {
    if schema.is_valid(&json!(entity.data)) {
        entities.insert(entity.id.clone(), entity.clone());
    } else {
        println!("Invalid entity: {:?}", entity);
    }
}