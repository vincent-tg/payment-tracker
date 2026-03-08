"use client"

import { useState, useEffect } from "react"
import {
  LineChart as RechartsLineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts"
import { getTransactions } from "@/lib/api"

interface ChartData {
  date: string
  income: number
  expenses: number
}

export function IncomeExpenseChart() {
  const [data, setData] = useState<ChartData[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadData()
  }, [])

  async function loadData() {
    try {
      setLoading(true)
      const transactions = await getTransactions({ limit: 1000 }) // get enough to aggregate
      
      // Group by date and sum income/expenses
      const grouped = transactions.reduce((acc, tx) => {
        const date = tx.date.split("T")[0] // YYYY-MM-DD
        if (!acc[date]) {
          acc[date] = { income: 0, expenses: 0 }
        }
        if (tx.type === "in") {
          acc[date].income += tx.amount_usd
        } else {
          acc[date].expenses += tx.amount_usd
        }
        return acc
      }, {} as Record<string, { income: number; expenses: number }>)

      // Convert to array and sort by date
      const chartData: ChartData[] = Object.entries(grouped)
        .map(([date, sums]) => ({
          date,
          income: parseFloat(sums.income.toFixed(2)),
          expenses: parseFloat(sums.expenses.toFixed(2)),
        }))
        .sort((a, b) => a.date.localeCompare(b.date))

      // Limit to last 30 days for clarity
      const recent = chartData.slice(-30)
      setData(recent)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load chart data")
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return <div className="h-[300px] flex items-center justify-center text-muted-foreground">Loading chart...</div>
  }

  if (error) {
    return <div className="h-[300px] flex items-center justify-center text-destructive">Error: {error}</div>
  }

  if (data.length === 0) {
    return <div className="h-[300px] flex items-center justify-center text-muted-foreground">No transaction data yet.</div>
  }

  return (
    <div className="h-[300px] w-full">
      <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
        <RechartsLineChart data={data} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
          <XAxis dataKey="date" className="text-xs fill-muted-foreground" />
          <YAxis className="text-xs fill-muted-foreground" />
          <Tooltip
            contentStyle={{
              backgroundColor: "var(--background)",
              border: "1px solid var(--border)",
              borderRadius: "var(--radius)",
            }}
            formatter={(value) => [`$${Number(value).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`, ""]}
          />
          <Line
            type="monotone"
            dataKey="income"
            name="Income"
            stroke="hsl(var(--primary))"
            strokeWidth={2}
            dot={{ fill: "hsl(var(--primary))" }}
          />
          <Line
            type="monotone"
            dataKey="expenses"
            name="Expenses"
            stroke="hsl(var(--destructive))"
            strokeWidth={2}
            dot={{ fill: "hsl(var(--destructive))" }}
          />
        </RechartsLineChart>
      </ResponsiveContainer>
    </div>
  )
}