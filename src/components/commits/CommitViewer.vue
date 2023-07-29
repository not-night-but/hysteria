<template>
  <div class="viewer-wrapper flex row">
    <div class="graph-wrapper">
      <commit-graph></commit-graph>
    </div>
    <div class="table-wrapper">
      <commit-table></commit-table>
    </div>
  </div>
</template>

<script lang="ts">
// import { invoke } from '@tauri-apps/api';
import CommitGraph from './CommitGraph.vue';
import CommitTable from './CommitTable.vue';

export default {
  components: {
    CommitGraph,
    CommitTable
  },
};
</script>

<style lang="scss" scoped>
.viewer-wrapper {
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: scroll;

  .table-wrapper {
    width: 100%;

    overflow-x: visible;
  }
}
</style>