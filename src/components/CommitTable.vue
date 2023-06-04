<template>
  <!-- TODO We need to set the width dynamically based on the viewport width - the svg width -->
  <div v-if="dataLoaded" class="table-wrapper" :style="'width: ' + '500' + 'px'">
    <div v-for="(commit, i) in commits">
      <div class="commit-entry" @click.prevent.stop="commit_onClick(commit)"
        :class="{ 'text-dark': commit.sha === clickedId }"
        :style="{ 'background': commit.sha === clickedId ? getColour(vertices?.at(i)) : '' }">
        <div class="commit-desc">
          {{ commit.subject }}
        </div>
        <div class="author-avatar">
          <img :src="getUserAvatar(commit.author?.email)" alt="avatar">
        </div>
        <div class="author-name">
          {{ commit.author?.name }}
        </div>
        <div class="commit-sha">
          {{ commit.sha?.substring(0, 7) }}
        </div>
        <div class="commit-date">
          {{ commit.date }}
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { mapState } from 'pinia';
import { useGitDataStore } from '../stores/gitData';
import { BranchData, Commit, Vertex } from '../lib/graph/classes';
import { Md5 } from 'ts-md5';

export default {
  data() {
    return {
      clickedId: null
    }
  },
  computed: {
    ...mapState(useGitDataStore, {
      commitDatas: 'commitDatas',
      dataBranchesMap: 'branchesMap',
      currentRepo: 'currentRepo',
    }),
    dataLoaded(): boolean {
      return this.commitDatas?.dataLoaded;
    },
    commits() {
      return this.commitDatas?.commitData;
    },
    vertices(): Vertex[] {
      return this.commitDatas?.vertices as Vertex[];
    },
    colours() {
      return this.commitDatas?.config.colours;
    },
    branches() {
      return this.dataBranchesMap?.get(this.currentRepo);
    },
    branchesMap() {
      if (this.commits.length > 0) {
        return new Map(this.branches?.map(branch => [this.commits.findIndex(commit => commit.sha === branch.tip_id), branch]));
      } else {
        return new Map();
      }
    }
  },
  methods: {
    commit_onClick(commit: Commit): void {
      this.clickedId = commit.sha;
    },
    getColour(vertex: Vertex | undefined): string {
      if (vertex === undefined) return '';
      this.colours[vertex.getColour() as number % this.colours.length];
    },
    getUserAvatar(email: string | null | undefined): string {
      var hash = Md5.hashStr(email?.trim().toLowerCase() ?? '');
      return `https://www.gravatar.com/avatar/${hash}?s=18&d=robohash`;
    },
    getBranchTag(data: BranchData | undefined): string {
      if (data === undefined) return '';
      return data.name;
    }
  }
};
</script>

<style lang="scss">
.table-wrapper {
  margin-left: 0;
  margin-right: 0;
  color: white;

  overflow-x: scroll;
  white-space: nowrap;
}

.commit-entry {
  transition: all 0.2s eas-in-out;
  font-size: 0.8rem;
  line-height: 1.5rem;
  display: flex;
  flex-direction: row;

  &:hover {
    cursor: pointer;
    background: hsl(0, 0%, 12%);
  }
}

.author-avatar {
  padding: 0;
  display: flex;
  align-items: center;

  img {
    display: block;
    margin: auto;
  }
}
</style>