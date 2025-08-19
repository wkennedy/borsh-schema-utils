<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Analysis Results</h2>

    <div v-if="result" class="space-y-6">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-medium">Analysis Overview</h3>
        <span
            class="confidence px-3 py-1 rounded-full text-xs"
            :class="getConfidenceClass(result.confidence)"
        >
          Confidence: {{ result.confidence }}%
        </span>
      </div>

      <p>{{ result.description }}</p>

      <div v-if="result.matches && result.matches.length > 0">
        <h3 class="text-lg font-medium mb-4">Detected Patterns</h3>

        <div class="space-y-4">
          <div
              v-for="(match, index) in result.matches"
              :key="index"
              class="match-item"
              :class="getMatchConfidenceClass(match.confidence)"
          >
            <div class="flex items-center justify-between">
              <h4 class="font-medium">{{ match.pattern_name }}</h4>
              <span
                  class="confidence px-2 py-0.5 rounded-full text-xs"
                  :class="getConfidenceClass(match.confidence)"
              >
                {{ match.confidence }}%
              </span>
            </div>

            <p class="mt-1">{{ match.interpretation }}</p>

            <p class="text-sm text-gray-600 mt-1">
              Offset: 0x{{ match.offset.toString(16) }}, Length: {{ match.length }} bytes
            </p>

            <div v-if="match.data && match.data.length > 0" class="mt-2">
              <p class="text-xs text-gray-500 mb-1">Raw bytes:</p>
              <div class="bg-gray-100 p-2 rounded font-mono text-xs overflow-x-auto">
                {{ formatBytes(match.data) }}
              </div>
            </div>
          </div>
        </div>
      </div>
      <div v-else>
        <p class="text-gray-500 italic">No patterns detected</p>
      </div>
    </div>
    <div v-else>
      <p class="text-gray-500 italic">No analysis performed yet</p>
    </div>
  </div>
</template>

<script>
import { toHex } from '../utils/binaryUtils';

export default {
  props: {
    result: {
      type: Object,
      required: false,
      default: null
    }
  },
  setup() {
    // Format bytes for display
    const formatBytes = (bytes) => {
      if (!bytes || bytes.length === 0) return '';
      return toHex(new Uint8Array(bytes));
    };

    // Get confidence class based on confidence value
    const getConfidenceClass = (confidence) => {
      if (confidence >= 70) return 'confidence-high';
      if (confidence >= 50) return 'confidence-medium';
      return 'confidence-low';
    };

    // Get match item border color based on confidence
    const getMatchConfidenceClass = (confidence) => {
      if (confidence >= 70) return 'border-l-green-500';
      if (confidence >= 50) return 'border-l-yellow-500';
      return 'border-l-red-500';
    };

    // Get overall confidence class based on confidence value
    const getOverallConfidenceClass = (confidence) => {
      if (confidence >= 70) return 'confidence-high';
      if (confidence >= 50) return 'confidence-medium';
      return 'confidence-low';
    };

    return {
      formatBytes,
      getConfidenceClass,
      getMatchConfidenceClass,
      getOverallConfidenceClass
    };
  }
}
</script>