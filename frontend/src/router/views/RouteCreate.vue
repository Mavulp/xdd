<script setup lang='ts'>
import { computed, onBeforeMount, reactive } from 'vue'
import { maxLength, minLength, required, useValidation } from '@dolanske/v-valid'
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

const form = reactive<PostAlias>({
  name: '',
  content: '',
  type: 'emote',
})

const rules = computed(() => ({
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
}))

const validation = useValidation(form, rules, { autoclear: true })

async function submit() {
  validation.reset()
  validation.validate()
    .then(() => {
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

function clear() {
  form.name = ''
  form.content = ''
  validation.reset()

  if (isEdit.value)
    router.push({ name: 'RouteHome' })
}

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
</script>

<template>
  <div class="route-create">
    <div class="container small">
      <h3>{{ isEdit ? `Edit "${route.params.name}"` : 'Add Alias' }}</h3>

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
</template>
