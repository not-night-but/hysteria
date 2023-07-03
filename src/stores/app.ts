import { defineStore } from 'pinia';

export interface AppState {
}

export const useAppStore = defineStore('app', {
  state: () => ({
    selectedCommitId: null
  }),
  getters: {
    
  },
  actions: {
    selectCommit(commitId: string) {
      this.selectedCommitId = commitId;
    }
  }
});