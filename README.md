# 🌍 EUROPA DOMINION

> **Ultra-Realistic European Business Expansion Strategy Simulation Game**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Language: Python](https://img.shields.io/badge/Language-Python-blue.svg)](https://www.python.org/)
[![Status: In Development](https://img.shields.io/badge/Status-In%20Development-green.svg)]()

---

## 🎯 Overview

**EUROPA DOMINION** is a turn-based business strategy simulation game set across 10 European markets. Players build, expand, and manage a multi-national corporation — navigating real regulatory environments, macroeconomic forces, labour markets, and competitive pressure across the continent.

This is not a casual game. Every decision has consequences modelled on real European economic data.

---

## 🗺️ European Markets

| 🏳️ Country | Market Size | Regulatory Complexity | Labour Cost Index | Purchasing Power |
|---|---|---|---|---|
| 🇩🇪 Germany | ⭐⭐⭐⭐⭐ | High | High | Very High |
| 🇫🇷 France | ⭐⭐⭐⭐⭐ | Very High | High | High |
| 🇬🇧 United Kingdom | ⭐⭐⭐⭐⭐ | Medium | High | High |
| 🇵🇱 Poland | ⭐⭐⭐ | Medium | Low | Medium |
| 🇪🇸 Spain | ⭐⭐⭐⭐ | High | Medium | Medium |
| 🇮🇹 Italy | ⭐⭐⭐⭐ | Very High | Medium | Medium |
| 🇳🇱 Netherlands | ⭐⭐⭐ | Low | High | High |
| 🇸🇪 Sweden | ⭐⭐⭐ | Low | Very High | Very High |
| 🇨🇭 Switzerland | ⭐⭐ | Low | Very High | Very High |
| 🇦🇹 Austria | ⭐⭐ | Medium | High | High |

---

## 🏗️ Tech Stack

| Layer | Technology |
|---|---|
| 🦀 **Core Engine** | Rust — simulation loop, game state, turn logic |
| 🐍 **Economic Models** | Python — macroeconomic simulation, market data |
| 🗃️ **Regulatory Data** | JSON — per-country rule sets and compliance data |
| ⚙️ **CI/CD** | GitHub Actions — automated build, test, lint |
| ☁️ **Infrastructure** | Terraform + Kubernetes (planned) |

---

## 📁 Project Structure

```
europa-dominion/
├── src/                        # Rust core engine
│   ├── main.rs                 # Entry point
│   ├── game.rs                 # Game state & turn logic
│   ├── countries.rs            # Country data & market models
│   ├── player.rs               # Player entity & actions
│   └── economy.rs              # Economic simulation core
├── economic-models/            # Python macroeconomic models
│   ├── macro_model.py          # Base macroeconomic simulation
│   ├── inflation.py            # Inflation & interest rate models
│   └── labour_market.py        # Labour cost & unemployment models
├── data/
│   └── countries/              # Per-country regulatory JSON data
│       ├── germany.json
│       ├── france.json
│       ├── uk.json
│       └── ...
├── docs/
│   └── GDD.md                  # Game Design Document
├── .github/
│   └── workflows/
│       └── ci.yml              # GitHub Actions CI/CD pipeline
├── devops/
│   ├── terraform/              # Infrastructure as Code
│   └── k8s/                    # Kubernetes manifests
├── Cargo.toml                  # Rust project manifest
├── .gitignore
├── LICENSE
└── README.md
```

---

## 🎮 Gameplay

### Core Loop
1. **Survey** — Analyse European markets, regulatory environments, and economic indicators
2. **Invest** — Allocate capital to enter new markets or expand existing operations
3. **Operate** — Manage employees, comply with local regulations, optimise costs
4. **Compete** — React to competitor moves and market disruptions
5. **Report** — End-of-turn financial reporting, KPIs, and board pressure

### Key Mechanics
- 📊 **Dynamic Economic Model** — GDP growth, inflation, and interest rates affect your margins every turn
- ⚖️ **Regulatory Compliance** — Each country has unique compliance costs and legal risks
- 👷 **Labour Markets** — Hire, train, and manage workers under country-specific labour laws
- 🏦 **Capital Management** — Loans, equity, dividends, and cash flow under pressure
- 🌐 **Geopolitical Events** — Random events: trade wars, elections, economic crises

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) >= 1.75
- [Python](https://www.python.org/) >= 3.11
- [Git](https://git-scm.com/)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/AlexanderMaluche/europa-dominion.git
cd europa-dominion

# Build the Rust core engine
cargo build --release

# Run the game
cargo run

# Run all tests
cargo test
```

---

## 🗺️ Roadmap

| Phase | Status | Description |
|---|---|---|
| **Phase 1** | 🔨 In Progress | Core Rust engine, country data, basic turn loop |
| **Phase 2** | 📋 Planned | Python economic models, dynamic market simulation |
| **Phase 3** | 📋 Planned | Full regulatory compliance system per country |
| **Phase 4** | 📋 Planned | AI competitors, geopolitical events |
| **Phase 5** | 📋 Planned | UI/Frontend, save/load system |
| **Phase 6** | 📋 Planned | Multiplayer support |

---

## 🤝 Contributing

Contributions are welcome! Please read the contributing guidelines (coming soon) and open a pull request.

---

## 📜 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

> *"Europe is not a problem to be solved. It is a promise to be fulfilled."*
> — EUROPA DOMINION, 2026