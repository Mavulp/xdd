import { isArray } from 'lodash-es'

// The scripts folder just contains any js / ts related stuff Feel free
// to create sub folders and group relevant functionality in files etc.
// We're not strict, but appreciate clean code and structure :)

// Return a random number in the provided range
export function getRanMinMax(min: number, max: number) {
  return Math.floor(Math.random() * (max - min) + min)
}

// Searches through the input and checkes wether it contains match It
// searches the input by splitting it by whitespace and matching each
// word against the string
export function searchInStr(match: string | string[], input: string) {
  if (!match)
    return false

  const joint: string = isArray(match) ? match.join(' ') : match

  const split = input.trim().split(/\s+/)
  return split.every(s => joint.toLowerCase().includes(s.toLowerCase()))
}

// Valid formats when inputting an image as an alias
export const validImageFormats = ['.jpeg', '.gif', '.png', '.apng', '.svg', '.bmp', '.bmp', '.ico', '.jpg', '.webp'] as const

export function isValidImage(text: string) {
  return validImageFormats.some(format => text.endsWith(format))
}

// Takes in a string and removes any valid inline javascript
export function sanitize(text: string) {
  if (!text)
    return ''

  const regex = /\bon\w+\=\"?[\w\:\(\)\']+\"?/g
  return text.replaceAll(regex, '')
}

export function parseJwt(token: string) {
  const base64Url = token.split('.')[1]
  const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
  const jsonPayload = decodeURIComponent(window.atob(base64).split('').map((c) => {
    return `%${(`00${c.charCodeAt(0).toString(16)}`).slice(-2)}`
  }).join(''))

  return JSON.parse(jsonPayload)
}
