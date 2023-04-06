<script setup lang='ts'>
import { computed, reactive, ref } from 'vue'
import { maxLength, minLength, required, useValidation } from '@dolanske/v-valid'
import type { PostAlias } from '../../types/PostAlias'
import InputText from '../../components/form/InputText.vue'
import InputTextarea from '../../components/form/InputTextarea.vue'
import { post } from '../../js/fetch'
import { useToast } from '../../store/toast'
import { useLoading } from '../../store/loading'
import { LOAD } from '../../js/definitions'
import Spinner from '../../components/generic/Spinner.vue'
import InputSelect from '../../components/form/InputSelect.vue'
import type { AliasType } from '../../types/AliasType'

const { push } = useToast()
const loading = useLoading()

// TODO: type
// this is all preparation for adding a type to the alias object
const type = ref<AliasType>('emote')
const typeOptions: Record<AliasType, string> = {
  animatedEmote: 'Animated Emote',
  emote: 'Emote',
  gif: 'Gif',
  image: 'Image',
  text: 'Text',
}

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
      loading.add(LOAD.CREATE)
      post('/alias', form)
        .then(() => push({
          type: 'success',
          message: `Successfully added alias "${form.name}""`,
        }))
        .catch(({ message }) => push({
          type: 'error',
          message,
        }))
        .finally(() => loading.del(LOAD.CREATE))
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
          :options="typeOptions"
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

      <pre>
        {{ form }}
      </pre>

      <!-- <div class="preview">
          hehe
        </div>
      </div> -->
    </div>
    <div class="container-mid" />
  </div>
</template>
