<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Pattern Interpretation</h2>

    <div class="space-y-4">
      <div>
        <label for="patternSelect" class="block text-sm font-medium text-gray-700 mb-1">
          Select Pattern:
        </label>
        <select
            id="patternSelect"
            v-model="selectedPattern"
            class="form-input"
        >
          <option v-if="patterns.length === 0" value="">Loading patterns...</option>
          <option v-for="pattern in patterns" :key="pattern" :value="pattern">
            {{ pattern }}
          </option>
        </select>
      </div>

      <div class="bg-gray-50 p-3 rounded-md border border-gray-200">
        <div v-if="patternDetails">
          <h3 class="font-medium">{{ patternDetails.name }}</h3>
          <p class="text-sm text-gray-600 mt-1">{{ patternDetails.description }}</p>
          <p class="text-sm mt-2"><span class="font-medium">Example:</span> {{ patternDetails.example }}</p>
        </div>
        <p v-else class="text-sm text-gray-500">Select a pattern to view details</p>
      </div>

      <button @click="interpretAsPattern" class="btn" :disabled="!selectedPattern">
        Interpret As Selected Pattern
      </button>

      <div v-if="interpretation" class="match-item p-3 mt-2">
        <div class="font-medium">Interpretation as {{ selectedPattern }}:</div>
        <p class="mt-1">{{ interpretation }}</p>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, watch, onMounted } from 'vue';

// We'll dynamically import unborsh-wasm
let Unborsh;

export default {
  props: {
    currentData: {
      type: Object, // Uint8Array
      required: false,
      default: null
    },
    patterns: {
      type: Array,
      required: true,
      default: () => []
    }
  },
  emits: ['load-patterns'],
  setup(props, { emit }) {
    const selectedPattern = ref('');
    const patternDetails = ref(null);
    const interpretation = ref('');

    // When component mounts, load patterns if needed
    onMounted(async () => {
      if (props.patterns.length === 0) {
        emit('load-patterns');
      }

      // Try to load Unborsh if it's available
      try {
        const module = await import('/wasm/js/index.js?url').then(module => {
          Unborsh = module.Unborsh;
        });
      } catch (err) {
        console.error('Failed to load unborsh-wasm:', err);
      }
    });

    // Update pattern details when pattern changes
    watch(selectedPattern, async (newPattern) => {
      if (!newPattern || !Unborsh) return;

      try {
        patternDetails.value = Unborsh.getPatternDetails(newPattern);
      } catch (err) {
        console.error('Failed to get pattern details:', err);
        patternDetails.value = null;
      }
    });

    // Interpret data as selected pattern
    const interpretAsPattern = () => {
      if (!props.currentData || !selectedPattern.value || !Unborsh) return;

      try {
        interpretation.value = Unborsh.interpretAs(props.currentData, selectedPattern.value);
      } catch (err) {
        console.error('Interpretation error:', err);
        interpretation.value = `Error: ${err.message}`;
      }
    };

    return {
      selectedPattern,
      patternDetails,
      interpretation,
      interpretAsPattern
    };
  }
}
</script>