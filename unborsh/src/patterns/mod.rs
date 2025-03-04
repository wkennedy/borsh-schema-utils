// src/patterns/mod.rs

//! Definitions of Borsh serialization patterns
//!
//! This module contains pattern definitions for common Borsh-serialized types.
//! These patterns are used by the analyzers to identify and interpret binary data.

pub mod blockchain;
pub mod collections;
pub mod complex;
pub mod primitives;

// Public exports
pub use blockchain::*;
pub use collections::*;
pub use complex::*;
pub use primitives::*;

use crate::core::PatternDictionary;

/// Create a dictionary with all default patterns
pub fn create_pattern_dictionary() -> PatternDictionary {
    let mut dict = PatternDictionary::new();

    // Add primitive types
    dict.add_pattern(primitives::create_u8_pattern());
    dict.add_pattern(primitives::create_u16_pattern());
    dict.add_pattern(primitives::create_u32_pattern());
    dict.add_pattern(primitives::create_u64_pattern());
    dict.add_pattern(primitives::create_i8_pattern());
    dict.add_pattern(primitives::create_i16_pattern());
    dict.add_pattern(primitives::create_i32_pattern());
    dict.add_pattern(primitives::create_i64_pattern());
    dict.add_pattern(primitives::create_f32_pattern());
    dict.add_pattern(primitives::create_f64_pattern());
    dict.add_pattern(primitives::create_bool_true_pattern());
    dict.add_pattern(primitives::create_bool_false_pattern());

    // Add collection types
    dict.add_pattern(collections::create_string_pattern());
    dict.add_pattern(collections::create_vec_pattern());
    dict.add_pattern(collections::create_option_some_pattern());
    dict.add_pattern(collections::create_option_none_pattern());
    dict.add_pattern(collections::create_result_ok_pattern());
    dict.add_pattern(collections::create_result_err_pattern());
    dict.add_pattern(collections::create_hashmap_pattern());
    dict.add_pattern(collections::create_hashset_pattern());
    dict.add_pattern(collections::create_btreemap_pattern());
    dict.add_pattern(collections::create_btreeset_pattern());
    dict.add_pattern(collections::create_array_pattern());
    dict.add_pattern(collections::create_tuple_pattern());

    // Add complex types
    dict.add_pattern(complex::create_enum_pattern());
    dict.add_pattern(complex::create_struct_pattern());
    dict.add_pattern(complex::create_nested_struct_pattern());
    dict.add_pattern(complex::create_optional_field_pattern());
    dict.add_pattern(complex::create_variant_field_pattern());
    dict.add_pattern(complex::create_map_entry_pattern());
    dict.add_pattern(complex::create_tagged_data_pattern());
    dict.add_pattern(complex::create_recursive_pattern());
    dict.add_pattern(complex::create_object_id_pattern());
    dict.add_pattern(complex::create_entity_pattern());
    dict.add_pattern(complex::create_nested_collection_pattern());
    dict.add_pattern(complex::create_message_pattern());
    dict.add_pattern(complex::create_versioned_data_pattern());

    // Add blockchain-specific patterns
    dict.add_pattern(blockchain::create_pubkey_pattern());
    dict.add_pattern(blockchain::create_signature_pattern());
    dict.add_pattern(blockchain::create_hash_pattern());
    dict.add_pattern(blockchain::create_token_amount_pattern());
    dict.add_pattern(blockchain::create_account_discriminator_pattern());
    dict.add_pattern(blockchain::create_instruction_discriminator_pattern());
    dict.add_pattern(blockchain::create_program_id_pattern());
    dict.add_pattern(blockchain::create_timestamp_pattern());
    dict.add_pattern(blockchain::create_transaction_id_pattern());
    dict.add_pattern(blockchain::create_sequence_number_pattern());
    dict.add_pattern(blockchain::create_address_pattern());
    dict.add_pattern(blockchain::create_consensus_data_pattern());
    dict.add_pattern(blockchain::create_event_data_pattern());
    dict.add_pattern(blockchain::create_nft_metadata_pattern());
    dict.add_pattern(blockchain::create_solana_account_pattern());

    dict
}

/// Create a subset dictionary with only primitive patterns
pub fn create_primitive_dictionary() -> PatternDictionary {
    let mut dict = PatternDictionary::new();

    dict.add_pattern(primitives::create_u8_pattern());
    dict.add_pattern(primitives::create_u16_pattern());
    dict.add_pattern(primitives::create_u32_pattern());
    dict.add_pattern(primitives::create_u64_pattern());
    dict.add_pattern(primitives::create_i8_pattern());
    dict.add_pattern(primitives::create_i16_pattern());
    dict.add_pattern(primitives::create_i32_pattern());
    dict.add_pattern(primitives::create_i64_pattern());
    dict.add_pattern(primitives::create_f32_pattern());
    dict.add_pattern(primitives::create_f64_pattern());
    dict.add_pattern(primitives::create_bool_true_pattern());
    dict.add_pattern(primitives::create_bool_false_pattern());

    dict
}

/// Create a subset dictionary with only collection patterns
pub fn create_collection_dictionary() -> PatternDictionary {
    let mut dict = PatternDictionary::new();

    dict.add_pattern(collections::create_string_pattern());
    dict.add_pattern(collections::create_vec_pattern());
    dict.add_pattern(collections::create_option_some_pattern());
    dict.add_pattern(collections::create_option_none_pattern());
    dict.add_pattern(collections::create_result_ok_pattern());
    dict.add_pattern(collections::create_result_err_pattern());
    dict.add_pattern(collections::create_hashmap_pattern());
    dict.add_pattern(collections::create_hashset_pattern());
    dict.add_pattern(collections::create_btreemap_pattern());
    dict.add_pattern(collections::create_btreeset_pattern());
    dict.add_pattern(collections::create_array_pattern());
    dict.add_pattern(collections::create_tuple_pattern());

    dict
}

/// Create a subset dictionary with only complex patterns
pub fn create_complex_dictionary() -> PatternDictionary {
    let mut dict = PatternDictionary::new();

    dict.add_pattern(complex::create_enum_pattern());
    dict.add_pattern(complex::create_struct_pattern());
    dict.add_pattern(complex::create_nested_struct_pattern());
    dict.add_pattern(complex::create_optional_field_pattern());
    dict.add_pattern(complex::create_variant_field_pattern());
    dict.add_pattern(complex::create_map_entry_pattern());
    dict.add_pattern(complex::create_tagged_data_pattern());
    dict.add_pattern(complex::create_recursive_pattern());
    dict.add_pattern(complex::create_object_id_pattern());
    dict.add_pattern(complex::create_entity_pattern());
    dict.add_pattern(complex::create_nested_collection_pattern());
    dict.add_pattern(complex::create_message_pattern());
    dict.add_pattern(complex::create_versioned_data_pattern());

    dict
}

/// Create a subset dictionary with only blockchain-specific patterns
pub fn create_blockchain_dictionary() -> PatternDictionary {
    let mut dict = PatternDictionary::new();

    dict.add_pattern(blockchain::create_pubkey_pattern());
    dict.add_pattern(blockchain::create_signature_pattern());
    dict.add_pattern(blockchain::create_hash_pattern());
    dict.add_pattern(blockchain::create_token_amount_pattern());
    dict.add_pattern(blockchain::create_account_discriminator_pattern());
    dict.add_pattern(blockchain::create_instruction_discriminator_pattern());
    dict.add_pattern(blockchain::create_program_id_pattern());
    dict.add_pattern(blockchain::create_timestamp_pattern());
    dict.add_pattern(blockchain::create_transaction_id_pattern());
    dict.add_pattern(blockchain::create_sequence_number_pattern());
    dict.add_pattern(blockchain::create_address_pattern());
    dict.add_pattern(blockchain::create_consensus_data_pattern());
    dict.add_pattern(blockchain::create_event_data_pattern());
    dict.add_pattern(blockchain::create_nft_metadata_pattern());
    dict.add_pattern(blockchain::create_solana_account_pattern());

    dict
}

/// Create a custom dictionary by combining pattern types
pub fn create_custom_dictionary(
    include_primitives: bool,
    include_collections: bool,
    include_complex: bool,
    include_blockchain: bool,
) -> PatternDictionary {
    let mut dict = PatternDictionary::new();

    if include_primitives {
        let primitives = create_primitive_dictionary();
        for pattern in primitives.get_patterns() {
            dict.add_pattern(pattern.clone());
        }
    }

    if include_collections {
        let collections = create_collection_dictionary();
        for pattern in collections.get_patterns() {
            dict.add_pattern(pattern.clone());
        }
    }

    if include_complex {
        let complex = create_complex_dictionary();
        for pattern in complex.get_patterns() {
            dict.add_pattern(pattern.clone());
        }
    }

    if include_blockchain {
        let blockchain = create_blockchain_dictionary();
        for pattern in blockchain.get_patterns() {
            dict.add_pattern(pattern.clone());
        }
    }

    dict
}
