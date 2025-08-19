// Utility for loading the unborsh-wasm module

// Cache for the loaded module
let wasmModule = null;
let isLoading = false;
let loadPromise = null;

/**
 * Load the WASM module asynchronously (with caching)
 * @returns {Promise<Object>} The loaded WASM module
 */
export async function loadWasmModule() {
    // Return cached module if available
    if (wasmModule) {
        console.log('Returning cached WASM module');
        return wasmModule;
    }

    // If loading is already in progress, return the existing promise
    if (isLoading) {
        console.log('WASM module loading already in progress, returning promise');
        return loadPromise;
    }

    console.log('Starting WASM module load');
    // Start loading
    isLoading = true;
    loadPromise = (async () => {
        try {
            // Load both scripts directly
            // First load the wasm file directly as a blob
            console.log('Fetching WASM file');
            const wasmResponse = await fetch('/wasm/unborsh_wasm_bg.wasm');
            if (!wasmResponse.ok) {
                throw new Error(`Failed to fetch WASM file: ${wasmResponse.status} ${wasmResponse.statusText}`);
            }
            const wasmBytes = await wasmResponse.arrayBuffer();
            console.log('WASM file fetched, size:', wasmBytes.byteLength);

            // Now load the JS file
            console.log('Loading JS module script');
            // Create a script element to load the JS file
            const scriptPromise = new Promise((resolve, reject) => {
                const script = document.createElement('script');
                script.src = '/wasm/unborsh_wasm.js';
                script.type = 'text/javascript';
                script.onload = () => {
                    console.log('JS script loaded');
                    resolve();
                };
                script.onerror = (err) => {
                    console.error('Failed to load JS script:', err);
                    reject(new Error('Failed to load JS script'));
                };
                document.head.appendChild(script);
            });

            await scriptPromise;
            console.log('JS module loaded successfully');

            // At this point the global wasm_bindgen should be available
            if (!window.wasm_bindgen) {
                throw new Error('wasm_bindgen not found in global scope');
            }

            // Initialize the WASM module with the bytes we loaded
            console.log('Initializing wasm_bindgen with WASM bytes');
            const initResult = await window.wasm_bindgen(wasmBytes);
            console.log('WASM initialization complete', initResult);

            // Get the exports from the initialized module
            wasmModule = {
                Unborsh: window.Unborsh,
                AnalysisStrategy: window.AnalysisStrategy,
                BinaryUtils: window.BinaryUtils,
                createSampleData: window.createSampleData,
                createComplexSample: window.createComplexSample
            };

            // Log available exports for debugging
            console.log('WASM module loaded successfully with exports:', Object.keys(wasmModule));
            console.log('Window exports:', Object.keys(window).filter(k => !k.startsWith('_')).slice(0, 20));

            return wasmModule;
        } catch (error) {
            console.error('Failed to load WASM module:', error);
            throw error;
        } finally {
            isLoading = false;
        }
    })();

    return loadPromise;
}

/**
 * Get key components from the WASM module
 * @returns {Promise<{Unborsh, AnalysisStrategy, BinaryUtils}>}
 */
export async function getWasmComponents() {
    try {
        const module = await loadWasmModule();
        return {
            Unborsh: module.Unborsh,
            AnalysisStrategy: module.AnalysisStrategy,
            BinaryUtils: module.BinaryUtils,
            createSampleData: module.createSampleData,
            createComplexSample: module.createComplexSample
        };
    } catch (error) {
        console.error('Failed to get WASM components:', error);
        throw error;
    }
}