"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Mail } from "lucide-react"
import { fetchEmails } from "@/lib/api"

interface FetchEmailsButtonProps {
  onSuccess?: (result: any) => void
}

export function FetchEmailsButton({ onSuccess }: FetchEmailsButtonProps) {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleFetch = async () => {
    setLoading(true)
    setError(null)
    try {
      const result = await fetchEmails()
      onSuccess?.(result)
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to fetch emails")
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="flex flex-col gap-2">
      <Button onClick={handleFetch} disabled={loading} variant="outline">
        <Mail className="mr-2 h-4 w-4" />
        {loading ? "Fetching..." : "Fetch Emails"}
      </Button>
      {error && <p className="text-sm text-destructive">{error}</p>}
    </div>
  )
}