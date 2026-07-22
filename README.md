# Fern

### Your money, clearly. Private by design.

A local-first desktop expense tracker for understanding your spending without giving up your data.

[![Release](https://img.shields.io/github/v/release/qutad/Fern?style=flat-square)](https://github.com/qutad/Fern/releases)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?style=flat-square&logo=svelte&logoColor=white)](https://svelte.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-6-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

[Download](https://github.com/qutad/Fera/releases) · [Features](#features) · [Development](#development)

</div>

---

## About

Fern is a private, local-first expense tracker for desktop. It brings transactions, budgets, recurring payments, and financial trends together in a calm and focused interface.

Your financial data is stored in a local SQLite database. Fern requires no account, subscription, or cloud connection.

<!-- Replace this block with a product screenshot when one is available:
<p align="center">
  <img src="docs/images/dashboard.png" alt="Fern dashboard" width="900">
</p>
-->

<p align="center">
  <strong>Product preview coming soon</strong><br>
  <sub>A dashboard screenshot will be added here.</sub>
</p>

## Features

| Feature | Description |
| --- | --- |
| **Dashboard** | Review balances, income, expenses, savings rate, and recent activity. |
| **Transactions** | Add, edit, delete, search, and filter income, expenses, and transfers. |
| **Budgets** | Create monthly category budgets and track remaining spending. |
| **Recurring activity** | Manage subscriptions, bills, and repeating income. |
| **Analytics** | Explore daily spending, category breakdowns, and long-term cash flow. |
| **CSV tools** | Import bank CSV files with column mapping or export your ledger. |
| **Backups** | Create and restore complete database snapshots. |
| **Preferences** | Choose a currency, theme, and preferred first day of the week. |
| **Local storage** | Keep your financial information on your own device. |

## Download

Installers are published through [GitHub Releases](https://github.com/qutad/Fern/releases).

| Platform | Package |
| --- | --- |
| Linux | AppImage |
| Windows | NSIS installer |
| macOS | DMG for Apple Silicon and Intel |

Download the latest package for your operating system and follow its standard installation process.

## Development

### Prerequisites

Before starting, install:

- [Node.js](https://nodejs.org/) LTS
- [Rust](https://www.rust-lang.org/tools/install)
- The [Tauri system dependencies](https://v2.tauri.app/start/prerequisites/) for your operating system

### Setup

Clone the repository and install its dependencies:

```sh
git clone https://github.com/qutad/Fern.git
cd Fern
npm ci
```

Start the desktop application in development mode:

```sh
npm run desktop:dev
```

To run only the Svelte frontend:

```sh
npm run dev
```

## Build

Create a production desktop bundle:

```sh
npm run desktop:build
```

Build only the frontend:

```sh
npm run build
```

Generated desktop packages are written to the Tauri target directory.

## Quality Checks

Run the project checks before submitting changes:

```sh
npm run check
npm run lint
npm test
```

Format the codebase with Prettier:

```sh
npm run format
```

## Technology

| Layer | Technology |
| --- | --- |
| Desktop runtime | [Tauri 2](https://tauri.app/) |
| Interface | [Svelte 5](https://svelte.dev/) and [SvelteKit](https://svelte.dev/docs/kit) |
| Language | [TypeScript](https://www.typescriptlang.org/) |
| Native backend | [Rust](https://www.rust-lang.org/) |
| Database | [SQLite](https://www.sqlite.org/) through `rusqlite` |
| Charts | [Chart.js](https://www.chartjs.org/) |
| Tooling | [Vite](https://vite.dev/), ESLint, Prettier, and Vitest |

## Project Structure

```text
.
├── src/
│   ├── lib/              # Components, charts, API bindings, and shared utilities
│   └── routes/           # Dashboard and financial management pages
├── src-tauri/
│   ├── src/              # Rust commands, database access, and services
│   └── tauri.conf.json   # Desktop application configuration
├── static/               # Static web assets
└── package.json          # Frontend scripts and dependencies
```

## Privacy

Fern is designed around local ownership:

- Financial records are stored on your device.
- No Fern account is required.
- No cloud connection is required.
- Transactions can be exported to CSV.
- The complete database can be backed up and restored.

You remain responsible for securely storing exported files and database backups.


<div align="center">

Built for a clearer view of your money.

</div>
