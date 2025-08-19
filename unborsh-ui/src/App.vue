<template>
  <div class="min-h-screen bg-gray-50 py-6">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <header class="text-center mb-8">
        <h1 class="text-3xl font-bold text-gray-900">Unborsh - Borsh Format Analyzer</h1>
        <p class="mt-2 text-gray-600">
          Analyze and visualize Borsh-serialized data with this interactive tool
        </p>
      </header>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Input Section - Full width -->
        <div class="col-span-1 lg:col-span-2 card">
          <InputSection
              :currentData="currentData"
              @data-updated="updateCurrentData"
              @analyze-default="analyzeDefault"
          />
        </div>

        <!-- Analysis Options -->
        <div class="card">
          <AnalysisOptions
              @analyze="analyzeWithOptions"
              :options="analysisOptions"
              @update:options="analysisOptions = $event"
          />
        </div>

        <!-- Pattern Interpreter -->
        <div class="card">
          <PatternInterpreter
              :currentData="currentData"
              :patterns="availablePatterns"
              @load-patterns="loadPatterns"
          />
        </div>

        <!-- Structure Hypothesis -->
        <div class="card">
          <StructureHypothesis
              :currentData="currentData"
              :hypothesis="hypothesis"
              @extract="extractHypothesis"
          />
        </div>

        <!-- Analysis Results - Full width -->
        <div class="col-span-1 lg:col-span-2 card">
          <AnalysisResults :result="analysisResult" />
        </div>
      </div>

      <footer class="mt-8 text-center text-gray-500 text-sm py-4">
        <p>Based on unborsh-wasm library. View the <a href="https://github.com/yourusername/unborsh" class="text-blue-500 hover:underline">source code</a>.</p>
      </footer>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { createSimpleSample } from './utils/sampleData';
import InputSection from './components/InputSection.vue';
import AnalysisOptions from './components/AnalysisOptions.vue';
import PatternInterpreter from './components/PatternInterpreter.vue';
import StructureHypothesis from './components/StructureHypothesis.vue';
import AnalysisResults from './components/AnalysisResults.vue';

// We'll dynamically import unborsh-wasm to ensure it's loaded correctly
let Unborsh, AnalysisStrategy;

export default {
  components: {
    InputSection,
    AnalysisOptions,
    PatternInterpreter,
    StructureHypothesis,
    AnalysisResults
  },
  setup() {
    const currentData = ref(null);
    const analysisResult = ref(null);
    const hypothesis = ref('');
    const availablePatterns = ref([]);
    const analysisOptions = ref({
      strategy: 3, // Comprehensive (default)
      maxDepth: 5,
      minConfidence: 30,
      includeRawBytes: false,
      maxMatches: 20
    });

    // Load unborsh-wasm module
    onMounted(async () => {
      try {
        const module = await import('/wasm/js/index.js?url');
        Unborsh = module.Unborsh;
        AnalysisStrategy = module.AnalysisStrategy;

        // Load initial data and analyze
        currentData.value = createSimpleSample();
        loadPatterns();
        analyzeDefault();
      } catch (err) {
        console.error('Failed to load unborsh-wasm:', err);
        alert('Failed to load the Unborsh analyzer module. Please check console for details.');
      }
    });

    // Update current data
    const updateCurrentData = (data) => {
      currentData.value = data;
    };

    // Analyze with default settings
    const analyzeDefault = () => {
      if (!currentData.value || !Unborsh) return;

      try {
        analysisResult.value = Unborsh.analyze(currentData.value);

        // If result has a hypothesis, update it
        if (analysisResult.value.structure_hypothesis) {
          hypothesis.value = analysisResult.value.structure_hypothesis;
        }
      } catch (err) {
        console.error('Analysis error:', err);
        alert(`Analysis failed: ${err.message}`);
      }
    };

    // Analyze with custom options
    const analyzeWithOptions = () => {
      if (!currentData.value || !Unborsh) return;

      try {
        const options = {
          strategy: analysisOptions.value.strategy,
          maxDepth: analysisOptions.value.maxDepth,
          minConfidence: analysisOptions.value.minConfidence,
          includeRawBytes: analysisOptions.value.includeRawBytes,
          maxMatches: analysisOptions.value.maxMatches
        };

        analysisResult.value = Unborsh.analyzeWithOptions(currentData.value, options);

        // If result has a hypothesis, update it
        if (analysisResult.value.structure_hypothesis) {
          hypothesis.value = analysisResult.value.structure_hypothesis;
        }
      } catch (err) {
        console.error('Analysis error:', err);
        alert(`Analysis failed: ${err.message}`);
      }
    };

    // Extract structure hypothesis
    const extractHypothesis = () => {
      if (!currentData.value || !Unborsh) return;

      try {
        hypothesis.value = Unborsh.extractStructureHypothesis(currentData.value);
      } catch (err) {
        console.error('Hypothesis extraction error:', err);
        alert(`Failed to extract hypothesis: ${err.message}`);
      }
    };

    // Load available patterns
    const loadPatterns = () => {
      if (!Unborsh) return;

      try {
        availablePatterns.value = Unborsh.getAvailablePatterns();
      } catch (err) {
        console.error('Failed to load patterns:', err);
        availablePatterns.value = [];
      }
    };

    return {
      currentData,
      analysisResult,
      hypothesis,
      availablePatterns,
      analysisOptions,
      updateCurrentData,
      analyzeDefault,
      analyzeWithOptions,
      extractHypothesis,
      loadPatterns
    };
  }
};
</script>