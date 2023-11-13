use std::fs::File;
use std::io::{Write};
use borsh::{BorshSchema, schema_container_of, to_vec};
use borsh::schema::BorshSchemaContainer;
use crate::deserialize_adapter::deserialize_from_schema;

///This function takes a Struct with the BorshSchema trait and writes the schema as JSON to a specified file.
///This is useful for portability to other languages.
pub fn write_schema_as_json<T: BorshSchema>(_: T, file_path: String) -> std::io::Result<()> {
    let mut defs = Default::default();
    T::add_definitions_recursively(&mut defs);
    let container: BorshSchemaContainer = BorshSchemaContainer::for_type::<T>();
    let data = to_vec(&container).expect("Failed to serialize BorshSchemaContainer");

    let mut con_defs = Default::default();
    BorshSchemaContainer::add_definitions_recursively(&mut con_defs);
    let con_container: BorshSchemaContainer = schema_container_of::<BorshSchemaContainer>();

    let result = deserialize_from_schema(&mut data.as_slice(), &con_container).expect("Deserialization failed");

    let mut file = File::create(file_path).expect("Failed to create borsh schema json file");
    file.write_all(result.to_string().as_bytes()).expect("Failed to write file");
    Ok(())
}

//TODO this is a nice to have, but not really needed if your project is using rust, since you can
//just use the binary schema generated. The borsh schema as JSON is meant for portability to other
//languages.
// pub fn read_schema_from_json<T: BorshSchema>(_: T, file_path: String) {
//
// }