"use client"

import { Moon, Sun } from "lucide-react"
import { Switch } from "@/components/ui/switch"
import { useTheme } from "next-themes"
import { useEffect, useState } from "react"

export function Header() {
  const { theme, setTheme } = useTheme()
  const [mounted, setMounted] = useState(false)
  
  // Ensure we only render after mounting to avoid hydration mismatch
  useEffect(() => {
    setMounted(true)
  }, [])

  const isDarkMode = theme === "dark"

  const toggleTheme = () => {
    setTheme(isDarkMode ? "light" : "dark")
  }

  return (
    <header className="flex h-16 items-center justify-between border-b bg-background px-6">
      <div className="flex items-center gap-3">
        <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary">
          <span className="text-primary-foreground font-bold text-sm">D</span>
        </div>
        <h1 className="text-xl font-semibold">Payment Tracker</h1>
      </div>
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <Sun className="h-4 w-4" />
          {mounted ? (
            <Switch 
              checked={isDarkMode} 
              onCheckedChange={toggleTheme} 
              aria-label="Toggle dark mode"
            />
          ) : (
            <div className="h-[18.4px] w-[32px] rounded-full bg-input" /> // placeholder
          )}
          <Moon className="h-4 w-4" />
        </div>
      </div>
    </header>
  )
}