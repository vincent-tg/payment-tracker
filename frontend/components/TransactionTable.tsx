"use client"

import { useState, useEffect } from "react"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { getTransactions, Transaction } from "@/lib/api"

export function TransactionTable() {
  const [transactions, setTransactions] = useState<Transaction[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadTransactions()
  }, [])

  async function loadTransactions() {
    try {
      setLoading(true)
      const data = await getTransactions({ limit: 50 })
      setTransactions(data)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load transactions")
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return <div className="py-8 text-center text-muted-foreground">Loading transactions...</div>
  }

  if (error) {
    return (
      <div className="py-8 text-center">
        <p className="text-destructive">Error: {error}</p>
        <button
          onClick={loadTransactions}
          className="mt-2 text-sm text-primary underline"
        >
          Retry
        </button>
      </div>
    )
  }

  if (transactions.length === 0) {
    return <div className="py-8 text-center text-muted-foreground">No transactions yet.</div>
  }

  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Date</TableHead>
          <TableHead>Description</TableHead>
          <TableHead>Amount</TableHead>
          <TableHead>Type</TableHead>
          <TableHead>Currency</TableHead>
          <TableHead>Bank</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {transactions.map((tx) => (
          <TableRow key={tx.id}>
            <TableCell className="font-medium">{new Date(tx.date).toLocaleDateString()}</TableCell>
            <TableCell>{tx.description}</TableCell>
            <TableCell className={tx.type === "in" ? "text-green-600" : "text-red-600"}>
              {tx.formatted_amount}
            </TableCell>
            <TableCell>
              <span
                className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold ${
                  tx.type === "in"
                    ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100"
                    : "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-100"
                }`}
              >
                {tx.type}
              </span>
            </TableCell>
            <TableCell>{tx.currency}</TableCell>
            <TableCell>{tx.bank}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}