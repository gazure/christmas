use anyhow::Result;

mod data;
mod exchange;
mod giftexchange;
mod ui;
mod utils;

use exchange::ParticipantGraph;


fn build_exchange() -> Result<()> {
    // Get CLI args
    let pool_arg = std::env::args()
        .nth(1)
        .expect("No pool specified. Usage: cargo run <island|graber|pets>");
    
    let pool = utils::parse_pool_arg(&pool_arg).map_err(anyhow::Error::msg)?;

    // Get participants for the specified pool
    let participants = data::get_participants_by_pool(pool);

    // Build the graph and generate the exchange
    let graph = ParticipantGraph::from_participants(participants);
    let exchange = graph.build_exchange();
    
    // Print the exchange pairs
    println!("\nGift Exchange for {}:", pool);
    println!("==========================");
    exchange.iter().for_each(|(sender, receiver)| {
        println!("{} -> {}", sender, receiver);
    });
    
    // Print the letter for this year
    let year = utils::current_year();
    let letter = utils::letter_for_pool(pool);
    println!("\nLetter for {} {}: {}", pool, year, letter);
    
    Ok(())
}

fn main() -> Result<()> {
    // Check if CLI args are provided
    if std::env::args().len() > 1 {
        // Run CLI version
        build_exchange()?;
    } else {
        // Run Dioxus web app
        dioxus::launch(ui::app);
    }
    Ok(())
}