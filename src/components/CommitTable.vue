<template>
  <!-- TODO We need to set the width dynamically based on the viewport width - the svg width -->
  <div v-for="(commit, i) in commits" @click="commit_onClick(commit)" class="commit-entry"
    :style="{ 'background': commit.sha === clickedId ? getColour((vertices as Vertex[])?.at(i)) : '', 'margin-left': `-${getLeftMargin(i)}px` }">
    <span :style="{ 'width': `${500 + getLeftMargin(i)}px` }"> {{ commit.subject }} </span>
    <span>
      <img :src="getUserAvatar(commit.author?.email)" alt="avatar">
      {{ commit.author?.name }}
    </span>
    <span> {{ commit.sha?.substring(0, 7) }} </span>
    <span>{{ commit.date }}</span>
  </div>
</template>

<script lang="ts">
import { mapActions, mapState } from 'pinia';
import { useGitDataStore } from '../stores/gitData';
import { BranchData, Commit, Vertex } from '../lib/graph/classes';
import { Md5 } from 'ts-md5';
import { useAppStore } from '../stores/app';

export default {
  data() {
    return {
      clickedId: ''
    }
  },
  computed: {
    ...mapState(useGitDataStore, {
      dataBranchesMap: 'branchesMap',
      currentRepo: 'currentRepo',
      dataLoaded: 'dataLoaded',
      commitData: 'commitData',
      vertices: 'vertices',
      config: 'config'
    }),
    commits() {
      return this.commitData;
    },
    colours() {
      return this.config.colours;
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
    ...mapActions(useAppStore, ['selectCommit']),
    ...mapActions(useGitDataStore, ['getLeftMargin']),
    commit_onClick(commit: Commit): void {
      if (commit.sha) {
        this.selectCommit(commit.sha);
        this.clickedId = commit.sha;
      }
    },
    getColour(vertex: Vertex | undefined): string {
      if (vertex === undefined) return '';
      return this.colours[vertex.getColour() as number % this.colours.length];
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
  transition: all 0.2s ease-in-out;
  font-size: 0.8rem;
  line-height: 24px;
  padding: 0;
  height: 24px;
  overflow: hidden;
  white-space: nowrap;

  span {
    display: inline-block;
  }

  &:hover {
    cursor: pointer;
    background: hsl(0, 0%, 12%);
  }

  td {
    border: none;
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