import { $def, url } from '@dolanske/v-valid'
import { isValidImage, validImageFormats } from './utils'

export const isImageValidRule = $def((value: string) => {
  if (!url.validate(value))
    return true

  return isValidImage(value)
}, `Image link must end with one of these extensions: ${validImageFormats.join(', ')}`)

export const noExclamationMarkRule = $def((value: string) => {
  return !value.startsWith('!')
}, 'Don\'t add exclamation mark (\'!\') at the start of alias name')
