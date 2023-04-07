<script setup lang='ts'>
import { computed, reactive } from 'vue'
import { maxLength, minLength, required, useValidation } from '@dolanske/v-valid'
import type { PostAlias } from '../../types/PostAlias'
import InputText from '../../components/form/InputText.vue'
import InputTextarea from '../../components/form/InputTextarea.vue'
import { useLoading } from '../../store/loading'
import { LOAD } from '../../js/definitions'
import Spinner from '../../components/generic/Spinner.vue'
import InputSelect from '../../components/form/InputSelect.vue'
import { categoryLabels, useEmotes } from '../../store/emotes'

const loading = useLoading()
const emotes = useEmotes()

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
      emotes.addAlias(form)
    })
}

function clear() {
  form.name = ''
  form.content = ''
  validation.reset()
}
</script>

<template>
  <div class="route-create">
    <div class="container small">
      <h3>New Alias</h3>

      <!-- <div class="form-create"> -->
      <form class="form-create" @submit.prevent="submit">
        <InputText
          v-model="form.name"
          label="Alias Name"
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
            :disabled="loading.get(LOAD.CREATE)"
            style="width:84px"
            type="submit"
          >
            <Spinner v-if="loading.get(LOAD.CREATE)" />
            <template v-else>
              Create
            </template>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
