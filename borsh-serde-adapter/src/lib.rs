
//! ## borsh-serde-adapter
//!
//! This library provides functions to deserialize data that was serialized with Borsh and returns a serde_json value when
//! provided with the borsh schema binary file generated by the borsh-schema-writer library. It will take a serde_json value
//! and serialize using borsh. Here is an example of deserialization:
//!
//! ```
//! fn deserialize_from_borsh_schema_from_file() {
//!     let person = Person {
//!         first_name: "John".to_string(),
//!         last_name: "Doe".to_string(),
//!     };
//!
//!     let person_ser = person.try_to_vec().expect("Error trying to seralize Person");
//!
//!     let file = File::open("./tests/schema/person_schema.dat").unwrap();
//!     let mut reader = BufReader::new(file);
//!     let container_from_file = BorshSchemaContainer::deserialize_reader(&mut reader).expect("Deserializing BorshSchemaContainer failed.");
//!
//!     let result = deserialize_from_schema(&mut person_ser.as_slice(), &container_from_file).expect("Deserializing from schema failed.");
//!     println!("{}", result.to_string());
//! }
//! ```
//!
//! In this example you can see that we aren't deserializing with a struct with the BorshDeserialize trait. Typically you
//! would do something like this:
//!
//! ```
//!     let person_serialized = person.try_to_vec().expect("Error trying to serialize Person");
//!     let person_deserialized = Person::deserialize(&mut person_serialized.as_slice());
//! ```
//!
//! But with the schema, we can go from borsh serialized, directly to serde_json and then we can easily get JSON as a
//! string. This makes it much easier for other consumers to handle.
//!
//! We can do the do something similar with serializing from serde_json to borsh.
//!
//!
//! ```
//! fn serialize_from_borsh_schema() {
//!     let person = Person {
//!         first_name: "John".to_string(),
//!         last_name: "Doe".to_string(),
//!     };
//!
//!     let file = File::open("./tests/schema/person_schema.dat").unwrap();
//!     let mut reader = BufReader::new(file);
//!     let container = BorshSchemaContainer::deserialize_reader(&mut reader).expect("Deserializing BorshSchemaContainer failed.");
//!
//!     let person_value = serde_json::to_value(person).expect("Error serializing person");
//!     let mut person_writer = Vec::new();
//!     assert!(person_writer.len() == 0);
//!
//!     let _ = serialize_serde_json_to_borsh(&mut person_writer, &person_value, &container).expect("Serialization failed");
//! }
//! ```
//!
//! Here you can see instead of serializing from the Person struct with the BorshSerialize trait, we were able to go from
//! serde_json directly to borsh serialization, with person_writer containing the serialized struct as a vec of bytes.
//!
//! **Borsh Schema to JSON**
//!
//! It's possible to get the schema as JSON. This is useful for consumers that don't have access to the schema binary file.
//! This can be done using the write_schema_as_json function. Here is an example:
//!
//! ```
//! fn schema_to_json_test() {
//!     write_schema_as_json(Person::default(), "./tests/schema/person_schema.json".to_string());
//!     let file = File::open("./tests/schema/person_schema.json").unwrap();
//!     let reader = BufReader::new(file);
//!     let result: Value = serde_json::from_reader(reader).expect("Deserialization failed");
//!     println!("{}", result.to_string());
//! }
//! ```
//!
//! This will result in the following JSON:
//!
//! ```
//! {
//!   "declaration": "Person",
//!   "definitions": [
//!     [
//!       "Person",
//!       {
//!         "Struct": {
//!           "fields": {
//!             "NamedFields": [
//!               [
//!                 [
//!                   "first_name",
//!                   "string"
//!                 ],
//!                 [
//!                   "last_name",
//!                   "string"
//!                 ]
//!               ]
//!             ]
//!           }
//!         }
//!       }
//!     ]
//!   ]
//! }
//! ```
//!
//! **Caveats**
//!
//! This library is still in early development and there are some caveats to be aware of. The use of u128 and i128 are
//! somewhat supported. In the case of deserialization u128/i128 are deserialized as strings, but serialization is not
//! supported.
 
pub mod deserialize_adapter;
pub mod serialize_adapter;
pub mod errors;
pub mod borsh_schema_util;