#![recursion_limit = "256"]

use std::fs::File;
use std::io::{BufReader, Write};
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema, schema_container_of, to_vec};
use borsh::schema::{BorshSchemaContainer};
use borsh_serde_adapter::deserialize_adapter::deserialize_from_schema;
use borsh_serde_adapter::serialize_adapter::serialize_serde_json_to_borsh;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use borsh_serde_adapter::borsh_schema_util::write_schema_as_json;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct Person {
    first_name: String,
    last_name: String
}

impl Default for Person {
    fn default() -> Self {
        Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum TestEnum {
    One(u8),
    Two(u8),
    Three(u8)
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct AllTypes {
    type_u8: u8,
    type_u16: u16,
    type_u32: u32,
    type_u64: u64,
    type_u128: u128,
    type_i8: i8,
    type_i16: i16,
    type_i32: i32,
    type_i64: i64,
    type_i128: i128,
    type_f32: f32,
    type_f64: f64,
    type_string: String,
    type_bool: bool,
    type_array: [u8; 3],
    type_sequence: Vec::<String>,
    type_tuple: (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, String, bool),
    type_enum: TestEnum,
    type_struct: Person,
}

impl AllTypes {
    fn new() -> AllTypes {
        AllTypes {
            type_u8: u8::MAX,
            type_u16: u16::MAX,
            type_u32: u32::MAX,
            type_u64: u64::MAX,
            type_u128: u128::MAX,
            type_i8: i8::MIN,
            type_i16: i16::MIN,
            type_i32: i32::MIN,
            type_i64: i64::MIN,
            type_i128: i128::MIN,
            type_f32: f32::MIN,
            type_f64: f64::MIN,
            type_string: "abc".to_string(),
            type_bool: false,
            type_array: [b'a', b'b', b'c'],
            type_sequence: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            type_tuple: (u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX, i8::MIN, i16::MIN, i32::MIN, i64::MIN, i128::MIN, f32::MAX, f64::MAX, "xyz".to_string(), true),
            type_enum: TestEnum::One(1),
            type_struct: Person::default(),
        }
    }
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct AllTypesExcept128 {
    type_u8: u8,
    type_u16: u16,
    type_u32: u32,
    type_u64: u64,
    type_i8: i8,
    type_i16: i16,
    type_i32: i32,
    type_i64: i64,
    type_f32: f32,
    type_f64: f64,
    type_string: String,
    type_bool: bool,
    type_array: [u8; 3],
    type_sequence: Vec::<String>,
    type_tuple: (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, String, bool),
    type_enum: TestEnum,
    type_struct: Person,
}

impl AllTypesExcept128 {
    fn new() -> AllTypesExcept128 {
        AllTypesExcept128 {
            type_u8: u8::MAX,
            type_u16: u16::MAX,
            type_u32: u32::MAX,
            type_u64: u64::MAX,
            type_i8: i8::MIN,
            type_i16: i16::MIN,
            type_i32: i32::MIN,
            type_i64: i64::MIN,
            type_f32: f32::MIN,
            type_f64: f64::MIN,
            type_string: "abc".to_string(),
            type_bool: false,
            type_array: [b'a', b'b', b'c'],
            type_sequence: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            type_tuple: (u8::MAX, u16::MAX, u32::MAX, u64::MAX, i8::MIN, i16::MIN, i32::MIN, i64::MIN, f32::MAX, f64::MAX, "xyz".to_string(), true),
            type_enum: TestEnum::One(1),
            type_struct: Person::default(),
        }
    }
}

#[test]
fn deserialize_from_borsh_schema() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    let container: BorshSchemaContainer = schema_container_of::<Person>();

    let person_ser = to_vec(&person).expect("Error trying to seralize Person");

    let result = deserialize_from_schema(&mut person_ser.as_slice(), &container).expect("Deserialization from schema failed");
    assert_eq!(result, json!({"first_name": "John", "last_name": "Doe"}));
}

#[test]
fn deserialize_from_borsh_schema_from_file() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    let mut defs = Default::default();
    Person::add_definitions_recursively(&mut defs);
    let container: BorshSchemaContainer = schema_container_of::<Person>();
    let data = to_vec(&container)
        .expect("Failed to serialize BorshSchemaContainer");
    let mut file = File::create("./tests/schema/person_schema.dat").expect("Failed to create file");
    file.write_all(&data).expect("Failed to write file");

    let person_ser = to_vec(&person).expect("Error trying to seralize Person");

    let file = File::open("./tests/schema/person_schema.dat").unwrap();
    let mut reader = BufReader::new(file);
    let container_from_file = BorshSchemaContainer::deserialize_reader(&mut reader).expect("Deserializing BorshSchemaContainer failed.");

    let result = deserialize_from_schema(&mut person_ser.as_slice(), &container_from_file).expect("Deserializing from schema failed.");
    assert_eq!(result, json!({"first_name": "John", "last_name": "Doe"}));
}

#[test]
fn serialize_from_borsh_schema() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    let container: BorshSchemaContainer = schema_container_of::<Person>();

    let person_value = serde_json::to_value(person).expect("Error serializing person");
    let mut person_writer = Vec::new();
    assert!(person_writer.len() == 0);

    let _ = serialize_serde_json_to_borsh(&mut person_writer, &person_value, &container).expect("Serialization failed");
    assert!(person_writer.len() > 0);

    let result = deserialize_from_schema(&mut person_writer.as_slice(), &container).expect("Deserialization failed");
    assert_eq!(result, json!({"first_name": "John", "last_name": "Doe"}));
}

#[test]
fn serialize_from_borsh_schema_with_string() {
    let file = File::open("./tests/schema/person_schema.dat").unwrap();
    let mut reader = BufReader::new(file);
    let person_schema = BorshSchemaContainer::deserialize_reader(&mut reader).expect("Deserializing BorshSchemaContainer failed.");

    let person_value = json!({"first_name": "John", "last_name": "Doe"});
    let mut person_writer = Vec::new();

    let _ = serialize_serde_json_to_borsh(&mut person_writer, &person_value, &person_schema).expect("Serialization failed");

    let _result = deserialize_from_schema(&mut person_writer.as_slice(), &person_schema).expect("Deserialization failed");
}

#[test]
fn all_types_deserialize_test() {
    let all_types = AllTypes::new();

    let container: BorshSchemaContainer = schema_container_of::<AllTypes>();

    let all_types_ser = to_vec(&all_types).expect("Error trying to serialize Person");

    let result = deserialize_from_schema(&mut all_types_ser.as_slice(), &container).expect("Deserialization from schema failed");
    println!("{:?}", result);

    assert_eq!(result["type_array"], json!([97,98,99]));
    assert_eq!(result["type_bool"], json!(false));
    assert_eq!(result["type_enum"], json!({"One": [1]}));
    assert_eq!(result["type_f32"], json!(f32::MIN));
    assert_eq!(result["type_f64"], json!(f64::MIN));
    assert_eq!(result["type_i128"], json!("-170141183460469231731687303715884105728"));
    assert_eq!(result["type_i16"], json!(i16::MIN));
    assert_eq!(result["type_i32"], json!(i32::MIN));
    assert_eq!(result["type_i64"], json!(i64::MIN));
    assert_eq!(result["type_i8"], json!(i8::MIN));
    assert_eq!(result["type_sequence"], json!(["a", "b", "c"]));
    assert_eq!(result["type_string"], json!("abc"));
    assert_eq!(result["type_struct"], json!({"first_name": "John", "last_name": "Doe"}));
    assert_eq!(result["type_tuple"], json!([u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX.to_string(), i8::MIN, i16::MIN, i32::MIN, i64::MIN, i128::MIN.to_string(), f32::MAX, f64::MAX, "xyz".to_string(), true]));
    assert_eq!(result["type_u128"], json!(u128::MAX.to_string()));
    assert_eq!(result["type_u16"], json!(u16::MAX));
    assert_eq!(result["type_u32"], json!(u32::MAX));
    assert_eq!(result["type_u64"], json!(u64::MAX));
    assert_eq!(result["type_u8"], json!(u8::MAX));

    println!("{}", result.to_string());
}

#[test]
fn all_types_serialize_test() {
    let all_types = AllTypesExcept128::new();

    let container: BorshSchemaContainer = schema_container_of::<AllTypesExcept128>();
    let all_types_value = serde_json::to_value(all_types).expect("Error serializing all_types");
    let mut all_types_writer = Vec::new();
    assert_eq!(all_types_writer.len(), 0);

    serialize_serde_json_to_borsh(&mut all_types_writer, &all_types_value, &container).expect("Serialization failed");

    assert!(all_types_writer.len() > 0);

    let all_types = AllTypesExcept128::new();
    let vec = to_vec(&all_types).expect("blah");
    assert_eq!(all_types_writer.len(), vec.len());

    let result = deserialize_from_schema(&mut all_types_writer.as_slice(), &container).expect("Deserialization failed");
    println!("{}", result.to_string());

    assert_eq!(result["type_array"], json!([97,98,99]));
    assert_eq!(result["type_bool"], json!(false));
    assert_eq!(result["type_enum"], json!({"One": [1]}));
    assert_eq!(result["type_f32"], json!(f32::MIN));
    assert_eq!(result["type_f64"], json!(f64::MIN));
    assert_eq!(result["type_i16"], json!(i16::MIN));
    assert_eq!(result["type_i32"], json!(i32::MIN));
    assert_eq!(result["type_i64"], json!(i64::MIN));
    assert_eq!(result["type_i8"], json!(i8::MIN));
    assert_eq!(result["type_sequence"], json!(["a", "b", "c"]));
    assert_eq!(result["type_string"], json!("abc"));
    assert_eq!(result["type_struct"], json!({"first_name": "John", "last_name": "Doe"}));
    assert_eq!(result["type_tuple"], json!([u8::MAX, u16::MAX, u32::MAX, u64::MAX, i8::MIN, i16::MIN, i32::MIN, i64::MIN, f32::MAX, f64::MAX, "xyz".to_string(), true]));
    assert_eq!(result["type_u16"], json!(u16::MAX));
    assert_eq!(result["type_u32"], json!(u32::MAX));
    assert_eq!(result["type_u64"], json!(u64::MAX));
    assert_eq!(result["type_u8"], json!(u8::MAX));
}

#[test]
fn schema_to_json_test() {
    let _ = write_schema_as_json(AllTypes::new(), "./tests/schema/all_types_schema.json".to_string());
    let file = File::open("./tests/schema/all_types_schema.json").unwrap();
    let reader = BufReader::new(file);
    let result: Value = serde_json::from_reader(reader).expect("Deserialization failed");
    assert_eq!(result.is_object(), true);
}