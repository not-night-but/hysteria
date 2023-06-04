<template>
  <div class="app">
    <commit-viewer></commit-viewer>
  </div>
</template>

<script lang="ts">
import CommitViewer from './components/CommitViewer.vue';

export default {
  components: {
    CommitViewer
  }
};
</script>

<style scoped>
.app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
}
</style>
