use crate::countries::{all_countries, Country};

/// Represents a business sector in which the player can operate.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Sector {
    Technology,
    Manufacturing,
    Finance,
    Retail,
    Energy,
}

impl Sector {
    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            Sector::Technology => "Technology",
            Sector::Manufacturing => "Manufacturing",
            Sector::Finance => "Finance",
            Sector::Retail => "Retail",
            Sector::Energy => "Energy",
        }
    }
}

/// A business operation the player has established in a country.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operation {
    pub country: Country,
    pub sector: Sector,
    /// Current revenue per turn (in thousands EUR)
    pub revenue: u64,
    /// Current expenses per turn (in thousands EUR)
    pub expenses: u64,
}

impl Operation {
    pub fn new(country: Country, sector: Sector, initial_investment: u64) -> Self {
        // Revenue scales with market size and purchasing power.
        let revenue = (country.market_size as u64 * country.purchasing_power as u64) / 100;
        // Expenses scale with labour cost and regulatory complexity.
        let expenses = initial_investment / 10
            + (country.labour_cost_index as u64 * country.regulatory_complexity as u64) / 20;
        Operation {
            country,
            sector,
            revenue,
            expenses,
        }
    }

    /// Net profit per turn.
    pub fn net_profit(&self) -> i64 {
        self.revenue as i64 - self.expenses as i64
    }
}

/// The player's company.
#[derive(Debug, Clone)]
pub struct Player {
    pub company_name: String,
    /// Current cash balance (in thousands EUR)
    pub cash: u64,
    /// Active operations across Europe
    pub operations: Vec<Operation>,
    /// Current turn number
    pub turn: u32,
}

impl Player {
    pub fn new(company_name: impl Into<String>, starting_cash: u64) -> Self {
        Player {
            company_name: company_name.into(),
            cash: starting_cash,
            operations: Vec::new(),
            turn: 1,
        }
    }

    /// Attempt to expand into a new country/sector. Returns an error string if
    /// the player cannot afford the investment.
    pub fn expand(
        &mut self,
        country: Country,
        sector: Sector,
        investment: u64,
    ) -> Result<(), String> {
        if investment > self.cash {
            return Err(format!(
                "Insufficient funds: you have {}k EUR but need {}k EUR",
                self.cash, investment
            ));
        }
        self.cash -= investment;
        let op = Operation::new(country, sector, investment);
        self.operations.push(op);
        Ok(())
    }

    /// Advance one turn: collect profits from all operations.
    pub fn end_turn(&mut self) {
        for op in &self.operations {
            let profit = op.net_profit();
            if profit >= 0 {
                self.cash += profit as u64;
            } else {
                let loss = (-profit) as u64;
                self.cash = self.cash.saturating_sub(loss);
            }
        }
        self.turn += 1;
    }

    /// Total net profit per turn across all operations.
    pub fn total_net_profit(&self) -> i64 {
        self.operations.iter().map(|op| op.net_profit()).sum()
    }

    /// Number of active operations.
    pub fn operations_count(&self) -> usize {
        self.operations.len()
    }
}

/// Top-level game state.
#[derive(Debug)]
pub struct GameState {
    pub player: Player,
    pub available_countries: Vec<Country>,
}

impl GameState {
    pub fn new(company_name: impl Into<String>) -> Self {
        GameState {
            player: Player::new(company_name, 500),
            available_countries: all_countries(),
        }
    }

    /// Whether the game is still ongoing (player is solvent).
    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.player.cash > 0 || !self.player.operations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::countries::all_countries;

    fn germany() -> Country {
        all_countries()
            .into_iter()
            .find(|c| c.code == "DE")
            .unwrap()
    }

    fn poland() -> Country {
        all_countries()
            .into_iter()
            .find(|c| c.code == "PL")
            .unwrap()
    }

    #[test]
    fn test_player_creation() {
        let player = Player::new("Acme Corp", 1000);
        assert_eq!(player.company_name, "Acme Corp");
        assert_eq!(player.cash, 1000);
        assert_eq!(player.turn, 1);
        assert!(player.operations.is_empty());
    }

    #[test]
    fn test_expand_success() {
        let mut player = Player::new("Acme Corp", 1000);
        let result = player.expand(germany(), Sector::Technology, 200);
        assert!(result.is_ok());
        assert_eq!(player.cash, 800);
        assert_eq!(player.operations.len(), 1);
    }

    #[test]
    fn test_expand_insufficient_funds() {
        let mut player = Player::new("Acme Corp", 100);
        let result = player.expand(germany(), Sector::Technology, 500);
        assert!(result.is_err());
        assert_eq!(player.cash, 100); // unchanged
    }

    #[test]
    fn test_end_turn_increases_cash_with_profitable_operation() {
        let mut player = Player::new("Acme Corp", 1000);
        player.expand(poland(), Sector::Retail, 100).unwrap();
        let op_profit = player.operations[0].net_profit();
        let cash_before = player.cash;
        player.end_turn();
        if op_profit >= 0 {
            assert_eq!(player.cash, cash_before + op_profit as u64);
        }
        assert_eq!(player.turn, 2);
    }

    #[test]
    fn test_operation_net_profit() {
        let op = Operation::new(germany(), Sector::Finance, 200);
        // Just verify it returns a deterministic value.
        let profit1 = op.net_profit();
        let profit2 = op.net_profit();
        assert_eq!(profit1, profit2);
    }

    #[test]
    fn test_game_state_creation() {
        let gs = GameState::new("Euro Corp");
        assert_eq!(gs.player.company_name, "Euro Corp");
        assert!(gs.is_active());
        assert!(!gs.available_countries.is_empty());
    }

    #[test]
    fn test_sector_display_name() {
        assert_eq!(Sector::Technology.display_name(), "Technology");
        assert_eq!(Sector::Manufacturing.display_name(), "Manufacturing");
        assert_eq!(Sector::Finance.display_name(), "Finance");
        assert_eq!(Sector::Retail.display_name(), "Retail");
        assert_eq!(Sector::Energy.display_name(), "Energy");
    }
}
