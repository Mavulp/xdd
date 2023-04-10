<script setup lang='ts'>
import { computed, onBeforeMount, reactive, ref, watch } from 'vue'
import { maxLength, minLength, required, url, useValidation } from '@dolanske/v-valid'
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
    minLength: minLength(1),
    maxLength: maxLength(32),
  },
  content: {
    required,
    minLength: minLength(1),
    maxLength: maxLength(8192),
  },
  // FIXME remove after updating v-valid
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
const contentUrl = ref<string>()

watch(() => form.content, async (value) => {
  contentUrl.value = undefined

  if (!value)
    return

  // Check if content is URL or plain text
  if (url.validate(value)) {
    const emoteSizeThreshold = 64
    const image = new Image()
    image.src = value

    image.onload = () => {
      contentUrl.value = image.src

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
      <h3>{{ isEdit ? `Edit "${route.params.name}"` : 'Add Alias' }}</h3>

      <div class="form-wrap">
        <div class="form-preview">
          <span>Preview</span>

          <template v-if="form.content">
            <img v-if="contentUrl" :src="contentUrl" alt=" ">
            <p v-else>
              {{ form.content }}
            </p>
          </template>

          <strong v-else>
            No content to display. Add some
            <Icon icon="mdi:arrow-right" />
          </strong>
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
