<template>
  <div class="input-group">
    <input class="form-control clickable" type="text" placeholder="Nothing Selected" :title="value" :value="localValue"
      readonly @click.prevent.stop="openFilePickerDialog">
    <div class="input-group-append" @click.prevent.stop="openFilePickerDialog">
      <button type="button" class="btn btn-flat">
        {{ buttonText }}
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { open, OpenDialogOptions } from '@tauri-apps/api/dialog';

export default {
  props: {
    value: {
      type: String,
    },
    title: {
      type: String
    },
    directory: {
      type: Boolean
    },
    multiple: {
      type: Boolean
    },
    recursive: {
      type: Boolean
    },
    defaultPath: {
      type: String
    },
    buttonText: {
      type: String,
      default: 'Choose'
    }
  },
  data() {
    return {
      localValue: this.value
    }
  },
  computed: {
    options(): OpenDialogOptions {
      return {
        title: this.title ?? 'Select',
        defaultPath: this.defaultPath,
        multiple: this.multiple,
        directory: this.directory,
        recursive: this.recursive
      };
    }
  },
  methods: {
    async openFilePickerDialog() {
      const selected = await open(this.options);
      this.localValue = selected;

      this.$emit('selected', selected);
    }
  }
}
</script>