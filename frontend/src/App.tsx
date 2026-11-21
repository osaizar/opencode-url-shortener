import { useEffect, useRef, useState, type FormEvent } from 'react'
import { shortenUrl } from './api'
import { COPY_FALLBACK_MESSAGE, NETWORK_MESSAGE, validateUrl } from './validation'

type Status = 'idle' | 'submitting' | 'success' | 'error'

export default function App() {
  const [url, setUrl] = useState('')
  const [status, setStatus] = useState<Status>('idle')
  const [error, setError] = useState<string | null>(null)
  const [result, setResult] = useState<string | null>(null)
  const [copied, setCopied] = useState(false)
  const linkRef = useRef<HTMLAnchorElement>(null)
  const timerRef = useRef<number | null>(null)

  useEffect(
    () => () => {
      if (timerRef.current !== null) {
        window.clearTimeout(timerRef.current)
      }
    },
    [],
  )

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setError(null)
    setResult(null)
    setCopied(false)

    const localError = validateUrl(url)
    if (localError !== null) {
      setError(localError)
      setStatus('error')
      return
    }

    setStatus('submitting')
    try {
      const response = await shortenUrl(url.trim())
      setResult(response.short_url)
      setStatus('success')
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : NETWORK_MESSAGE)
      setStatus('error')
    }
  }

  async function handleCopy() {
    if (result === null) {
      return
    }
    try {
      await navigator.clipboard.writeText(result)
      setCopied(true)
      if (timerRef.current !== null) {
        window.clearTimeout(timerRef.current)
      }
      timerRef.current = window.setTimeout(() => setCopied(false), 2000)
    } catch {
      linkRef.current?.focus()
      const range = document.createRange()
      if (linkRef.current !== null) {
        range.selectNodeContents(linkRef.current)
        const selection = window.getSelection()
        selection?.removeAllRanges()
        selection?.addRange(range)
      }
      setError(COPY_FALLBACK_MESSAGE)
    }
  }

  const submitting = status === 'submitting'

  return (
    <main className="page">
      <h1>URL shortener</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="url-input">Long URL</label>
        <input
          id="url-input"
          name="url"
          type="text"
          value={url}
          onChange={(event) => setUrl(event.target.value)}
          disabled={submitting}
        />
        <button type="submit" disabled={submitting}>
          {submitting ? 'Shortening…' : 'Shorten'}
        </button>
      </form>
      {error !== null && (
        <p role="alert" className="error">
          {error}
        </p>
      )}
      {result !== null && (
        <section className="result">
          <a ref={linkRef} href={result}>
            {result}
          </a>
          <button type="button" onClick={handleCopy}>
            {copied ? 'Copied!' : 'Copy'}
          </button>
        </section>
      )}
    </main>
  )
}
