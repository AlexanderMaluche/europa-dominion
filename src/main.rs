mod countries;
mod game;

use game::{GameState, Sector};

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║         EUROPA DOMINION  v0.1.0                     ║");
    println!("║  Ultra-Realistic European Business Expansion Game    ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    let mut gs = GameState::new("Maluche Industries");

    println!("Welcome, CEO of {}!", gs.player.company_name);
    println!("Starting capital: {}k EUR", gs.player.cash);
    println!();

    // Demo: expand into two countries and run a few turns.
    let countries = gs.available_countries.clone();

    println!("Available markets ({} countries):", countries.len());
    for c in &countries {
        println!(
            "  [{code}] {name:<20} market={ms:>3}  reg={rc}  labour={lc}  ppi={pp}",
            code = c.code,
            name = c.name,
            ms = c.market_size,
            rc = c.regulatory_complexity,
            lc = c.labour_cost_index,
            pp = c.purchasing_power,
        );
    }
    println!();

    // Enter Germany – Technology sector.
    let germany = countries.iter().find(|c| c.code == "DE").cloned().unwrap();
    match gs.player.expand(germany, Sector::Technology, 150) {
        Ok(()) => println!("✓ Expanded into Germany (Technology). Remaining cash: {}k EUR", gs.player.cash),
        Err(e) => println!("✗ Expansion failed: {}", e),
    }

    // Enter Poland – Manufacturing sector.
    let poland = countries.iter().find(|c| c.code == "PL").cloned().unwrap();
    match gs.player.expand(poland, Sector::Manufacturing, 80) {
        Ok(()) => println!("✓ Expanded into Poland (Manufacturing). Remaining cash: {}k EUR", gs.player.cash),
        Err(e) => println!("✗ Expansion failed: {}", e),
    }

    println!();
    println!("── Turn-by-turn simulation (3 turns) ──────────────────");
    for _ in 0..3 {
        println!(
            "Turn {turn:>2} │ Cash: {cash:>6}k EUR │ Operations: {ops} │ Net/turn: {profit:+}k EUR",
            turn   = gs.player.turn,
            cash   = gs.player.cash,
            ops    = gs.player.operations_count(),
            profit = gs.player.total_net_profit(),
        );
        gs.player.end_turn();
    }
    println!(
        "Turn {turn:>2} │ Cash: {cash:>6}k EUR │ (end of demo)",
        turn = gs.player.turn,
        cash = gs.player.cash,
    );
}
