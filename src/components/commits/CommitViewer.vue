<template>
  <div class="viewer-wrapper">
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
import { useGitDataStore } from '@/stores/gitData';

export default {
  components: {
    CommitGraph,
    CommitTable
  },
  mounted() {
    const store = useGitDataStore();
    store.loadRepo('/home/notnight/dev/beekeeper-studio/')
    // invoke('get_changes', { repoPath: '/home/notnight/dev/beekeeper-studio/' });
  }
};
</script>

<style lang="scss" scoped>
.viewer-wrapper {
  display: flex;
  flex-direction: row;
  padding: 0;
  overflow-x: hidden;
  width: 100%;

  .table-wrapper {
    margin-top: 28px;
    width: 100%;

    overflow: visible;
  }
}
</style>