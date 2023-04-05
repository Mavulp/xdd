<script setup lang='ts'>
import { computed, reactive } from 'vue'
import { maxLength, minLength, required, useValidation } from '@dolanske/v-valid'
import type { PostAlias } from '../../types/PostAlias'
import InputText from '../../components/form/InputText.vue'
import InputTextarea from '../../components/form/InputTextarea.vue'

const form = reactive<PostAlias>({
  name: '',
  content: '',
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
      // post(form.name, form.content)
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
      <div class="flex between title">
        <h3>New Alias</h3>

        <button
          v-if="form.name || form.content"
          class="button btn-icon btn-white"
          data-title-top="Reset"
          @click="clear()"
        >
          <Icon icon="ph:x" />
        </button>
      </div>

      <!-- <div class="form-create"> -->
      <form class="form-create" @submit.prevent="submit">
        <InputText
          v-model="form.name"
          label="Alias Name"
          placeholder="!funny"
          :err="validation.errors.name"
        />
        <InputTextarea
          v-model="form.content"
          label="Content"
          placeholder="Paste a Url or a copy-pasta here..."
          :err="validation.errors.content"
        />

        <button class="button btn-accent">
          Create
        </button>
      </form>

      <pre>
        {{ validation.errors }}
      </pre>

      <!-- <div class="preview">
          hehe
        </div>
      </div> -->
    </div>
    <div class="container-mid" />
  </div>
</template>
