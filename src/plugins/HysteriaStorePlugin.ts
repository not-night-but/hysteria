import { useGitDataStore } from '../stores/gitData';
import { useAppStore } from '../stores/app';

export const HysteriaStorePlugin = {
  gitData() {
    return useGitDataStore();
  },
  app() {
    return useAppStore();
  }
}

export type HysteriaStorePlugin = typeof HysteriaStorePlugin;

export default {
  // @ts-expect-error
  install(app) {
    app.config.globalProperties.$store = HysteriaStorePlugin;
  }
}
