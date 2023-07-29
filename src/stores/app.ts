import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api';
import { Repo } from '@/lib/models';

interface AppState {
  selectedCommitId: string,
  repos: Array<Repo>,
  currentRepo: Repo | null
}

export const useAppStore = defineStore('app', {
  state: (): AppState => ({
    selectedCommitId: "",
    repos: [],
    currentRepo: null
  }),
  getters: {
    
  },
  actions: {
    async init() {
      try {
        const repos = (await invoke('get_user_repos')) as Array<Repo>;
        this.repos = repos;
        // TEMP (@day): We want to grab the most recently accessed repo
        this.currentRepo = repos[0];
      } catch (e) {
        console.error(e);
      }
    },
    selectCommit(commitId: string) {
      this.selectedCommitId = commitId;
    },
    selectRepo(id: number): void {
      this.currentRepo = this.repos.find((x) => x.id == id) as Repo;
      // TODO (@day): set as "recent repo"
    },
    async addRepo(repo: Repo) {
      await invoke('add_repo', { repo });
    }
  }
});