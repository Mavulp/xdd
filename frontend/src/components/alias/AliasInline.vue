<script setup lang='ts'>
import { useClipboard } from '@vueuse/core'
import { useAlias } from '../../store/alias'
import { useToast } from '../../store/toast'
import type { Alias } from '../../types/Alias'

const props = defineProps<{ data: Alias }>()
const { push } = useToast()
const { copy } = useClipboard()
const alias = useAlias()

function copyAlias() {
  const name = props.data.name
  copy(`!${name}`)
    .then(() => push({
      type: 'success',
      message: `Copied !${name} to clipboard`,
    }))
}

function copyContent() {
  const { name, content } = props.data
  copy(content)
    .then(() => push({
      type: 'success',
      message: `Copied ${name}'s content to clipboard`,
    }))
}
</script>

<template>
  <button
    :key="props.data.name"
    class="alias-item alias-inline"
    @click.self="alias.$patch({ active: props.data.name })"
  >
    <div class="thumbnail">
      <img
        v-if="props.data.type !== 'text'"
        :src="props.data.content"
        alt=" "
      >
      <p v-else :title="props.data.content">
        {{ props.data.content }}
      </p>
    </div>

    <strong>{{ props.data.name }}</strong>

    <button class="button btn-white btn-icon" data-title-bottom="Copy Alias" @click="copyAlias">
      <Icon icon="mdi:content-copy" />
    </button>
    <button class="button btn-white btn-icon" data-title-bottom="Copy Content (url or text)" @click="copyContent()">
      <Icon icon="mdi:attachment" />
    </button>
  </button>
</template>
