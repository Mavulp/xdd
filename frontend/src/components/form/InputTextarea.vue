<script setup lang='ts'>
import { computed, useAttrs } from 'vue'
import type { Error } from '@dolanske/v-valid/dist/src/types'

type Value = string | number | undefined
const props = defineProps<{
  modelValue: Value
  label?: string
  err?: Error
  cls?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: Value): void
}>()

const value = computed({
  get: () => props.modelValue,
  set: val => emit('update:modelValue', val),
})

const attrs = useAttrs()
const hasErr = computed(() => props.err && Object.keys(props.err.errors).length > 0)
</script>

<template>
  <div class="form-item" :class="[{ 'has-input': props.modelValue }, props.cls]">
    <label v-if="label">{{ label }}</label>
    <textarea v-model="value" v-bind="attrs" />

    <template v-if="hasErr && props.err">
      <p v-for="(message, errId) in props.err.errors" :key="errId" class="form-error">
        {{ message }}
      </p>
    </template>
  </div>
</template>
