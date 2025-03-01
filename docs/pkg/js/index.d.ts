export enum AnalysisStrategy {
    PATTERN = 0,
    RECURSIVE = 1,
    PROBABILISTIC = 2,
    COMPREHENSIVE = 3
}

export interface PatternMatch {
    pattern_name: string;
    offset: number;
    length: number;
    data: number[];
    interpretation: string;
    confidence: number;
}

export interface AnalysisResult {
    matches: PatternMatch[];
    structure_hypothesis: string | null;
    confidence: number;
    description: string;
}

export interface AnalysisOptions {
    strategy?: number;
    maxDepth?: number;
    minConfidence?: number;
    includeRawBytes?: boolean;
    maxMatches?: number;
}

export interface PatternDetails {
    name: string;
    description: string;
    example: string;
}

export class Unborsh {
    /**
     * Analyze Borsh-serialized data with default options (Comprehensive strategy)
     * @param data - The Borsh serialized data to analyze
     * @returns The analysis results
     */
    static analyze(data: Uint8Array): AnalysisResult;

    /**
     * Analyze Borsh-serialized data with a specific strategy
     * @param data - The Borsh serialized data to analyze
     * @param strategy - The analysis strategy to use
     * @returns The analysis results
     */
    static analyzeWithStrategy(data: Uint8Array, strategy: AnalysisStrategy): AnalysisResult;

    /**
     * Analyze Borsh-serialized data with custom options
     * @param data - The Borsh serialized data to analyze
     * @param options - Analysis options object
     * @returns The analysis results
     */
    static analyzeWithOptions(data: Uint8Array, options: AnalysisOptions): AnalysisResult;

    /**
     * Analyze base64-encoded Borsh data
     * @param base64 - Base64-encoded Borsh data
     * @returns The analysis results
     */
    static analyzeBase64(base64: string): AnalysisResult;

    /**
     * Analyze hex-encoded Borsh data
     * @param hex - Hex-encoded Borsh data
     * @returns The analysis results
     */
    static analyzeHex(hex: string): AnalysisResult;

    /**
     * Extract a structure hypothesis about the data
     * @param data - The Borsh serialized data
     * @returns Structure hypothesis as Rust-like code
     */
    static extractStructureHypothesis(data: Uint8Array): string;

    /**
     * Try to interpret the data as a specific type
     * @param data - The Borsh serialized data
     * @param typeName - Name of the type to interpret as
     * @returns Interpretation of the data
     */
    static interpretAs(data: Uint8Array, typeName: string): string;

    /**
     * Get all available pattern names
     * @returns List of pattern names
     */
    static getAvailablePatterns(): string[];

    /**
     * Get pattern details by name
     * @param patternName - Name of the pattern
     * @returns Pattern details
     */
    static getPatternDetails(patternName: string): PatternDetails;
}

export class BinaryUtils {
    /**
     * Convert a Uint8Array to a Base64 string
     * @param bytes - The byte array to convert
     * @returns Base64 encoded string
     */
    static toBase64(bytes: Uint8Array): string;

    /**
     * Convert a Base64 string to a Uint8Array
     * @param base64 - The Base64 string to convert
     * @returns The decoded byte array
     */
    static fromBase64(base64: string): Uint8Array;

    /**
     * Convert a Uint8Array to a hexadecimal string
     * @param bytes - The byte array to convert
     * @returns Hex encoded string
     */
    static toHex(bytes: Uint8Array): string;

    /**
     * Convert a hexadecimal string to a Uint8Array
     * @param hex - The hex string to convert
     * @returns The decoded byte array
     */
    static fromHex(hex: string): Uint8Array;
}

/**
 * Create a simple Borsh-encoded string "Hello World" for testing
 * @returns A Uint8Array containing Borsh-encoded data
 */
export function createSampleData(): Uint8Array;

/**
 * Create a more complex sample with multiple fields for testing
 * @returns A Uint8Array containing Borsh-encoded data
 */
export function createComplexSample(): Uint8Array;