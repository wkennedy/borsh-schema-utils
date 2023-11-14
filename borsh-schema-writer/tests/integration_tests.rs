use std::fs::File;
use std::io::BufReader;
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
use borsh::schema::{BorshSchemaContainer, Definition, Fields};
use borsh_schema_writer::schema_writer::{schema_to_bytes, write_schema};

#[derive(Debug, Default, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct Person {
    first_name: String,
    last_name: String
}

#[test]
fn write_schema_test() {
    let _ = write_schema(Person::default(), "./tests/schema/person_schema.dat".to_string());

    let file = File::open("./tests/schema/person_schema.dat").unwrap();
    let mut reader = BufReader::new(file);
    let container_from_file = BorshSchemaContainer::deserialize_reader(&mut reader).expect("Deserialization for BorshSchemaContainer failed");

    assert_eq!(container_from_file.declaration().to_string(), "Person");
    let definition = container_from_file.get_definition(container_from_file.declaration().as_str()).unwrap();
    assert!(matches!(definition, Definition::Struct { .. }));

    match definition {
        Definition::Struct { fields } => match fields {
            Fields::NamedFields(fields) => {
                for (key, value_declaration) in fields {
                    assert!(key.as_str() == "first_name" || key.as_str() == "last_name");
                    assert_eq!(value_declaration.as_str(), "String");
                }
            }
            _ => {assert!(false)}
        },
        _ => {assert!(false)}
    }
}

#[test]
fn write_to_bytes_test() {
    let schema = schema_to_bytes(Person::default()).expect("Failed to serialize BorshSchemaContainer");

    let container_from_bytes = BorshSchemaContainer::deserialize(&mut schema.as_slice()).expect("Deserialization for BorshSchemaContainer failed");

    assert_eq!(container_from_bytes.declaration().to_string(), "Person");
    let definition = container_from_bytes.get_definition(container_from_bytes.declaration().as_str()).unwrap();
    assert!(matches!(definition, Definition::Struct { .. }));

    match definition {
        Definition::Struct { fields } => match fields {
            Fields::NamedFields(fields) => {
                for (key, value_declaration) in fields {
                    assert!(key.as_str() == "first_name" || key.as_str() == "last_name");
                    assert_eq!(value_declaration.as_str(), "String");
                }
            }
            _ => {assert!(false)}
        },
        _ => {assert!(false)}
    }
}