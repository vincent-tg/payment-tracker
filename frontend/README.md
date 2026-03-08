# Dashboard - Next.js Migration

This is a migrated version of the original Vite React dashboard to Next.js 15 with App Router.

## Migration Summary

### Changes Made:

1. **Project Structure**
   - Created new Next.js 15 project with TypeScript, Tailwind CSS v4, and App Router
   - Migrated from Vite to Next.js build system
   - Updated project structure to use Next.js App Router conventions

2. **Dependencies**
   - Next.js 15 with Turbopack
   - React 19
   - Tailwind CSS v4
   - shadcn/ui components (button, card, switch, table)
   - Recharts for data visualization
   - Lucide React icons
   - next-themes for dark/light mode toggle

3. **Component Migration**
   - All components migrated to use `"use client"` directive where needed
   - Updated import paths to use Next.js alias `@/*`
   - Preserved all functionality: dark/light mode, charts, tables, responsiveness
   - Maintained same UI/UX and styling

4. **Theme System**
   - Implemented `next-themes` for theme management
   - Preserved dark/light mode toggle functionality
   - Updated theme toggle to use Next.js compatible approach

5. **Configuration**
   - Updated `components.json` with slate base color
   - Configured shadcn/ui with same components as original
   - Set up proper TypeScript paths and aliases

### Project Structure:
```
dashboard-next/
├── app/
│   ├── layout.tsx        # Root layout with theme provider
│   └── page.tsx          # Main dashboard page
├── components/
│   ├── Header.tsx        # Header with theme toggle
│   ├── Sidebar.tsx       # Navigation sidebar
│   ├── Layout.tsx        # Main layout wrapper
│   ├── MetricCard.tsx    # Metric cards
│   ├── LineChart.tsx     # Recharts line chart
│   ├── UserTable.tsx     # User data table
│   └── ui/               # shadcn/ui components
├── lib/
│   └── utils.ts          # Utility functions
└── public/               # Static assets
```

### Running the Application:

```bash
# Install dependencies
npm install

# Development server (port 3001)
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

The application runs on http://localhost:3001 to avoid conflict with the original Vite app on port 5173.

### Features Preserved:
- ✅ Dark/light mode toggle
- ✅ All UI components (Header, Sidebar, MetricCards, Charts, Tables)
- ✅ Responsive design
- ✅ Mock data and functionality
- ✅ shadcn/ui styling and theming
- ✅ TypeScript support

The migration successfully preserves all functionality while modernizing the stack to Next.js 15 with improved performance and developer experience.