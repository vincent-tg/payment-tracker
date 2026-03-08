"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { createTransaction, CreateTransactionRequest } from "@/lib/api"

interface AddTransactionFormProps {
  onSuccess?: () => void
}

export function AddTransactionForm({ onSuccess }: AddTransactionFormProps) {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [form, setForm] = useState<CreateTransactionRequest>({
    amount: 0,
    description: "",
    type: "out",
    date: new Date().toISOString().split("T")[0],
    currency: "USD",
    bank: "",
  })

  const handleChange = (field: keyof CreateTransactionRequest, value: any) => {
    setForm((prev) => ({ ...prev, [field]: value }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setError(null)

    try {
      await createTransaction(form)
      // Reset form
      setForm({
        amount: 0,
        description: "",
        type: "out",
        date: new Date().toISOString().split("T")[0],
        currency: "USD",
        bank: "",
      })
      onSuccess?.()
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create transaction")
    } finally {
      setLoading(false)
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Add Manual Transaction</CardTitle>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="amount">Amount</Label>
              <Input
                id="amount"
                type="number"
                step="0.01"
                required
                value={form.amount || ""}
                onChange={(e) => handleChange("amount", parseFloat(e.target.value))}
                placeholder="0.00"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="currency">Currency</Label>
              <Input
                id="currency"
                value={form.currency}
                onChange={(e) => handleChange("currency", e.target.value.toUpperCase())}
                placeholder="USD"
              />
            </div>
          </div>
          <div className="space-y-2">
            <Label htmlFor="description">Description</Label>
            <Input
              id="description"
              required
              value={form.description}
              onChange={(e) => handleChange("description", e.target.value)}
              placeholder="What was this transaction for?"
            />
          </div>
          <div className="space-y-2">
            <Label>Type</Label>
            <RadioGroup
              value={form.type}
              onValueChange={(value: "in" | "out") => handleChange("type", value)}
              className="flex space-x-4"
            >
              <div className="flex items-center space-x-2">
                <RadioGroupItem value="in" id="type-in" />
                <Label htmlFor="type-in" className="cursor-pointer">Income</Label>
              </div>
              <div className="flex items-center space-x-2">
                <RadioGroupItem value="out" id="type-out" />
                <Label htmlFor="type-out" className="cursor-pointer">Expense</Label>
              </div>
            </RadioGroup>
          </div>
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="date">Date</Label>
              <Input
                id="date"
                type="date"
                value={form.date}
                onChange={(e) => handleChange("date", e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="bank">Bank (optional)</Label>
              <Input
                id="bank"
                value={form.bank}
                onChange={(e) => handleChange("bank", e.target.value)}
                placeholder="Bank name"
              />
            </div>
          </div>
          {error && <p className="text-sm text-destructive">{error}</p>}
          <Button type="submit" disabled={loading}>
            {loading ? "Adding..." : "Add Transaction"}
          </Button>
        </form>
      </CardContent>
    </Card>
  )
}