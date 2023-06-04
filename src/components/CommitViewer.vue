<template>
  <div class="htstack px-0">
    <div class="mx-0 row align-items-start flex-nowrap" style="overflow-x: hidden; width: 100%">
      <div class="px-0 col-auto">
        <commit-graph></commit-graph>
      </div>
      <div class="px-0">
        <!-- Table -->
      </div>
    </div>
  </div>
</template>

<script lang="ts">
// import { invoke } from '@tauri-apps/api';
import CommitGraph from './CommitGraph.vue';
import { useGitDataStore } from '../stores/gitData'; 

export default {
  components: {
    CommitGraph
  },
  mounted() {
    const store = useGitDataStore();
    store.loadRepo('/home/notnight/dev/beekeeper-studio/')
    // invoke('get_changes', { repoPath: '/home/notnight/dev/beekeeper-studio/' });
  }
};
</script>