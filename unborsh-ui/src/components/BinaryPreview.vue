<template>
  <div class="binary-preview">
    <div class="bg-gray-900 text-gray-200 p-4 rounded-md overflow-auto">
      <pre class="font-mono text-sm">{{ formattedData }}</pre>
    </div>
  </div>
</template>

<script>
import { computed } from 'vue';
import { formatHexView } from '../utils/binaryUtils';

export default {
  props: {
    data: {
      type: Object, // Uint8Array
      required: false,
      default: null
    }
  },
  setup(props) {
    const formattedData = computed(() => {
      if (!props.data) return 'No data loaded';
      return formatHexView(props.data);
    });

    return {
      formattedData
    };
  }
}
</script>

<style scoped>
.binary-preview pre {
  max-height: 200px;
}
</style>