import { HysteriaStorePlugin } from './plugins/HysteriaStorePlugin';

declare module 'vue/types/vue' {
  interface Vue {
    $store: HysteriaStorePlugin
  }
}