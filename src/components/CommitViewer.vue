<template>
  <div class="viewer-wrapper">
    <div class="mx-0 row align-items-start flex-nowrap" style="overflow-x: hidden; width: 100%">
      <div class="px-0 col-auto">
        <commit-graph></commit-graph>
      </div>
      <div class="px-0">
        <commit-table></commit-table>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
// import { invoke } from '@tauri-apps/api';
import CommitGraph from './CommitGraph.vue';
import CommitTable from './CommitTable.vue';
import { useGitDataStore } from '../stores/gitData';

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

<style lang="scss">
.viewer-wrapper {
  display: flex;
  flex-direction: row;
  padding: 0;
}
</style>