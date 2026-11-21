import { NETWORK_MESSAGE } from './validation'

export interface CreateUrlResponse {
  code: string
  short_url: string
}

export class ApiRequestError extends Error {
  constructor(message: string) {
    super(message)
    this.name = 'ApiRequestError'
  }
}

export async function shortenUrl(url: string): Promise<CreateUrlResponse> {
  let response: Response
  try {
    response = await fetch('/api/v1/urls', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ url }),
    })
  } catch {
    throw new ApiRequestError(NETWORK_MESSAGE)
  }

  if (!response.ok) {
    let message = NETWORK_MESSAGE
    try {
      const body = (await response.json()) as { message?: unknown }
      if (typeof body.message === 'string') {
        message = body.message
      }
    } catch {
      // Keep the fallback message when the error body is not JSON.
    }
    throw new ApiRequestError(message)
  }

  return (await response.json()) as CreateUrlResponse
}
