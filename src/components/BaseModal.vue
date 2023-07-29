<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="show" class="overlay">
        <div class="modal flex col">
          <slot />
          <div class="footer flex row">
            <div class="expand"></div>
            <button class="btn btn-secondary" type="button" @click="$emit('close')">
              Close
            </button>
            <button class="btn btn-primary" type="button" @click="$emit('done')">
              Done
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script lang="ts">

export default {
  props: {
    show: Boolean
  },
  emits: ['close', 'done']
}
</script>

<style lang="scss" scoped>
.overlay {
  transition: all ease 0.35s;
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 998;
  background: rgba($hys-bg, 80%);
}

.modal {
  position: fixed;
  z-index: 999;
  top: 20%;
  left: 50%;
  // TODO (@day): should this be dynamic?
  width: 500px;
  margin-left: -250px;
  min-height: 250px;
  background: $hys-bg-softer;
  border-radius: 10px;
  padding: 1rem;

  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.33);
  transition: all 0.3s ease;

  .footer {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 4rem;
    padding: 1rem;

  }
}

.modal-enter-from {
  opacity: 0;
}

.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  -webkit-transform: scale(1.1);
  transform: scale(1.1);
}
</style>