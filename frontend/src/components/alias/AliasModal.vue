<script setup lang='ts'>
// This is the pop-up component which displays information about an alias and is
// visible as long as a component is clicked on
import { computed, nextTick, ref, watch } from 'vue'
import { onKeyDown, useClipboard } from '@vueuse/core'
import { useRouter } from 'vue-router'
import { categoryLabels, useAlias } from '../../store/alias'
import { useToast } from '../../store/toast'
import Spinner from '../generic/Spinner.vue'
import { useLoading } from '../../store/loading'
import { LOAD } from '../../js/definitions'
import { useUser } from '../../store/user'

const loading = useLoading()
const alias = useAlias()
const user = useUser()
const { push } = useToast()
const active = computed(() => alias.activeAlias)

// const date = useDateFormat(Number(active.value?.createdAt ? Number(active.value.createdAt) * 1000 : 0), 'D MMMM YYYY')
const date = computed(() => {
  if (!active.value)
    return null

  const d = new Date(Number(active.value.createdAt) * 1000)

  return new Intl.DateTimeFormat('en-GB', {
    year: 'numeric',
    month: 'long',
    day: '2-digit',
  }).format(d)
})

const { copy } = useClipboard()

function close() {
  alias.$patch({ active: undefined })
}

function copyAlias() {
  if (!active.value)
    return

  const name = active.value.name
  copy(`!${name}`)
    .then(() => push({
      type: 'success',
      message: `Copied !${name} to clipboard`,
    }))
}

function copyContent() {
  if (!active.value)
    return

  const { name, content } = active.value
  copy(content)
    .then(() => push({
      type: 'success',
      message: `Copied ${name}'s content to clipboard`,
    }))
}

// Update padding from title depending on the height of the content
const contentEl = ref<HTMLDivElement>()
const showPadding = ref(false)

watch(() => active.value?.name, async () => {
  await nextTick()
  nextTick(() => {
    // The content wrapper is 141 pixels tall
    if (contentEl.value && contentEl.value.offsetHeight > 150)
      showPadding.value = true
    else
      showPadding.value = false
  })
}, { immediate: true })

// Close when escape is pressed
onKeyDown('Escape', close)

// Redirect to edit page
const router = useRouter()
function goToEdit() {
  if (!active.value)
    return

  router.push({
    name: 'RouteEdit',
    params: {
      name: active.value.name,
    },
  })
}
</script>

<template>
  <div class="alias-modal" :class="{ 'modal-active': alias.activeAlias }">
    <button class="button btn-icon btn-white btn-close" @click="close()">
      <Icon icon="mdi:close-thick" />
    </button>
    <div v-if="active" class="modal-wrapper">
      <div ref="contentEl" class="content">
        <div v-if="active.type !== 'text'" class="thumbnail">
          <img :src="active.content" alt=" ">
        </div>
        <div v-else class="text">
          <p>{{ active.content }}</p>
        </div>
      </div>
      <div class="info" :class="{ 'show-padding': showPadding }">
        <span class="category">{{ categoryLabels[active.type] }}</span>
        <div class="name">
          <strong>{{ active.name }}</strong>

          <button class="button btn-white btn-icon btn-large" data-title-bottom="Copy Alias" @click="copyAlias">
            <Icon icon="mdi:content-copy" />
          </button>
          <button class="button btn-white btn-icon btn-large" data-title-bottom="Copy Content (url or text)" @click="copyContent()">
            <Icon icon="mdi:attachment" />
          </button>
        </div>

        <div class="meta">
          <p>Added {{ date }}</p>
          <div class="spacer" />
          <p>By {{ active.author }}</p>
          <div v-if="user.can(['edit-aliases', 'delete-aliases'])" class="spacer" />

          <button v-if="user.can('edit-aliases')" @click="goToEdit()">
            Edit
          </button>
          <button v-if="user.can('delete-aliases')" @click="alias.remove(active?.name ?? '')">
            <Spinner v-if="loading.get(LOAD.DELETE)" />
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
