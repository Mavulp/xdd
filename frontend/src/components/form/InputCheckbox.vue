<script setup lang="ts">
import { computed } from 'vue'
import { IconCheckboxBlank, IconCheckboxMarked } from '@iconify-prerendered/vue-mdi'

interface Props {
  label?: string
  modelValue: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const data = computed<boolean>({
  get() {
    return props.modelValue
  },
  set(value) {
    emit('update:modelValue', value)
  },
})

const d = computed(() => `id${Math.random().toString(16).slice(2)}`)
</script>

<template>
  <div class="form-checkbox">
    <input :id="d" v-model="data" type="checkbox" :name="d">
    <label :for="d">

      <div class="icon">
        <IconCheckboxMarked v-if="modelValue" />
        <IconCheckboxBlank v-else />
      </div>

      <p v-if="props.label">{{ props.label }}</p>
    </label>
  </div>
</template>
