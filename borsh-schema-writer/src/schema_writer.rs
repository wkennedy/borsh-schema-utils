use std::fs::File;
use std::io::Write;
use borsh::{BorshSchema, to_vec};
use borsh::schema::BorshSchemaContainer;

/// This function takes a Struct with the BorshSchema trait and writes the schema to a specified file.
///```rust
///fn write_schema_test() {
///     let _ = write_schema(Person::default(), "./tests/schema/person_schema.dat".to_string());
///
///     let file = File::open("./tests/schema/person_schema.dat").unwrap();
///     let mut reader = BufReader::new(file);
///     let container_from_file = BorshSchemaContainer::deserialize_reader(&mut reader).expect("Deserialization for BorshSchemaContainer failed");
///}
/// ```
pub fn write_schema<T: BorshSchema>(_: T, file_path: String) -> std::io::Result<()> {
    let mut defs = Default::default();
    T::add_definitions_recursively(&mut defs);
    let container: BorshSchemaContainer = BorshSchemaContainer::for_type::<T>();
    let data = to_vec(&container)
        .expect("Failed to serialize BorshSchemaContainer");
    let mut file = File::create(file_path).expect("Failed to create borsh schema file");
    file.write_all(&data).expect("Failed to write file");
    Ok(())
}

/// This function takes a Struct with the BorshSchema trait and writes the schema to a specified file.
///```rust
///fn write_to_bytes_test() {
///    let schema = schema_to_bytes(Person::default()).expect("Failed to serialize BorshSchemaContainer");
///
///    let container_from_bytes = BorshSchemaContainer::deserialize(&mut schema.as_slice()).expect("Deserialization for BorshSchemaContainer failed");
///}
/// ```
pub fn schema_to_bytes<T: BorshSchema>(_: T) -> std::io::Result<Vec<u8>> {
    let mut defs = Default::default();
    T::add_definitions_recursively(&mut defs);
    let container: BorshSchemaContainer = BorshSchemaContainer::for_type::<T>();
    let data = to_vec(&container)
        .expect("Failed to serialize BorshSchemaContainer");
    Ok(data)
}