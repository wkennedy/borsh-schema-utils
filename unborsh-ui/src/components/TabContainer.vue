<template>
  <div>
    <div class="flex border-b">
      <div
          v-for="tab in tabs"
          :key="tab.id"
          class="tab"
          :class="{ 'active': activeTab === tab.id }"
          @click="setActiveTab(tab.id)"
      >
        {{ tab.label }}
      </div>
    </div>
    <div class="py-4">
      <div v-for="tab in tabs" :key="`content-${tab.id}`" v-show="activeTab === tab.id">
        <slot :name="tab.id"></slot>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  props: {
    tabs: {
      type: Array,
      required: true,
      // tabs should be in format [{ id: 'tab1', label: 'Tab 1' }, ...]
    },
    initialTab: {
      type: String,
      default: ''
    }
  },
  setup(props) {
    const activeTab = ref(props.initialTab || (props.tabs.length > 0 ? props.tabs[0].id : ''));

    const setActiveTab = (tabId) => {
      console.log('Setting active tab to:', tabId);
      activeTab.value = tabId;
    };

    return {
      activeTab,
      setActiveTab
    };
  }
}
</script>