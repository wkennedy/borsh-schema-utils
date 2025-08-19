<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Structure Hypothesis</h2>

    <div class="space-y-4">
      <button @click="$emit('extract')" class="btn">
        Extract Structure Hypothesis
      </button>

      <div class="bg-gray-900 text-gray-200 p-4 rounded-md overflow-auto">
        <pre class="font-mono text-sm whitespace-pre-wrap" v-if="hypothesis">{{ hypothesis }}</pre>
        <p class="italic text-gray-400" v-else>No data analyzed yet</p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    currentData: {
      type: Object, // Uint8Array
      required: false,
      default: null
    },
    hypothesis: {
      type: String,
      required: false,
      default: ''
    }
  },
  emits: ['extract']
}
</script>

<style scoped>
pre {
  max-height: 300px;
}
</style>