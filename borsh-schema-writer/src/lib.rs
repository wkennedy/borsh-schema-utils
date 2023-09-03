//! ## borsh-schema-writer
//! This library provides a function to take a Struct with the BorshSchema trait and writes the schema to a specified file.
//! This file can then be hosted in a registry, file system, database, web storage, etc... for consumers to use. Here is an
//! example:
//! 
//! ```
//! use std::fs::File;
//! use std::io::BufReader;
//! use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
//! use borsh::schema::{BorshSchemaContainer, Definition, Fields};
//! use borsh_schema_writer::schema_writer::write_schema;
//! 
//! #[derive(Debug, Default, BorshSerialize, BorshDeserialize, BorshSchema)]
//! pub struct Person {
//!     first_name: String,
//!     last_name: String
//! }
//! 
//! fn write_schema_example() {
//!     write_schema(Person::default(), "./tests/schema/person_schema.dat".to_string());
//!     let file = File::open("./tests/schema/person_schema.dat").unwrap();
//!     let mut reader = BufReader::new(file);
//!     let container_from_file = BorshSchemaContainer::deserialize_reader(&mut reader).expect("Deserialization for BorshSchemaContainer failed");
//! }
//! ```
//!
pub mod schema_writer;