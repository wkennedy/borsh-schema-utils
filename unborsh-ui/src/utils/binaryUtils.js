/**
 * Utility functions for binary data manipulation
 */

/**
 * Convert a Uint8Array to a Base64 string
 * @param {Uint8Array} bytes - The byte array to convert
 * @returns {string} Base64 encoded string
 */
export function toBase64(bytes) {
    let binary = '';
    const len = bytes.byteLength;
    for (let i = 0; i < len; i++) {
        binary += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binary);
}

/**
 * Convert a Base64 string to a Uint8Array
 * @param {string} base64 - The Base64 string to convert
 * @returns {Uint8Array} The decoded byte array
 */
export function fromBase64(base64) {
    const binary = window.atob(base64);
    const len = binary.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = binary.charCodeAt(i);
    }
    return bytes;
}

/**
 * Convert a Uint8Array to a hexadecimal string
 * @param {Uint8Array} bytes - The byte array to convert
 * @returns {string} Hex encoded string
 */
export function toHex(bytes) {
    return Array.from(bytes)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
}

/**
 * Convert a hexadecimal string to a Uint8Array
 * @param {string} hex - The hex string to convert
 * @returns {Uint8Array} The decoded byte array
 */
export function fromHex(hex) {
    // Remove 0x prefix if present
    const cleanHex = hex.startsWith('0x') ? hex.slice(2) : hex;

    const len = cleanHex.length / 2;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = parseInt(cleanHex.substr(i * 2, 2), 16);
    }
    return bytes;
}

/**
 * Format bytes for display in a hex viewer
 * @param {Uint8Array} bytes - The bytes to format
 * @returns {string} Formatted hex view
 */
export function formatHexView(bytes) {
    if (!bytes || bytes.length === 0) return 'Empty data';

    const chunks = [];

    // Display in 16-byte chunks with hex and ASCII
    for (let i = 0; i < bytes.length; i += 16) {
        const chunk = bytes.slice(i, i + 16);
        const hexPart = Array.from(chunk).map(b => b.toString(16).padStart(2, '0')).join(' ');
        const asciiPart = Array.from(chunk).map(b =>
            (b >= 32 && b <= 126) ? String.fromCharCode(b) : '.'
        ).join('');

        chunks.push(`${i.toString(16).padStart(8, '0')}  ${hexPart.padEnd(48, ' ')}  |${asciiPart}|`);
    }

    return chunks.join('\n');
}