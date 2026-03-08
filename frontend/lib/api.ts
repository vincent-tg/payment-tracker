// API client for payment tracker backend

const API_BASE = "/api"; // proxied via Next.js rewrites to localhost:8080

export interface Transaction {
  id: number;
  date: string;
  description: string;
  amount: number;
  currency: string;
  type: "in" | "out";
  source: string;
  bank: string;
  amount_usd: number;
  formatted_amount: string;
  transaction_id: string | null;
  email_message_id: string | null;
  created_at: string;
}

export interface Summary {
  period: string;
  total_transactions: number;
  total_in: number;
  total_out: number;
  net_balance: number;
  top_categories: Array<{ category: string; amount: number }>;
}

export interface FetchResult {
  emails_found: number;
  parsed: number;
  saved: number;
  skipped: number;
  errors: string[];
}

export interface CurrencyInfo {
  currencies: string[];
}

export interface CreateTransactionRequest {
  amount: number;
  description: string;
  type: "in" | "out";
  date?: string;
  currency?: string;
  bank?: string;
}

export async function getTransactions(params?: {
  type?: "in" | "out";
  from?: string;
  to?: string;
  limit?: number;
}): Promise<Transaction[]> {
  const query = new URLSearchParams();
  if (params?.type) query.set("type", params.type);
  if (params?.from) query.set("from", params.from);
  if (params?.to) query.set("to", params.to);
  if (params?.limit) query.set("limit", params.limit.toString());

  const res = await fetch(`${API_BASE}/transactions?${query}`);
  if (!res.ok) throw new Error(`Failed to fetch transactions: ${res.statusText}`);
  const data = await res.json();
  return data.data;
}

export async function getSummary(period: string = "month", date?: string): Promise<Summary> {
  const query = new URLSearchParams({ period });
  if (date) query.set("date", date);
  const res = await fetch(`${API_BASE}/summary?${query}`);
  if (!res.ok) throw new Error(`Failed to fetch summary: ${res.statusText}`);
  const data = await res.json();
  return data.data;
}

export async function createTransaction(req: CreateTransactionRequest): Promise<Transaction> {
  const res = await fetch(`${API_BASE}/transactions`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error(`Failed to create transaction: ${res.statusText}`);
  const data = await res.json();
  return data.data;
}

export async function fetchEmails(): Promise<FetchResult> {
  const res = await fetch(`${API_BASE}/fetch`, {
    method: "POST",
  });
  if (!res.ok) throw new Error(`Failed to fetch emails: ${res.statusText}`);
  const data = await res.json();
  return data.data;
}

export async function getCurrencies(): Promise<string[]> {
  const res = await fetch(`${API_BASE}/currencies`);
  if (!res.ok) throw new Error(`Failed to fetch currencies: ${res.statusText}`);
  const data = await res.json();
  return data.data.currencies;
}