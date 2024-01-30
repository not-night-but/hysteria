import { RepoData } from "@/lib/models";
import { invoke } from "@tauri-apps/api";
import { defineStore } from "pinia";

export const useRepoDataStore = defineStore('repoData', {
  state: () => ({
    data: new RepoData(),
    furthestX: new Map<number, number>()
  }),
  getters: {
    
  },
  actions: {
    async loadRepo(repoPath: string): Promise<void> {
      const data = await invoke('get_repo_data', {
        repoPath
      }).catch((err) => {
        console.error(err);
      }) as RepoData;
      console.log('DATA: ', data);
      this.data = data;
    },
    updateFurthestX(forY: number, x: number): void {
      const currentFurthest = this.furthestX.get(forY);
      if (!currentFurthest || currentFurthest < x) {
        this.furthestX.set(forY, x);
      }
    }
  }
})