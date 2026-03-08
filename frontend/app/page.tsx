"use client"

import { useState, useEffect } from "react"
import { DollarSign, TrendingUp, ArrowUp, ArrowDown, RefreshCw } from "lucide-react"
import { Layout } from "@/components/Layout"
import { MetricCard } from "@/components/MetricCard"
import { IncomeExpenseChart } from "@/components/IncomeExpenseChart"
import { TransactionTable } from "@/components/TransactionTable"
import { AddTransactionForm } from "@/components/AddTransactionForm"
import { FetchEmailsButton } from "@/components/FetchEmailsButton"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { getSummary, Summary } from "@/lib/api"

export default function Home() {
  const [summary, setSummary] = useState<Summary | null>(null)
  const [loading, setLoading] = useState(true)
  const [refreshKey, setRefreshKey] = useState(0)

  useEffect(() => {
    loadSummary()
  }, [refreshKey])

  async function loadSummary() {
    try {
      setLoading(true)
      const data = await getSummary("month")
      setSummary(data)
    } catch (err) {
      console.error("Failed to load summary", err)
    } finally {
      setLoading(false)
    }
  }

  const handleRefresh = () => {
    setRefreshKey((prev) => prev + 1)
  }

  return (
    <Layout>
      <div className="space-y-6">
        <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
          <div>
            <h2 className="text-2xl font-bold">Financial Dashboard</h2>
            <p className="text-muted-foreground">Track your income and expenses.</p>
          </div>
          <div className="flex flex-wrap gap-2">
            <FetchEmailsButton onSuccess={handleRefresh} />
            <Button variant="outline" size="sm" onClick={handleRefresh}>
              <RefreshCw className="mr-2 h-4 w-4" />
              Refresh
            </Button>
          </div>
        </div>

        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
          <MetricCard
            value={summary ? `$${summary.total_in.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}` : "$0.00"}
            label="Total Income"
            icon={ArrowUp}
          />
          <MetricCard
            value={summary ? `$${summary.total_out.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}` : "$0.00"}
            label="Total Expenses"
            icon={ArrowDown}
          />
          <MetricCard
            value={summary ? `$${summary.net_balance.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}` : "$0.00"}
            label="Net Balance"
            icon={DollarSign}
          />
          <MetricCard
            value={summary ? summary.total_transactions.toString() : "0"}
            label="Total Transactions"
            icon={TrendingUp}
          />
        </div>

        <div className="grid gap-6 lg:grid-cols-2">
          <Card>
            <CardHeader>
              <CardTitle>Income vs Expenses Over Time</CardTitle>
            </CardHeader>
            <CardContent>
              <IncomeExpenseChart />
            </CardContent>
          </Card>
          <AddTransactionForm onSuccess={handleRefresh} />
        </div>

        <Card>
          <CardHeader>
            <CardTitle>Recent Transactions</CardTitle>
          </CardHeader>
          <CardContent>
            <TransactionTable />
          </CardContent>
        </Card>
      </div>
    </Layout>
  )
}