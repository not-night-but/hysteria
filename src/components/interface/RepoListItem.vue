<template>
  <!-- class inherited from AppSidebar.vue -->
  <div class="repo" v-tooltip.right="repoName" @click="$emit('select', repo)">
    {{ abbreviation }}
  </div>
</template>

<script lang="ts">
import { Repo } from '@/lib/models';
import { PropType } from 'vue';

export default {
  props: {
    repo: {
      type: Object as PropType<Repo>,
      required: true
    }
  },
  emits: {
    select(payload: Repo) {
      return payload;
    }
  },
  computed: {
    repoName() {
      const expr = /\/(?<repo>[\w-]+?).git/g;
      const groups = expr.exec(this.repo.url)?.groups;
      return groups ? groups['repo'] : '??';
    },
    abbreviation() {
      if (this.repo.abb) return this.repo.abb.toUpperCase();
      const name = this.repoName;
      return `${name[0]}${name[1]}`.toUpperCase();
    }
  }
}
</script>

<style lang="scss"></style>