# 📖 Game Design Document — EUROPA DOMINION

**Version:** 0.1  
**Status:** Draft  
**Author:** AlexanderMaluche  

---

## 1. Vision

EUROPA DOMINION is a turn-based business strategy simulation game.  
The player builds and manages a multinational corporation across 10 European markets.  
Every decision is grounded in real regulatory, economic, and labour market data.

---

## 2. Core Concept

- **Genre:** Turn-based strategy / business simulation  
- **Platform:** PC (CLI first, GUI planned)  
- **Engine:** Rust core + Python economic models  
- **Players:** 1–4 (multiplayer planned Phase 6)  

---

## 3. Game Mechanics

### 3.1 Turn Structure
Each turn represents one fiscal quarter (3 months).

1. **Survey Phase** — Review market data, economic indicators, competitor moves  
2. **Decision Phase** — Invest, hire, expand, withdraw, lobby  
3. **Resolution Phase** — Engine calculates outcomes based on all inputs  
4. **Report Phase** — Financial report, KPIs, board pressure  

### 3.2 Core Actions per Turn
| Action | Cost | Effect |
|---|---|---|
| Enter new market | High capital | Opens new revenue stream |
| Expand operations | Medium capital | Increases revenue & headcount |
| Hire specialists | Ongoing salary | Reduces compliance risk |
| Lobby regulators | Medium capital | Lowers regulatory complexity |
| Withdraw from market | Low cost | Stops losses, damages reputation |

---

## 4. Economic System

### 4.1 Revenue Formula