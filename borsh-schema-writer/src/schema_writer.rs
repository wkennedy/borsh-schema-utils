use std::fs::File;
use std::io::Write;
use borsh::{BorshSchema, BorshSerialize};
use borsh::schema::BorshSchemaContainer;

/// This function takes a Struct with the BorshSchema trait and writes the schema to a specified file.
pub fn write_schema<T: BorshSchema>(_: T, file_path: String) -> std::io::Result<()> {
    let mut defs = Default::default();
    T::add_definitions_recursively(&mut defs);
    let container: BorshSchemaContainer = T::schema_container();
    let data = container
        .try_to_vec()
        .expect("Failed to serialize BorshSchemaContainer");
    let mut file = File::create(file_path).expect("Failed to create borsh schema file");
    file.write_all(&data).expect("Failed to write file");
    Ok(())
}