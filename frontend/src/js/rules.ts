import { $def } from '@dolanske/v-valid'
import { isValidImage, validImageFormats } from './utils'

export const isImageValidRule = $def((value: string) => {
  return isValidImage(value)
}, `Image link must end with one of these extensions: ${validImageFormats.join(', ')}`)
