<template>
  <div class="app flex row">
    <app-sidebar></app-sidebar>
    <div class="content flex col">
      <app-topbar></app-topbar>
      <div v-if="dataLoaded" class="repo flex row">
        <div class="lists flex col">
          <changes-list></changes-list>
          <branch-list></branch-list>
        </div>
        <commit-viewer></commit-viewer>
      </div>
    </div>
    <div id="modals"></div>
  </div>
</template>

<script lang="ts">
import { mapActions, mapState } from 'pinia';
import CommitViewer from './components/commits/CommitViewer.vue';
import AppSidebar from './components/interface/AppSidebar.vue';
import AppTopbar from './components/interface/AppTopbar.vue';
import BranchList from './components/repository/BranchList.vue';
import ChangesList from './components/repository/ChangesList.vue';
import { useAppStore } from './stores/app';
import { useGitDataStore } from './stores/gitData';
import { useRepoDataStore } from './stores/repoData';

export default {
  components: {
    CommitViewer,
    AppSidebar,
    AppTopbar,
    BranchList,
    ChangesList
  },
  data: () => {
    return {
      dataLoaded: false
    }
  },
  computed: {
    ...mapState(useAppStore, {
      currentRepo: 'currentRepo'
    })
  },
  methods: {
    ...mapActions(useAppStore, {
      init: 'init'
    }),
    ...mapActions(useRepoDataStore, {
      loadRepo: 'loadRepo'
    })
  },
  watch: {
    async currentRepo() {
      if (this.currentRepo) {
        await this.loadRepo(this.currentRepo.local_path);
        this.dataLoaded = true;
      }
    }
  },
  async mounted() {
    await this.init();
  }
};
</script>

<style lang="scss" scoped>
.app {
  margin: 0;
  padding: 0;
  height: 100vh;
  max-height: 100vh;
  width: 100vw;
  overflow: hidden;

  .content {
    width: 100%;
    height: 100%;

    .repo {
      margin-left: 10px;
      height: calc(100% - $topbar-height);

      .lists {
        margin-right: 10px;
      }
    }
  }
}
</style>
