<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Input Data</h2>

    <TabContainer :tabs="tabs" initialTab="binary">
      <!-- Binary sample tab -->
      <template #binary>
        <div class="space-y-4">
          <div class="flex flex-wrap gap-2">
            <button @click="loadSimpleSample" class="btn btn-sm">Simple String</button>
            <button @click="loadComplexSample" class="btn btn-sm">Complex Object</button>
            <button @click="loadCustomSample" class="btn btn-sm">Custom Structure</button>
          </div>

          <BinaryPreview :data="currentData" />
        </div>
      </template>

      <!-- Hex input tab -->
      <template #hex>
        <div class="space-y-4">
          <textarea
              v-model="hexInput"
              class="form-input font-mono text-sm"
              rows="5"
              placeholder="Enter hex data (e.g., 0b00000048656c6c6f20576f726c64)"
              @input="console.log('Hex input changed')"
          ></textarea>
          <button @click="analyzeHex" class="btn">Analyze Hex Data</button>
        </div>
      </template>

      <!-- Base64 input tab -->
      <template #base64>
        <div class="space-y-4">
          <textarea
              v-model="base64Input"
              class="form-input font-mono text-sm"
              rows="5"
              placeholder="Enter base64 data (e.g., CwAAAEhlbGxvIFdvcmxk)"
              @input="console.log('Base64 input changed')"
          ></textarea>
          <button @click="analyzeBase64" class="btn">Analyze Base64 Data</button>
        </div>
      </template>
    </TabContainer>
  </div>
</template>

<script>
import { ref, watch, onMounted } from 'vue';
import TabContainer from './TabContainer.vue';
import BinaryPreview from './BinaryPreview.vue';
import { toHex, toBase64, fromHex, fromBase64 } from '../utils/binaryUtils';
import { createSimpleSample, createComplexSample, createCustomSample } from '../utils/sampleData';
// import { getWasmComponents } from '../utils/wasmLoader';

// We'll dynamically load these from the WASM module
let Unborsh;

export default {
  components: {
    TabContainer,
    BinaryPreview
  },
  props: {
    currentData: {
      type: Object, // Uint8Array
      required: false,
      default: null
    }
  },
  emits: ['data-updated', 'analyze-default'],
  setup(props, { emit }) {
    const tabs = [
      { id: 'binary', label: 'Binary Sample' },
      { id: 'hex', label: 'Hex Input' },
      { id: 'base64', label: 'Base64 Input' }
    ];

    const hexInput = ref('');
    const base64Input = ref('');

    // Set initial values when currentData changes
    watch(() => props.currentData, (newData) => {
      if (newData) {
        hexInput.value = toHex(newData);
        base64Input.value = toBase64(newData);
      }
    }, { immediate: true });

    // Load Unborsh from WASM module on component setup
    onMounted(async () => {
      try {
        // const components = await getWasmComponents();
        // Unborsh = components.Unborsh;
        const module = await import('/wasm/js/index.js?url');
        Unborsh = module.Unborsh;
      } catch (err) {
        console.error('Failed to load WASM components:', err);
      }
    });

    // Load and analyze sample data
    const loadSimpleSample = () => {
      console.log('Loading simple sample');
      const data = createSimpleSample();
      updateData(data);
      emit('analyze-default');
    };

    const loadComplexSample = () => {
      console.log('Loading complex sample');
      const data = createComplexSample();
      updateData(data);
      emit('analyze-default');
    };

    const loadCustomSample = () => {
      console.log('Loading custom sample');
      const data = createCustomSample();
      updateData(data);
      emit('analyze-default');
    };

    // Update the current data and notify parent
    const updateData = (data) => {
      console.log('Updating data', data);
      emit('data-updated', data);
      hexInput.value = toHex(data);
      base64Input.value = toBase64(data);
    };

    // Process hex input
    const analyzeHex = () => {
      console.log('Analyzing hex input:', hexInput.value);
      try {
        const hexValue = hexInput.value.trim();
        if (!hexValue) {
          alert('Please enter some hex data');
          return;
        }

        const data = fromHex(hexValue);
        console.log('Parsed hex data:', data);
        updateData(data);
        emit('analyze-default');
      } catch (err) {
        console.error('Error parsing hex:', err);
        alert(`Failed to parse hex data: ${err.message}`);
      }
    };

    // Process base64 input
    const analyzeBase64 = () => {
      console.log('Analyzing base64 input:', base64Input.value);
      try {
        const base64Value = base64Input.value.trim();
        if (!base64Value) {
          alert('Please enter some base64 data');
          return;
        }

        const data = fromBase64(base64Value);
        console.log('Parsed base64 data:', data);
        updateData(data);
        emit('analyze-default');
      } catch (err) {
        console.error('Error parsing base64:', err);
        alert(`Failed to parse base64 data: ${err.message}`);
      }
    };

    return {
      tabs,
      hexInput,
      base64Input,
      loadSimpleSample,
      loadComplexSample,
      loadCustomSample,
      analyzeHex,
      analyzeBase64,
      updateData
    };
  }
}
</script>