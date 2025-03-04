// src/patterns/blockchain.rs

//! Pattern definitions for blockchain-specific Borsh types

use crate::core::BorshPattern;

/// Create a pattern for public keys (e.g., Ed25519)
pub fn create_pubkey_pattern() -> BorshPattern {
    BorshPattern::new(
        "PublicKey",
        "32-byte public key (often Ed25519)",
        vec![0x00; 32], // Placeholder for 32 bytes
        vec![0x00; 32], // All variable
        "Public key bytes",
        |bytes| {
            if bytes.len() >= 32 {
                // Check for high entropy (typical of keys)
                let unique_bytes = bytes[..32]
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                if unique_bytes > 20 {
                    Some(format!(
                        "Probable public key: {}",
                        hex::encode(&bytes[..32])
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for signatures (e.g., Ed25519)
pub fn create_signature_pattern() -> BorshPattern {
    BorshPattern::new(
        "Signature",
        "64-byte cryptographic signature (often Ed25519)",
        vec![0x00; 64], // Placeholder for 64 bytes
        vec![0x00; 64], // All variable
        "Signature bytes",
        |bytes| {
            if bytes.len() >= 64 {
                // Check for high entropy (typical of signatures)
                let unique_bytes = bytes[..64]
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                if unique_bytes > 40 {
                    Some(format!("Probable signature: {}", hex::encode(&bytes[..16])))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for hashes (e.g., SHA-256)
pub fn create_hash_pattern() -> BorshPattern {
    BorshPattern::new(
        "Hash",
        "32-byte cryptographic hash (often SHA-256)",
        vec![0x00; 32], // Placeholder for 32 bytes
        vec![0x00; 32], // All variable
        "Hash bytes",
        |bytes| {
            if bytes.len() >= 32 {
                // Check for high entropy (typical of hashes)
                let unique_bytes = bytes[..32]
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                if unique_bytes > 20 {
                    Some(format!("Probable hash: {}", hex::encode(&bytes[..32])))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for token amounts (u64 values representing tokens)
pub fn create_token_amount_pattern() -> BorshPattern {
    BorshPattern::new(
        "TokenAmount",
        "8-byte token amount (usually in smallest units like lamports)",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "Token amount in smallest units",
        |bytes| {
            if bytes.len() >= 8 {
                let amount = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                // Common range for token amounts (not too small, not too large)
                if amount > 1_000 && amount < 1_000_000_000_000_000 {
                    Some(format!("Possible token amount: {}", amount))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for account discriminators
pub fn create_account_discriminator_pattern() -> BorshPattern {
    BorshPattern::new(
        "AccountDiscriminator",
        "8-byte account type identifier (common in Anchor)",
        vec![0x00; 8], // Placeholder for 8 bytes
        vec![0x00; 8], // All variable
        "Account discriminator",
        |bytes| {
            if bytes.len() >= 8 {
                Some(format!(
                    "Possible account discriminator: {}",
                    hex::encode(&bytes[..8])
                ))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for instruction discriminators
pub fn create_instruction_discriminator_pattern() -> BorshPattern {
    BorshPattern::new(
        "InstructionDiscriminator",
        "8-byte instruction identifier (common in Anchor)",
        vec![0x00; 8], // Placeholder for 8 bytes
        vec![0x00; 8], // All variable
        "Instruction discriminator",
        |bytes| {
            if bytes.len() >= 8 {
                Some(format!(
                    "Possible instruction discriminator: {}",
                    hex::encode(&bytes[..8])
                ))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for program IDs
pub fn create_program_id_pattern() -> BorshPattern {
    BorshPattern::new(
        "ProgramID",
        "32-byte program identifier",
        vec![0x00; 32], // Placeholder for 32 bytes
        vec![0x00; 32], // All variable
        "Program ID",
        |bytes| {
            if bytes.len() >= 32 {
                Some(format!(
                    "Possible program ID: {}",
                    hex::encode(&bytes[..32])
                ))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for blockchain timestamps
pub fn create_timestamp_pattern() -> BorshPattern {
    BorshPattern::new(
        "Timestamp",
        "8-byte Unix timestamp (seconds or milliseconds since epoch)",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "Unix timestamp",
        |bytes| {
            if bytes.len() >= 8 {
                let value = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);

                // Check if it's in a reasonable range for Unix timestamps
                // 1600000000 = September 2020, 4000000000 = 2096
                if value > 1_600_000_000 && value < 4_000_000_000 {
                    let dt = chrono::DateTime::from_timestamp(value as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "invalid date".to_string());
                    return Some(format!("Unix timestamp: {} ({})", value, dt));
                }

                // Check if it's milliseconds
                if value > 1_600_000_000_000 && value < 4_000_000_000_000 {
                    let dt = chrono::DateTime::from_timestamp_millis(value as i64)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "invalid date".to_string());
                    return Some(format!("Unix timestamp (ms): {} ({})", value, dt));
                }
            }
            None
        },
    )
}

/// Create a pattern for blockchain transaction IDs
pub fn create_transaction_id_pattern() -> BorshPattern {
    BorshPattern::new(
        "TransactionID",
        "32-byte transaction identifier",
        vec![0x00; 32], // Placeholder for 32 bytes
        vec![0x00; 32], // All variable
        "Transaction ID",
        |bytes| {
            if bytes.len() >= 32 {
                // Check for high entropy (typical of transaction IDs)
                let unique_bytes = bytes[..32]
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                if unique_bytes > 20 {
                    Some(format!(
                        "Probable transaction ID: {}",
                        hex::encode(&bytes[..32])
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for blockchain sequence numbers
pub fn create_sequence_number_pattern() -> BorshPattern {
    BorshPattern::new(
        "SequenceNumber",
        "8-byte sequence number or nonce",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "Sequence number or nonce",
        |bytes| {
            if bytes.len() >= 8 {
                let value = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                Some(format!("Possible sequence number: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for blockchain address (human-readable format)
pub fn create_address_pattern() -> BorshPattern {
    BorshPattern::new(
        "Address",
        "Blockchain address (often base58 or bech32 encoded)",
        vec![0x00; 32], // Placeholder for a typical address
        vec![0x00; 32], // All variable
        "Blockchain address",
        |bytes| {
            if bytes.len() >= 20 && bytes.len() <= 32 {
                // Check for high entropy but not too high (some structure)
                let unique_bytes = bytes.iter().collect::<std::collections::HashSet<_>>().len();
                if unique_bytes > bytes.len() / 2 && unique_bytes < bytes.len() * 9 / 10 {
                    Some(format!(
                        "Possible blockchain address: {}",
                        hex::encode(bytes)
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for blockchain consensus data (votes, etc.)
pub fn create_consensus_data_pattern() -> BorshPattern {
    BorshPattern::new(
        "ConsensusData",
        "Blockchain consensus-related data",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        "Consensus data",
        |bytes| {
            if bytes.len() >= 16 {
                // Look for timestamp-like value followed by counter and flags
                let potential_timestamp = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);

                let counter = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

                if potential_timestamp > 1_600_000_000
                    && potential_timestamp < 3_000_000_000
                    && counter < 10000
                {
                    return Some(format!(
                        "Possible consensus data (timestamp: {}, counter: {})",
                        potential_timestamp, counter
                    ));
                }
            }
            None
        },
    )
}

/// Create a pattern for smart contract event data
pub fn create_event_data_pattern() -> BorshPattern {
    BorshPattern::new(
        "EventData",
        "Smart contract event data",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0xFF, 0xFF, 0x00, 0x00], // First bytes often event type
        "Event data",
        |bytes| {
            if bytes.len() >= 6 {
                let event_type = u16::from_le_bytes([bytes[0], bytes[1]]);
                let data_len = u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);

                if data_len as usize + 6 <= bytes.len() && data_len < 10000 {
                    return Some(format!(
                        "Possible event data (type: {}, data: {} bytes)",
                        event_type, data_len
                    ));
                }
            }
            None
        },
    )
}

/// Create a pattern for NFT metadata
pub fn create_nft_metadata_pattern() -> BorshPattern {
    BorshPattern::new(
        "NFTMetadata",
        "NFT token metadata",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00],
        "NFT metadata",
        |bytes| {
            if bytes.len() >= 8 {
                // Look for typical string patterns in NFT metadata
                if let Some(str_len) = try_extract_string_length(&bytes[0..]) {
                    if str_len > 2 && str_len < 100 && bytes.len() >= 4 + str_len as usize {
                        // Try to extract what might be a name field
                        if let Ok(str_content) =
                            std::str::from_utf8(&bytes[4..4 + str_len as usize])
                        {
                            return Some(format!(
                                "Possible NFT metadata with name: \"{}\"",
                                str_content
                            ));
                        }
                    }
                }
            }
            None
        },
    )
}

// Helper function to try extracting a string length
fn try_extract_string_length(bytes: &[u8]) -> Option<u32> {
    if bytes.len() >= 4 {
        let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if len < 10000 {
            Some(len)
        } else {
            None
        }
    } else {
        None
    }
}

/// Create a pattern for Solana-specific account data
pub fn create_solana_account_pattern() -> BorshPattern {
    BorshPattern::new(
        "SolanaAccount",
        "Solana account data structure",
        vec![0x00; 8], // First 8 bytes often discriminator
        vec![0x00; 8], // All variable
        "Solana account data",
        |bytes| {
            if bytes.len() >= 8 {
                // Check if the first 8 bytes could be a discriminator
                let discrim = &bytes[0..8];

                // Check if it's followed by typical account data patterns
                if bytes.len() > 40 {
                    // Look for authority (pubkey) after discriminator
                    let potential_pubkey = &bytes[8..40];

                    let unique_bytes = potential_pubkey
                        .iter()
                        .collect::<std::collections::HashSet<_>>()
                        .len();

                    if unique_bytes > 20 {
                        return Some(format!(
                            "Possible Solana account (discriminator: {}, authority: {})",
                            hex::encode(discrim),
                            hex::encode(&potential_pubkey[..])
                        ));
                    }
                }

                Some(format!(
                    "Possible Solana account (discriminator: {})",
                    hex::encode(discrim)
                ))
            } else {
                None
            }
        },
    )
}
