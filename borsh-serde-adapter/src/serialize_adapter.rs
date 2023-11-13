use std::io::{Write};
use std::str::FromStr;

use anyhow::anyhow;
use borsh::schema::{BorshSchemaContainer, Definition, Fields};
use borsh::{BorshSerialize};
use log::debug;
use serde_json::{json};
use crate::errors::ExpectationError;

/// Serializes serde_json::Value to borsh serialized bytes using the provided schema
pub fn serialize_serde_json_to_borsh(writer: &mut impl Write, value: &serde_json::Value, schema: &BorshSchemaContainer) -> anyhow::Result<()> {
    serialize_serde_json_by_declaration_with_schema(writer, value, schema, schema.declaration())
}

fn serialize_signed_to_borsh<T: BorshSerialize + TryFrom<i64>>(writer: &mut impl Write, value: &serde_json::Value) -> anyhow::Result<()>
    where <T as TryFrom<i64>>::Error: std::error::Error + Send + Sync + 'static, {
    let v = value
        .as_i64()
        .ok_or(ExpectationError::Number)
        .map(T::try_from)??;
    BorshSerialize::serialize(&v, writer)?;
    Ok(())
}

fn serialize_unsigned_to_borsh<T: BorshSerialize + TryFrom<u64>>(writer: &mut impl Write, value: &serde_json::Value) -> anyhow::Result<()>
    where <T as TryFrom<u64>>::Error: std::error::Error + Send + Sync + 'static, {
    let value = value
        .as_u64()
        .ok_or(ExpectationError::Number)
        .map(T::try_from)??;
    BorshSerialize::serialize(&value, writer)?;
    Ok(())
}

fn serialize_serde_json_to_borsh_by_type<T: BorshSerialize + FromStr>(writer: &mut impl Write, value: &serde_json::Value) -> anyhow::Result<()>
    where <T as FromStr>::Err: std::error::Error + Send + Sync + 'static, {
    let value = value
        .as_str()
        .ok_or(ExpectationError::String)
        .map(T::from_str)??;
    BorshSerialize::serialize(&value, writer)?;
    Ok(())
}

fn serialize_serde_json_by_declaration_with_schema(
    writer: &mut impl Write,
    value: &serde_json::Value,
    schema: &BorshSchemaContainer,
    declaration: &borsh::schema::Declaration,
) -> anyhow::Result<()> {
    match &declaration[..] {
        "u8" => serialize_unsigned_to_borsh::<u8>(writer, value),
        "u16" => serialize_unsigned_to_borsh::<u16>(writer, value),
        "u32" => serialize_unsigned_to_borsh::<u32>(writer, value),
        "u64" => serialize_unsigned_to_borsh::<u64>(writer, value),
        "u128" => serialize_serde_json_to_borsh_by_type::<u128>(writer, value),
        "i8" => serialize_signed_to_borsh::<i8>(writer, value),
        "i16" => serialize_signed_to_borsh::<i16>(writer, value),
        "i32" => serialize_signed_to_borsh::<i32>(writer, value),
        "i64" => serialize_signed_to_borsh::<i64>(writer, value),
        "i128" => serialize_serde_json_to_borsh_by_type::<i128>(writer, value),
        "f32" => {
            //TODO Is there a better way to do this?
            let value = value.as_f64().ok_or(ExpectationError::Number)? as f32;
            BorshSerialize::serialize(&value, writer)?;
            Ok(())
        },
        "f64" => {
            let value = value.as_f64().ok_or(ExpectationError::Number)?;
            BorshSerialize::serialize(&value, writer)?;
            Ok(())
        },
        "String" => serialize_serde_json_to_borsh_by_type::<String>(writer, value),
        "bool" => {
            let value = value.as_bool().ok_or(ExpectationError::Boolean)?;
            BorshSerialize::serialize(&value, writer)?;
            Ok(())
        }
        _ => {
            if let Some(definition) = schema.get_definition(declaration) {
                match definition {

                    Definition::Primitive { .. } => {
                        serialize_serde_json_by_declaration_with_schema(writer, value, schema, declaration)?;
                        Ok(())
                    }

                    //TODO cleanup
                    Definition::Sequence { length_width, length_range, elements } => {
                        let sequence = value.as_array().ok_or(ExpectationError::Array)?;
                        let length_width = *length_width as u32;
                        let mut length = 0;
                        if length_width == 0  {
                            length = *length_range.end() as usize
                        } else {
                            //Only serialize length for dynamically sized sequence (vec)
                            length = sequence.len();
                            BorshSerialize::serialize(&(length as u32), writer)?;
                        };
                        for item in sequence {
                            serialize_serde_json_by_declaration_with_schema(writer, item, schema, elements)?;
                        }
                        Ok(())
                    }

                    Definition::Tuple { elements } => {
                        let tuple = value.as_array().ok_or(ExpectationError::Array)?;
                        if tuple.len() != elements.len() {
                            return Err(
                                ExpectationError::ArrayOfLength(elements.len() as u32).into()
                            );
                        }
                        for (declaration, value) in elements.iter().zip(tuple) {
                            serialize_serde_json_by_declaration_with_schema(writer, value, schema, declaration)?;
                        }
                        Ok(())
                    }

                    Definition::Enum { variants, .. } => {
                        let (input_variant, variant_values) = value
                            .as_object()
                            .and_then(|o| o.keys().next().map(|s| (s.as_str(), Some(&o[s]))))
                            .or_else(|| value.as_str().map(|s| (s, None)))
                            .ok_or(ExpectationError::Object)?;

                        let (variant_index, variant_declaration) = variants
                            .iter()
                            .enumerate()
                            .find_map(|(i, (.., v))| {
                                Some((i, v))
                            })
                            .ok_or_else(|| {
                                anyhow!(
                                    "Variant {input_variant} does not exist in schema"
                                )
                            })?;

                        BorshSerialize::serialize(&(variant_index as u8), writer)?;
                        serialize_serde_json_by_declaration_with_schema(
                            writer,
                            variant_values.unwrap_or(&json!({})),
                            schema,
                            variant_declaration,
                        )?;
                        Ok(())
                    }

                    Definition::Struct { fields } => match fields {
                        Fields::NamedFields(fields) => {
                            let object = value.as_object().ok_or(ExpectationError::Object)?;
                            for (key, value_declaration) in fields {
                                let property_value = object
                                    .get(key.as_str())
                                    .ok_or_else(|| anyhow!("Expected property {key}"))?;
                                serialize_serde_json_by_declaration_with_schema(
                                    writer,
                                    property_value,
                                    schema,
                                    value_declaration,
                                )?;
                            }
                            Ok(())
                        }

                        Fields::UnnamedFields(fields) => {
                            if fields.len() == 1 {
                                serialize_serde_json_by_declaration_with_schema(
                                    writer, value, schema, &fields[0],
                                )?;
                                return Ok(());
                            }

                            let array = value.as_array().ok_or(ExpectationError::Array)?;
                            if array.len() != fields.len() {
                                return Err(
                                    ExpectationError::ArrayOfLength(fields.len() as u32).into()
                                );
                            }
                            for (declaration, value) in fields.iter().zip(array) {
                                serialize_serde_json_by_declaration_with_schema(
                                    writer,
                                    value,
                                    schema,
                                    declaration,
                                )?;
                            }
                            Ok(())
                        }

                        Fields::Empty => {
                            Ok(())
                        }
                    },
                }
            } else {
                debug!("Can't serialize unknown type: {:?}. Returning Ok(())", declaration);
                Ok(())
            }
        }
    }
}