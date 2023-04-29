<script setup lang='ts'>
import { computed, onBeforeMount, reactive, watch } from 'vue'
import { maxLenNoSpace, maxLength, minLenNoSpace, minLength, required, url, useValidation } from '@dolanske/v-valid'
import { useRoute, useRouter } from 'vue-router'
import { IconClose, IconInfo, IconUpload } from '@iconify-prerendered/vue-mdi'
import type { AliasType } from 'src/types/AliasType'
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
import { post } from '../../js/fetch'

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

// Checks for content type
// By default also assigns image metadata to the form type
function getContentType(value: string, assignMeta = true): Promise<AliasType> {
  return new Promise((resolve) => {
    if (url.validate(value)) {
      const image = new Image()
      image.src = value

      image.onload = () => {
        if (assignMeta) {
          Object.assign(contentMeta, {
            url: image.src,
            width: image.naturalWidth,
            height: image.naturalHeight,
            type: image.src.split('.').at(-1),
          })
        }

        if (image.naturalHeight > emoteSizeThreshold || image.naturalWidth > emoteSizeThreshold)
          resolve(value.endsWith('.gif') ? 'gif' : 'image')
        else
          resolve(value.endsWith('.gif') ? 'animatedEmote' : 'emote')
      }

      // On whatever error, just stick to text
      image.onerror = () => resolve('text')
    }
    else {
      resolve('text')
    }
  })
}

watch(() => form.content, async (value) => {
  Object.assign(contentMeta, { ...defaultMeta })
  if (!value)
    return

  form.type = await getContentType(value)
})

// Upload CSV files
function showFileInput() {
  const input = document.createElement('input')
  input.type = 'file'
  input.oninput = e => upladCSV(e)
  input.click()
}

function upladCSV(e: Event) {
  const files = (e.target as HTMLInputElement).files

  if (files) {
    loading.add(LOAD.UPLOAD)

    const file = files[0]
    const reader = new FileReader()

    reader.onload = async (e) => {
      let finalResult: string
      const result = e?.target?.result

      if (!result) {
        toast.push({
          type: 'error',
          message: 'Could not process the uploaded file',
        })
        loading.del(LOAD.UPLOAD)
        return
      }

      // Convert any options to just string
      if (typeof result !== 'string') {
        const encoder = new TextDecoder('utf-8')
        finalResult = encoder.decode(result)
      }
      else {
        finalResult = result
      }

      const promises = []

      // Iterate over rows
      for (const row of finalResult.split('\n')) {
        const [name, content] = row.split(/,(.*)/s)
        // Create form submit object
        const type = await getContentType(content, false)
        const alias = { name, content, type }
        promises.push(post('/alias', alias))
      }

      Promise.all(promises)
        .then(() => toast.push({
          type: 'success',
          message: `Successfully uploaded ${promises.length} aliases`,
        }))
        .catch(e => toast.push({
          type: 'error',
          message: `Error uploading a CSV file. ${e}`,
        }))
        .finally(() => {
          loading.del(LOAD.UPLOAD)
        })
    }

    reader.onerror = () => {
      toast.push({
        type: 'error',
        message: 'Could not read the CSV file',
      })
      loading.del(LOAD.UPLOAD)
    }
    reader.readAsBinaryString(file)
  }
  else {
    toast.push({
      type: 'error',
      message: 'No file was input',
    })
  }
}
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
            <IconInfo />
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
            <IconInfo />
            The type is automatically set based on the input but can be changed. <br> The main difference between an <code>image / gif</code> and an <code>emote / animated emote</code> is the size. Emotes (below {{ emoteSizeThreshold }}x{{ emoteSizeThreshold }}) are displayed inline with text while <br> images / gifs render in full size, pushing the remaining content aside.
          </p>

          <div class="flex right">
            <button
              v-if="form.content || form.name"
              class="button btn-white"
              @click.prevent="clear()"
            >
              <IconClose />
              Cancel
            </button>

            <button class="button btn-gray btn-wider" type="button" style="width:87px;" @click="showFileInput">
              <Spinner v-if="loading.get(LOAD.UPLOAD)" />
              <template v-else>
                <IconUpload />
                CSV
              </template>
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
