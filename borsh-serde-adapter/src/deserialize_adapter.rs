use std::collections::HashMap;
use std::io::{Error};
use std::u128;

use borsh::schema::{BorshSchemaContainer, Definition, Fields};
use borsh::{BorshDeserialize};
use log::debug;
use serde_json::json;

//Deserializes borsh serialized bytes to serde_json::Value
fn deserialize_to_serde_json_by_type<T: BorshDeserialize + Into<serde_json::Value>>(buffer: &mut &[u8], type_name: &str) -> std::io::Result<serde_json::Value> {
    T::deserialize(buffer)
        .map(Into::into)
        .map_err(|_err| Error::new(std::io::ErrorKind::InvalidData, type_name))
}

fn deserialize_to_serde_json(buffer: &mut &[u8], schema: &BorshSchemaContainer, declaration: &borsh::schema::Declaration) -> std::io::Result<serde_json::Value> {
    match &declaration[..] {
        "u8" => deserialize_to_serde_json_by_type::<u8>(buffer, "u8"),
        "u16" => deserialize_to_serde_json_by_type::<u16>(buffer, "u16"),
        "u32" => deserialize_to_serde_json_by_type::<u32>(buffer, "u32"),
        "u64" => deserialize_to_serde_json_by_type::<u64>(buffer, "u64"),
        "u128" => u128::deserialize(buffer)
            .map(|value| value.to_string().into())
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "u128")),
        "i8" => deserialize_to_serde_json_by_type::<i8>(buffer, "i8"),
        "i16" => deserialize_to_serde_json_by_type::<i16>(buffer, "i16"),
        "i32" => deserialize_to_serde_json_by_type::<i32>(buffer, "i32"),
        "i64" => deserialize_to_serde_json_by_type::<i64>(buffer, "i64"),
        "i128" => i128::deserialize(buffer)
            .map(|value| value.to_string().into())
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "i128")),
        "f32" => deserialize_to_serde_json_by_type::<f32>(buffer, "f32"),
        "f64" => deserialize_to_serde_json_by_type::<f64>(buffer, "f64"),
        "String" => deserialize_to_serde_json_by_type::<String>(buffer, "String"),
        "bool" => deserialize_to_serde_json_by_type::<bool>(buffer, "bool"),

        _ => {
            if let Some(d) = schema.get_definition(declaration) {
                match d {
                    Definition::Primitive { .. } => {
                        let value = deserialize_to_serde_json(buffer, schema, declaration)?;

                        Ok(value)
                    }

                    Definition::Sequence { length_width, length_range, elements } => {
                        let length_width = *length_width as u32;
                        let length = if length_width == 0  {
                            *length_range.end() as usize
                        } else {
                            u32::deserialize(buffer)? as usize
                        };

                        let mut values = Vec::<serde_json::Value>::with_capacity(length);
                        for _ in 0..length {
                            let value = deserialize_to_serde_json(buffer, schema, elements)?;
                            values.push(value);
                        }
                        Ok(values.into())
                    }

                    Definition::Tuple { elements } => {
                        let mut values = Vec::<serde_json::Value>::with_capacity(elements.len());
                        for element in elements {
                            let value = deserialize_to_serde_json(buffer, schema, element)?;
                            values.push(value);
                        }
                        Ok(values.into())
                    }

                    Definition::Enum { tag_width: _, variants } => {
                        let variant_index = u8::deserialize(buffer)?;
                        let (_dicriminator_value, variant_name, variant_declaration) = &variants[variant_index as usize];
                        deserialize_to_serde_json(buffer, schema, variant_declaration)
                            .map(|value| json!({ variant_name: value }))
                    }

                    Definition::Struct { fields } => match fields {
                        Fields::NamedFields(fields) => {
                            let mut object = HashMap::<String, serde_json::Value>::new();
                            for (key, value_declaration) in fields {
                                let value = deserialize_to_serde_json(
                                    buffer,
                                    schema,
                                    value_declaration,
                                )?;
                                object.insert(key.to_string(), value);
                            }
                            Ok(serde_json::to_value(object)?)
                        }

                        Fields::UnnamedFields(elements) => {
                            let mut values = Vec::<serde_json::Value>::with_capacity(elements.len());
                            for element in elements {
                                let value = deserialize_to_serde_json(buffer, schema, element)?;
                                values.push(value);
                            }
                            Ok(values.into())
                        }

                        Fields::Empty => Ok(Vec::<u8>::new().into()),
                    }
                }
            } else {
                debug!("Can't deserialize unknown type: {:?}. Using Value::Null", declaration);
                Ok(serde_json::Value::Null)
            }
        }
    }
}

/// Deserializes borsh serialized bytes to serde_json::Value using the provided schema
pub fn deserialize_from_schema(buffer: &mut &[u8], schema: &BorshSchemaContainer) -> std::io::Result<serde_json::Value> {
    deserialize_to_serde_json(buffer, schema, schema.declaration())
}