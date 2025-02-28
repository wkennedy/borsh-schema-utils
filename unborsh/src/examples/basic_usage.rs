//examples/basic_usage.rs

use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize as BorshSerializeDerive, BorshSerialize as BorshDeserializeDerive};
use hex;
use unborsh::{analyze, analyze_with_options, analyze_with_strategy, AnalysisOptions, AnalysisStrategy};

// Example data structures for testing
#[derive(BorshSerializeDerive, BorshDeserializeDerive, Debug)]
struct Person {
    name: String,
    age: u32,
    is_verified: bool,
}

#[derive(BorshSerializeDerive, BorshDeserializeDerive, Debug)]
enum TokenData {
    Cryptocurrency {
        name: String,
        balance: u64,
        value: u64,
    },
    Stablecoin {
        name: String,
        balance: u64,
        pegged_value: u64,
    },
}

fn main() {
    println!("Unborsh - Basic Analysis Example");
    println!("==================================\n");

    // Create sample data
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        is_verified: true,
    };

    let token_btc = TokenData::Cryptocurrency {
        name: "Token: BTC".to_string(),
        balance: 149,
        value: 390000,
    };

    let token_usdt = TokenData::Stablecoin {
        name: "Token: USDT".to_string(),
        balance: 10000,
        pegged_value: 1000,
    };

    // Serialize to Borsh
    let mut person_borsh: Vec<u8> = Vec::new();
    Person::serialize(&person, &mut person_borsh).expect("Failed to serialize Person");
    let mut token_btc_borsh: Vec<u8> = Vec::new();
    TokenData::serialize(&token_btc, &mut token_btc_borsh).expect("Failed to serialize Person");
    let mut token_usdt_borsh: Vec<u8> = Vec::new();
    TokenData::serialize(&token_usdt, &mut token_usdt_borsh).expect("Failed to serialize Person");

    // Print hex representation of the serialized data
    println!("Person (Borsh hex): {}", hex::encode(&person_borsh));
    println!("TokenData::Cryptocurrency (Borsh hex): {}", hex::encode(&token_btc_borsh));
    println!("TokenData::Stablecoin (Borsh hex): {}", hex::encode(&token_usdt_borsh));
    println!();

    // Analyze person data using default options (Comprehensive strategy)
    println!("Analyzing Person data with default options (Comprehensive strategy):");
    let result = analyze(&person_borsh);
    println!("{}", result);
    println!();

    // Analyze token_btc data using Pattern strategy
    println!("Analyzing TokenData::Cryptocurrency with Pattern strategy:");
    let result = analyze_with_strategy(&token_btc_borsh, AnalysisStrategy::Pattern);
    println!("{}", result);
    println!();

    // Analyze token_usdt data using Recursive strategy
    println!("Analyzing TokenData::Stablecoin with Recursive strategy:");
    let result = analyze_with_strategy(&token_usdt_borsh, AnalysisStrategy::Recursive);
    println!("{}", result);
    println!();

    // Analyze token_usdt data using Probabilistic strategy
    println!("Analyzing TokenData::Stablecoin with Probabilistic strategy:");
    let result = analyze_with_strategy(&token_usdt_borsh, AnalysisStrategy::Probabilistic);
    println!("{}", result);
    println!();

    // Analyze using custom options
    println!("Analyzing Person data with custom options:");
    let options = AnalysisOptions {
        strategy: AnalysisStrategy::Comprehensive,
        max_depth: 3,
        min_confidence: 50,
        include_raw_bytes: true,
        max_matches: Some(10),
    };
    let result = analyze_with_options(&person_borsh, options);
    println!("{}", result);
    println!();

    // Analyze unknown data received as hex string
    let unknown_hex = "0b000000546f6b656e3a204254431027000000000000e8030000000000000000";
    println!("Analyzing unknown data from hex string: {}", unknown_hex);
    if let Ok(unknown_data) = hex::decode(unknown_hex) {
        let result = analyze(&unknown_data);
        println!("{}", result);
    } else {
        println!("Invalid hex string");
    }

    //     // Some Borsh-serialized data
    //     let data = vec![11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
    // 
    //     // Analyze the data
    //     let result = analyze(&data);
    // 
    //     // Create visualization options
    //     let viz_options = VisualizationOptions {
    //         max_depth: 5,
    //         show_interpretations: true,
    //         show_confidence: true,
    //         show_raw_bytes: true,
    //         max_bytes_per_match: 32,
    //         theme: ColorTheme::Dark,
    //     };
    // 
    //     // Generate ASCII visualization for terminal display
    //     let ascii_viz = visualize(&result, &data, VisualizationFormat::Ascii, &viz_options);
    //     println!("{}", ascii_viz);
    // 
    //     // Generate HTML visualization and save to file
    //     unborsh::visualize_to_file(
    //         &result,
    //         &data,
    //         VisualizationFormat::Html,
    //         "visualization.html",
    //         &viz_options,
    //     ).expect("Failed to save visualization");
}