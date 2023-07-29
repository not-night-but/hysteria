<template>
  <div class="list">
    Branches

    <ul>
      <li v-for="(branch, index) of local" :key="index">
        {{ branch.name }}
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { useGitDataStore } from '@/stores/gitData';
import { mapState } from 'pinia';


export default {
  computed: {
    ...mapState(useGitDataStore, {
      branches: 'repoBranches'
    }),
    local() {
      return this.branches.filter((b) => !b.is_remote);
    },
    remote() {
      return this.branches.filter((b) => b.is_remote);
    }
  }
}
</script>

<style lang="scss" scoped>
.list {
  height: 50%;
  width: 300px;
  margin-bottom: 10px;
  background-color: $hys-bg-soft;
  border-radius: 20px;
  padding: 1rem;
}
</style>