<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Analysis Options</h2>

    <div class="space-y-4">
      <div>
        <label for="strategySelect" class="block text-sm font-medium text-gray-700 mb-1">
          Analysis Strategy:
        </label>
        <select
            id="strategySelect"
            v-model="localOptions.strategy"
            class="form-input"
        >
          <option value="3">Comprehensive (Default)</option>
          <option value="0">Pattern Matching</option>
          <option value="1">Recursive Analysis</option>
          <option value="2">Probabilistic Analysis</option>
        </select>
      </div>

      <div>
        <label for="maxDepth" class="block text-sm font-medium text-gray-700 mb-1">
          Max Depth:
        </label>
        <input
            type="number"
            id="maxDepth"
            v-model.number="localOptions.maxDepth"
            class="form-input"
            min="1"
            max="20"
        />
      </div>

      <div>
        <label for="minConfidence" class="block text-sm font-medium text-gray-700 mb-1">
          Min Confidence (%):
        </label>
        <input
            type="number"
            id="minConfidence"
            v-model.number="localOptions.minConfidence"
            class="form-input"
            min="0"
            max="100"
        />
      </div>

      <div class="flex items-center">
        <input
            type="checkbox"
            id="includeRawBytes"
            v-model="localOptions.includeRawBytes"
            class="h-4 w-4 text-blue-600 rounded border-gray-300 focus:ring-blue-500"
        />
        <label for="includeRawBytes" class="ml-2 block text-sm text-gray-700">
          Include Raw Bytes
        </label>
      </div>

      <div>
        <label for="maxMatches" class="block text-sm font-medium text-gray-700 mb-1">
          Max Matches:
        </label>
        <input
            type="number"
            id="maxMatches"
            v-model.number="localOptions.maxMatches"
            class="form-input"
            min="1"
        />
      </div>

      <button @click="handleAnalyze" class="btn">Analyze with Options</button>
    </div>
  </div>
</template>

<script>
import { ref, watch } from 'vue';

export default {
  props: {
    options: {
      type: Object,
      required: true
    }
  },
  emits: ['analyze', 'update:options'],
  setup(props, { emit }) {
    // Create a local copy of the options
    const localOptions = ref({
      strategy: 3,
      maxDepth: 5,
      minConfidence: 30,
      includeRawBytes: false,
      maxMatches: 20
    });

    // Initialize with props
    watch(() => props.options, (newOptions) => {
      if (newOptions) {
        localOptions.value = { ...newOptions };
      }
    }, { immediate: true });

    // Update parent when local options change
    watch(localOptions, (newOptions) => {
      emit('update:options', { ...newOptions });
    }, { deep: true });

    const handleAnalyze = () => {
      emit('analyze');
    };

    return {
      localOptions,
      handleAnalyze
    };
  }
}
</script>