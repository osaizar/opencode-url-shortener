import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { afterEach, beforeEach, expect, it, vi } from 'vitest'
import App from './App'
import styles from './styles.css?raw'

const SHORT_URL = 'http://localhost:8080/aB3xY7z'
const VALID_URL = 'https://example.com/path?q=1'

let writeText: ReturnType<typeof vi.fn>

function fakeResponse(status: number, body: unknown): Response {
  return {
    ok: status >= 200 && status < 300,
    status,
    json: async () => body,
  } as Response
}

function successFetch() {
  return vi.fn().mockResolvedValue(fakeResponse(201, { code: 'aB3xY7z', short_url: SHORT_URL }))
}

function renderApp() {
  render(<App />)
  return {
    input: screen.getByLabelText('Long URL'),
    button: screen.getByRole('button', { name: 'Shorten' }),
  }
}

beforeEach(() => {
  writeText = vi.fn().mockResolvedValue(undefined)
  Object.defineProperty(navigator, 'clipboard', {
    configurable: true,
    value: { writeText },
  })
  vi.stubGlobal('fetch', successFetch())
})

afterEach(() => {
  cleanup()
  vi.unstubAllGlobals()
  vi.useRealTimers()
  vi.clearAllMocks()
})

it('renders the form with a visible label and a Shorten button', () => {
  const { input, button } = renderApp()
  expect(input).toBeInTheDocument()
  expect(input).toHaveAttribute('id', 'url-input')
  expect(button).toHaveAttribute('type', 'submit')
})

it('submits the URL and displays the returned short link', async () => {
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)

  const link = await screen.findByRole('link')
  expect(link).toHaveTextContent(SHORT_URL)
  expect(link).toHaveAttribute('href', SHORT_URL)
  expect(globalThis.fetch).toHaveBeenCalledWith(
    '/api/v1/urls',
    expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ url: VALID_URL }),
    }),
  )
})

it('disables the submit button and shows Shortening… while in flight', async () => {
  let resolveFetch!: (value: unknown) => void
  vi.stubGlobal(
    'fetch',
    vi.fn().mockReturnValue(
      new Promise((resolve) => {
        resolveFetch = resolve
      }),
    ),
  )
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)

  expect(button).toBeDisabled()
  expect(button).toHaveTextContent('Shortening…')

  await act(async () => {
    resolveFetch(fakeResponse(201, { code: 'aB3xY7z', short_url: SHORT_URL }))
  })
  expect(button).toBeEnabled()
  expect(button).toHaveTextContent('Shorten')
})

it('rejects empty input locally', async () => {
  const { button } = renderApp()
  fireEvent.click(button)
  expect(await screen.findByRole('alert')).toHaveTextContent('Enter a URL')
  expect(globalThis.fetch).not.toHaveBeenCalled()
})

it.each(['example.com', 'ftp://example.com'])('rejects %s locally', async (value) => {
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value } })
  fireEvent.click(button)
  expect(await screen.findByRole('alert')).toHaveTextContent('URL must use http or https')
  expect(globalThis.fetch).not.toHaveBeenCalled()
})

it('rejects input longer than 2048 characters locally', async () => {
  const base = 'https://example.com/'
  const value = base + 'a'.repeat(2049 - base.length)
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value } })
  fireEvent.click(button)
  expect(await screen.findByRole('alert')).toHaveTextContent('URL must be at most 2048 characters')
  expect(globalThis.fetch).not.toHaveBeenCalled()
})

it('displays the API error message in an alert', async () => {
  const message = 'URL must be a valid http or https URL of at most 2048 characters'
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(fakeResponse(400, { error: 'invalid_url', message })))
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  const alert = await screen.findByRole('alert')
  expect(alert).toHaveTextContent(message)
})

it('displays the network-failure message when fetch throws', async () => {
  vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('offline')))
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  expect(await screen.findByRole('alert')).toHaveTextContent(
    'Could not reach the server. Please try again.',
  )
})

it('clears the previous result and error on a new submission', async () => {
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  expect(await screen.findByRole('link')).toHaveTextContent(SHORT_URL)

  vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('offline')))
  fireEvent.change(input, { target: { value: 'https://example.com/second' } })
  fireEvent.click(button)
  await waitFor(() => expect(screen.queryByRole('link')).not.toBeInTheDocument())
  expect(await screen.findByRole('alert')).toBeInTheDocument()
})

it('copies the short link and restores the Copy label after 2 seconds', async () => {
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  await screen.findByRole('link')

  vi.useFakeTimers()
  const copy = screen.getByRole('button', { name: 'Copy' })
  fireEvent.click(copy)
  await act(async () => {
    await Promise.resolve()
  })
  expect(writeText).toHaveBeenCalledWith(SHORT_URL)
  expect(copy).toHaveTextContent('Copied!')

  act(() => {
    vi.advanceTimersByTime(2000)
  })
  expect(copy).toHaveTextContent('Copy')
})

it('selects the link and shows a message when clipboard writing fails', async () => {
  writeText.mockRejectedValue(new Error('denied'))
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  await screen.findByRole('link')

  fireEvent.click(screen.getByRole('button', { name: 'Copy' }))
  expect(await screen.findByRole('alert')).toHaveTextContent(
    'Copy failed — select the link manually.',
  )
  expect(window.getSelection()?.toString()).toBe(SHORT_URL)
})

it('supports keyboard operation and has visible focus styles', async () => {
  const user = userEvent.setup()
  const { input, button } = renderApp()

  await user.tab()
  expect(input).toHaveFocus()
  await user.tab()
  expect(button).toHaveFocus()
  expect(styles).toContain(':focus-visible')

  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.submit(input.closest('form')!)
  expect(await screen.findByRole('link')).toHaveTextContent(SHORT_URL)
})

it('keeps the layout usable at a narrow viewport', () => {
  expect(styles).toContain('@media (max-width: 360px)')
  expect(styles).toContain('overflow-wrap: anywhere')
})

it('shows only the most recent result after two submissions', async () => {
  const secondUrl = 'http://localhost:8080/zzzzzzz'
  vi.stubGlobal(
    'fetch',
    vi
      .fn()
      .mockResolvedValueOnce(fakeResponse(201, { code: 'aB3xY7z', short_url: SHORT_URL }))
      .mockResolvedValueOnce(fakeResponse(201, { code: 'zzzzzzz', short_url: secondUrl })),
  )
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  expect(await screen.findByRole('link')).toHaveTextContent(SHORT_URL)

  fireEvent.change(input, { target: { value: 'https://example.com/second' } })
  fireEvent.click(button)
  await waitFor(() => expect(screen.getByRole('link')).toHaveTextContent(secondUrl))
  expect(screen.queryByText(SHORT_URL)).not.toBeInTheDocument()
})
