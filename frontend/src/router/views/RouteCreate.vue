<script setup lang='ts'>
import { computed, onBeforeMount, reactive, watch } from 'vue'
import { maxLenNoSpace, maxLength, minLenNoSpace, minLength, required, url, useValidation } from '@dolanske/v-valid'
import { useRoute, useRouter } from 'vue-router'
import type { PostAlias } from '../../types/PostAlias'
import InputText from '../../components/form/InputText.vue'
import InputTextarea from '../../components/form/InputTextarea.vue'
import { useLoading } from '../../store/loading'
import { LOAD } from '../../js/definitions'
import Spinner from '../../components/generic/Spinner.vue'
import InputSelect from '../../components/form/InputSelect.vue'
import { categoryLabels, useAlias } from '../../store/alias'
import { useToast } from '../../store/toast'
import { isImageValidRule, noExclamationMarkRule } from '../../js/rules'

const loading = useLoading()
const alias = useAlias()
const route = useRoute()
const router = useRouter()
const toast = useToast()
const isEdit = computed(() => route.name === 'RouteEdit')

// Define form and validation rules
const form = reactive<PostAlias>({
  name: '',
  content: '',
  type: 'emote',
})

const rules = {
  name: {
    required,
    minLength: minLenNoSpace(1),
    maxLength: maxLenNoSpace(24),
    noExclamationMarkRule,
  },
  content: {
    required,
    minLength: minLength(1),
    maxLength: maxLength(8192),
    isImageValidRule,
  },
  type: {},
}

const validation = useValidation(form, rules, { autoclear: true })

async function submit() {
  // Validate form
  validation.reset()
  validation.validate()
    .then(() => {
      // Send the correct API call based on if we're editing / creating
      if (isEdit.value) {
        alias.edit(String(route.params.name), {
          content: form.content,
          type: form.type,
        })
          .then(() => {
            router.push({ name: 'RouteHome' })
          })
      }
      else {
        alias.add(form)
      }
    })
}

// Reset function to default
// In case we're editing, also return back to alias list
function clear() {
  form.name = ''
  form.content = ''
  validation.reset()

  if (isEdit.value)
    router.push({ name: 'RouteHome' })
}

// Form editing check
onBeforeMount(() => {
  if (!isEdit.value)
    return

  const item = alias.list.find(a => a.name === route.params.name)
  if (!item) {
    // Handle error later
    toast.push({
      type: 'error',
      message: 'Incorrect alias identifier. Alias does not exist.',
    })
    router.push({ name: 'RouteHome' })
    return
  }

  const { name, content, type } = item
  Object.assign(form, { name, content, type })
})

// Watch for input content and manually select type
const defaultMeta = {
  url: null,
  width: 0,
  height: 0,
  type: undefined,
}
const emoteSizeThreshold = 40
const contentMeta = reactive({ ...defaultMeta })

watch(() => form.content, async (value) => {
  console.log(url.validate(value))

  Object.assign(contentMeta, { ...defaultMeta })

  if (!value)
    return

  // Check if content is URL or plain text
  if (url.validate(value)) {
    const image = new Image()
    image.src = value

    image.onload = () => {
      // contentUrl.value = image.src
      Object.assign(contentMeta, {
        url: image.src,
        width: image.naturalWidth,
        height: image.naturalHeight,
        type: image.src.split('.').at(-1),
      })

      if (image.naturalHeight > emoteSizeThreshold || image.naturalWidth > emoteSizeThreshold)
        form.type = value.endsWith('.gif') ? 'gif' : 'image'
      else
        form.type = value.endsWith('.gif') ? 'animatedEmote' : 'emote'
    }
  }
  else {
    form.type = 'text'
  }
})
</script>

<template>
  <div class="route-create">
    <div class="container mid">
      <h3>{{ isEdit ? `Edit "${route.params.name}"` : 'Create Alias' }}</h3>
      <div class="form-wrap">
        <div class="form-preview-wrap">
          <span>Preview</span>
          <div class="form-preview">
            <template v-if="form.content">
              <img v-if="contentMeta.url" :src="contentMeta.url" alt=" ">
              <p v-else>
                {{ form.content }}
              </p>
            </template>

            <strong v-else>
              Nothing to preview
              <!-- <Icon icon="mdi:arrow-right" /> -->
            </strong>
          </div>
          <div class="preview-meta">
            <template v-if="contentMeta.url">
              <p v-if="contentMeta.type">
                {{ contentMeta.type }}
              </p>
              <p>{{ contentMeta.width }}x{{ contentMeta.height }}</p>
            </template>
            <p v-else-if="form.content">
              {{ form.content.split(/(\s+)/).length }} words
            </p>
          </div>
        </div>
        <!-- <div class="form-create"> -->
        <form class="form-create" @submit.prevent="submit">
          <InputText
            v-model="form.name"
            :disabled="isEdit"
            :label="`Alias Name ${isEdit ? '(cannot be edited)' : ''}`"
            placeholder="funny"
            :err="validation.errors.name"
          />
          <p v-if="!isEdit" class="form-note">
            <Icon icon="mdi:info" />
            The alias name is what gets replaced with its content. <br>
            Name should be simple and easy to remember.
          </p>
          <InputTextarea
            v-model="form.content"
            label="Content"
            placeholder="Paste a Url or a copy-pasta here..."
            :err="validation.errors.content"
          />
          <InputSelect
            v-model="form.type"
            label="Alias type"
            :options="categoryLabels"
            cantclear
          />

          <p v-if="!isEdit" class="form-note">
            <Icon icon="mdi:info" />
            The type is automatically set based on the input but can be changed. <br> The main difference between an <code>image/gif</code> and a <code>emote/animatedEmote</code> is the size. Emotes (below {{ emoteSizeThreshold }}x{{ emoteSizeThreshold }}) are displayed inline with text while <br> images / gifs render in full size, pushing the remaining content aside.
          </p>

          <div class="flex right">
            <button
              v-if="form.content || form.name"
              class="button btn-white"
              @click.prevent="clear()"
            >
              <Icon icon="ph:x" />
              Cancel
            </button>
            <button
              class="button btn-accent btn-wider"
              :disabled="loading.get(LOAD.CREATE, LOAD.EDIT)"
              style="width:84px"
              type="submit"
            >
              <Spinner v-if="loading.get(LOAD.CREATE, LOAD.EDIT)" />
              <template v-else>
                {{ isEdit ? 'Update' : 'Create' }}
              </template>
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
