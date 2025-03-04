use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Uint8Array, Error as JsError};
use serde::{Serialize, Deserialize};
use unborsh::{
    analyze,
    analyze_with_strategy,
    analyze_with_options,
    AnalysisStrategy,
    AnalysisOptions,
    AnalysisResult,
    PatternMatch,
};

// Initialize panic hook for better error reporting in browser
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Converts a JavaScript Uint8Array to a Rust Vec<u8>
fn uint8array_to_vec(array: &Uint8Array) -> Vec<u8> {
    let mut result = vec![0; array.length() as usize];
    array.copy_to(&mut result);
    result
}

/// Options for the Borsh analysis
#[wasm_bindgen]
pub struct JsAnalysisOptions {
    strategy: u8,
    max_depth: u32,
    min_confidence: u8,
    include_raw_bytes: bool,
    max_matches: Option<u32>,
}

#[wasm_bindgen]
impl JsAnalysisOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            strategy: 3, // Default: Comprehensive (0=Pattern, 1=Recursive, 2=Probabilistic, 3=Comprehensive)
            max_depth: 5,
            min_confidence: 30,
            include_raw_bytes: false,
            max_matches: None,
        }
    }

    /// Set the analysis strategy
    /// 0: Pattern, 1: Recursive, 2: Probabilistic, 3: Comprehensive
    #[wasm_bindgen]
    pub fn strategy(mut self, strategy: u8) -> Self {
        if strategy <= 3 {
            self.strategy = strategy;
        }
        self
    }

    #[wasm_bindgen]
    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }

    #[wasm_bindgen]
    pub fn min_confidence(mut self, confidence: u8) -> Self {
        self.min_confidence = confidence;
        self
    }

    #[wasm_bindgen]
    pub fn include_raw_bytes(mut self, include: bool) -> Self {
        self.include_raw_bytes = include;
        self
    }

    #[wasm_bindgen]
    pub fn max_matches(mut self, max: Option<u32>) -> Self {
        self.max_matches = max;
        self
    }

    fn to_rust_options(&self) -> AnalysisOptions {
        AnalysisOptions {
            strategy: match self.strategy {
                0 => AnalysisStrategy::Pattern,
                1 => AnalysisStrategy::Recursive,
                2 => AnalysisStrategy::Probabilistic,
                _ => AnalysisStrategy::Comprehensive,
            },
            max_depth: self.max_depth as usize,
            min_confidence: self.min_confidence,
            include_raw_bytes: self.include_raw_bytes,
            max_matches: self.max_matches.map(|m| m as usize),
        }
    }
}

/// JavaScript-friendly representation of a pattern match
#[derive(Serialize, Deserialize)]
struct JsPatternMatch {
    pattern_name: String,
    offset: usize,
    length: usize,
    data: Vec<u8>,
    interpretation: String,
    confidence: u8,
}

impl From<PatternMatch> for JsPatternMatch {
    fn from(m: PatternMatch) -> Self {
        Self {
            pattern_name: m.pattern_name,
            offset: m.offset,
            length: m.length,
            data: m.data,
            interpretation: m.interpretation,
            confidence: m.confidence,
        }
    }
}

/// JavaScript-friendly representation of analysis results
#[derive(Serialize, Deserialize)]
struct JsAnalysisResult {
    matches: Vec<JsPatternMatch>,
    structure_hypothesis: Option<String>,
    confidence: u8,
    description: String,
}

impl From<AnalysisResult> for JsAnalysisResult {
    fn from(r: AnalysisResult) -> Self {
        Self {
            matches: r.matches.into_iter().map(JsPatternMatch::from).collect(),
            structure_hypothesis: r.structure_hypothesis,
            confidence: r.confidence,
            description: r.description,
        }
    }
}

/// Analyze Borsh-serialized data with default options (Comprehensive strategy)
#[wasm_bindgen]
pub fn analyze_borsh(data: &Uint8Array) -> Result<JsValue, JsError> {
    let data_vec = uint8array_to_vec(data);
    let result = analyze(&data_vec);
    let js_result = JsAnalysisResult::from(result);

    match serde_json::to_string(&js_result) {
        Ok(json_str) => Ok(JsValue::from_str(&json_str)),
        Err(e) => Err(JsError::new(&format!("Failed to serialize result: {}", e)).into())
    }
}

/// Analyze Borsh-serialized data with a specific strategy
#[wasm_bindgen]
pub fn analyze_borsh_with_strategy(data: &Uint8Array, strategy: u8) -> Result<JsValue, JsError> {
    let data_vec = uint8array_to_vec(data);

    let strategy_enum = match strategy {
        0 => AnalysisStrategy::Pattern,
        1 => AnalysisStrategy::Recursive,
        2 => AnalysisStrategy::Probabilistic,
        _ => AnalysisStrategy::Comprehensive,
    };

    let result = analyze_with_strategy(&data_vec, strategy_enum);
    let js_result = JsAnalysisResult::from(result);

    match serde_json::to_string(&js_result) {
        Ok(json_str) => Ok(JsValue::from_str(&json_str)),
        Err(e) => Err(JsError::new(&format!("Failed to serialize result: {}", e)).into())
    }
}

/// Analyze Borsh-serialized data with custom options
#[wasm_bindgen]
pub fn analyze_borsh_with_options(data: &Uint8Array, strategy: u8, max_depth: u32, min_confidence: u8, include_raw_bytes: bool, max_matches: Option<u32>) -> Result<JsValue, JsError> {
    let data_vec = uint8array_to_vec(data);

    let rust_options = AnalysisOptions {
        strategy: match strategy {
            0 => AnalysisStrategy::Pattern,
            1 => AnalysisStrategy::Recursive,
            2 => AnalysisStrategy::Probabilistic,
            _ => AnalysisStrategy::Comprehensive,
        },
        max_depth: max_depth as usize,
        min_confidence,
        include_raw_bytes,
        max_matches: max_matches.map(|m| m as usize),
    };

    let result = analyze_with_options(&data_vec, rust_options);
    let js_result = JsAnalysisResult::from(result);

    match serde_json::to_string(&js_result) {
        Ok(json_str) => Ok(JsValue::from_str(&json_str)),
        Err(e) => Err(JsError::new(&format!("Failed to serialize result: {}", e)).into())
    }
}

/// Analyze base64-encoded Borsh data
#[wasm_bindgen]
pub fn analyze_borsh_base64(base64_str: &str) -> Result<JsValue, JsError> {
    let data_vec = match base64::decode(base64_str) {
        Ok(data) => data,
        Err(e) => return Err(JsError::new(&format!("Failed to decode base64: {}", e)).into())
    };

    let result = analyze(&data_vec);
    let js_result = JsAnalysisResult::from(result);

    match serde_json::to_string(&js_result) {
        Ok(json_str) => Ok(JsValue::from_str(&json_str)),
        Err(e) => Err(JsError::new(&format!("Failed to serialize result: {}", e)).into())
    }
}

/// Analyze hex-encoded Borsh data
#[wasm_bindgen]
pub fn analyze_borsh_hex(hex_str: &str) -> Result<JsValue, JsError> {
    let data_vec = match hex::decode(hex_str) {
        Ok(data) => data,
        Err(e) => return Err(JsError::new(&format!("Failed to decode hex: {}", e)).into())
    };

    let result = analyze(&data_vec);
    let js_result = JsAnalysisResult::from(result);

    match serde_json::to_string(&js_result) {
        Ok(json_str) => Ok(JsValue::from_str(&json_str)),
        Err(e) => Err(JsError::new(&format!("Failed to serialize result: {}", e)).into())
    }
}

/// Extract a structure hypothesis about the data
#[wasm_bindgen]
pub fn extract_structure_hypothesis(data: &Uint8Array) -> Result<String, JsError> {
    let data_vec = uint8array_to_vec(data);
    match unborsh::extract_structure_hypothesis(&data_vec) {
        Some(hypothesis) => Ok(hypothesis),
        None => Err(JsError::new("Could not generate a structure hypothesis").into())
    }
}

/// Try to interpret the data as a specific type
#[wasm_bindgen]
pub fn interpret_as(data: &Uint8Array, type_name: &str) -> Result<String, JsError> {
    let data_vec = uint8array_to_vec(data);
    match unborsh::interpret_as(&data_vec, type_name) {
        Some(interpretation) => Ok(interpretation),
        None => Err(JsError::new(&format!("Failed to interpret data as {}", type_name)).into())
    }
}

/// Get all available pattern names
#[wasm_bindgen]
pub fn get_available_patterns() -> Result<Array, JsError> {
    let dictionary = unborsh::pattern_definitions::create_pattern_dictionary();
    let patterns = dictionary.get_patterns();

    let result = Array::new();
    for (i, pattern) in patterns.iter().enumerate() {
        result.set(i as u32, JsValue::from_str(&pattern.name));
    }

    Ok(result)
}

/// Get pattern details by name
#[wasm_bindgen]
pub fn get_pattern_details(pattern_name: &str) -> Result<JsValue, JsError> {
    let dictionary = unborsh::pattern_definitions::create_pattern_dictionary();

    if let Some(pattern) = dictionary.get_pattern_by_name(pattern_name) {
        let details = Object::new();

        js_sys::Reflect::set(&details, &JsValue::from_str("name"), &JsValue::from_str(&pattern.name)).unwrap();
        js_sys::Reflect::set(&details, &JsValue::from_str("description"), &JsValue::from_str(&pattern.description)).unwrap();
        js_sys::Reflect::set(&details, &JsValue::from_str("example"), &JsValue::from_str(&pattern.example)).unwrap();

        Ok(details.into())
    } else {
        Err(JsError::new(&format!("Pattern '{}' not found", pattern_name)).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_analyze_borsh() {
        // A simple String "Hello World" in Borsh format
        let data = vec![11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
        let uint8_array = Uint8Array::new_with_length(data.len() as u32);
        uint8_array.copy_from(&data);

        let result = analyze_borsh(&uint8_array).unwrap();
        assert!(result.is_string());

        let result_str = result.as_string().unwrap();
        assert!(result_str.contains("String"));
        assert!(result_str.contains("Hello World"));
    }
}