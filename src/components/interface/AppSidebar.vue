<template>
  <div class="sidebar flex col">
    <div v-for="(repo, index) in repos" :key="index">
      <repo-list-item :repo="repo" @select="selectRepoOnClick"></repo-list-item>
    </div>
    <div class="repo" @click="openAddModal">
      <i class="material-icons">add</i>
    </div>
    <div class="expand"></div>
    <button class="settings">
      <i class="material-icons">settings</i>
    </button>

    <modal :show="showAddRepoModal" @close="showAddRepoModal = false">
      <h2>Add New Repo</h2>
      <form>
        <file-picker :value="directory" @selected="directory = $event" directory
          :title="'Select a Repository'"></file-picker>
      </form>
    </modal>
  </div>
</template>

<script lang="ts">
import { Repo } from '@/lib/models';
import { mapActions, mapState } from 'pinia';
import { useAppStore } from '@/stores/app';
import Modal from '@/components/BaseModal.vue';
import RepoListItem from './RepoListItem.vue';
import FilePicker from '@/components/controls/FilePicker.vue';

export default {
  components: {
    Modal,
    RepoListItem,
    FilePicker
  },
  data: () => {
    return {
      showAddRepoModal: false,
      directory: ''
    }
  },
  watch: {
    directory() {
      console.log(this.directory)
    }
  },
  computed: {
    ...mapState(useAppStore, {
      repos: 'repos',
      currentRepo: 'currentRepo'
    })
  },
  methods: {
    ...mapActions(useAppStore, ['selectRepo']),
    selectRepoOnClick(repo: Repo): void {
      if (repo.id != this.currentRepo?.id) {
        this.selectRepo(repo.id);
      }
    },
    openAddModal() {
      this.showAddRepoModal = true;
    }
  },
}
</script>

<style lang="scss" scoped>
.sidebar {
  min-width: $sidebar-width;
  width: $sidebar-width;
  background-color: $hys-bg-soft;
  height: 100vh;
  padding-top: 0.5rem;


  .repo {
    display: flex;
    border-radius: 50%;
    background: $hys-bg-softer;
    width: 2.5rem;
    height: 2.5rem;
    margin: auto;
    margin-bottom: 0.5rem;
    transition: all ease-in-out 0.2s;
    justify-content: center;
    align-items: center;

    &:hover {
      cursor: pointer;
      background: lighten($hys-bg-softer, 10%);
    }
  }

  .settings {
    background: transparent;
    border: none;
    padding: 5px;
    transition: all ease-in-out 0.2s;

    &:hover {
      cursor: pointer;
      background: $hys-bg-softer;
    }
  }
}
</style>