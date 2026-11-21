import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, beforeEach, expect, it, vi } from 'vitest'
import App from './App'
import './styles.css'
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
  vi.stubGlobal(
    'fetch',
    vi.fn().mockResolvedValue(fakeResponse(201, { code: 'aB3xY7z', short_url: SHORT_URL })),
  )
})

afterEach(() => {
  cleanup()
  vi.unstubAllGlobals()
  vi.useRealTimers()
  vi.clearAllMocks()
})

it('shows READY with a prompt and decorative layers when idle', () => {
  renderApp()
  expect(screen.getByText('READY')).toBeInTheDocument()
  expect(screen.getByText('>')).toHaveAttribute('aria-hidden', 'true')
  expect(document.querySelector('.scanlines')).toHaveAttribute('aria-hidden', 'true')
  expect(document.querySelector('.boot')).toHaveAttribute('aria-hidden', 'true')
  expect(screen.getByRole('heading')).toHaveTextContent('[ URL SHORTENER ]')
})

it('shows SHORTENING... in amber while submitting', async () => {
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

  const status = screen.getByText('SHORTENING...')
  expect(status).toHaveClass('status-busy')
  await act(async () => {
    resolveFetch(fakeResponse(201, { code: 'aB3xY7z', short_url: SHORT_URL }))
  })
})

it('shows LINK READY and keeps the result link in the DOM', async () => {
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  expect(await screen.findByText(SHORT_URL)).toBeInTheDocument()
  expect(screen.getByText('LINK READY')).toBeInTheDocument()
  expect(document.querySelector('.result a')).toBeInTheDocument()
})

it('shows COPY OK after a successful clipboard write', async () => {
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  await screen.findByRole('link')

  fireEvent.click(screen.getByRole('button', { name: 'Copy' }))
  await waitFor(() => expect(screen.getByText('COPY OK')).toBeInTheDocument())
})

it('shows CLIPBOARD DENIED in error colour and keeps the R002 fallback message', async () => {
  writeText.mockRejectedValue(new Error('denied'))
  const { input, button } = renderApp()
  fireEvent.change(input, { target: { value: VALID_URL } })
  fireEvent.click(button)
  await screen.findByRole('link')

  fireEvent.click(screen.getByRole('button', { name: 'Copy' }))
  await waitFor(() => expect(screen.getByText('CLIPBOARD DENIED')).toHaveClass('status-error'))
  expect(await screen.findByRole('alert')).toHaveTextContent(
    'Copy failed — select the link manually.',
  )
})

it('uses the error colour for validation errors and falls back to READY', async () => {
  const { button } = renderApp()
  fireEvent.click(button)
  const alert = await screen.findByRole('alert')
  expect(alert).toHaveClass('error')
  expect(alert).toHaveTextContent('Enter a URL')
  expect(screen.getByText('READY')).toBeInTheDocument()
})

it('defines the terminal palette rules in the stylesheet', () => {
  // jsdom does not resolve CSS custom properties, so browser smoke is the
  // authoritative check for computed colours and pointer-events.
  renderApp()
  expect(styles).toContain('background: var(--bg)')
  expect(styles).toContain('color: var(--phosphor)')
  expect(document.querySelector('.scanlines')).not.toBeNull()
  expect(document.querySelector('.boot')).not.toBeNull()
})

it('defines blink, boot and typing animations plus reduced-motion rules in CSS', () => {
  expect(styles).toContain('--phosphor: #33ff66')
  expect(styles).toContain('--bg: #050805')
  expect(styles).toContain('--amber: #ffb000')
  expect(styles).toContain('--error: #ff5f56')
  expect(styles).toContain('monospace')
  expect(styles).toContain('@keyframes blink')
  expect(styles).toContain('@keyframes boot-out')
  expect(styles).toContain('@keyframes typing')
  expect(styles).toContain('500ms')
  expect(styles).toContain('pointer-events: none')
  expect(styles).toContain('@media (prefers-reduced-motion: reduce)')
  expect(styles).toContain('animation: none')
  expect(styles).toContain(':focus-visible')
})
