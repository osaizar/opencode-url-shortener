export const EMPTY_MESSAGE = 'Enter a URL'
export const SCHEME_MESSAGE = 'URL must use http or https'
export const LENGTH_MESSAGE = 'URL must be at most 2048 characters'
export const NETWORK_MESSAGE = 'Could not reach the server. Please try again.'
export const COPY_FALLBACK_MESSAGE = 'Copy failed — select the link manually.'

export function validateUrl(value: string): string | null {
  const input = value.trim()
  if (input.length === 0) {
    return EMPTY_MESSAGE
  }
  if (!input.startsWith('http://') && !input.startsWith('https://')) {
    return SCHEME_MESSAGE
  }
  if (input.length > 2048) {
    return LENGTH_MESSAGE
  }
  return null
}
