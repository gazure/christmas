# Christmas Gift Exchange Project Structure

This document describes the modular organization of the Christmas gift exchange application.

## Directory Structure

```
christmas/
├── src/
│   ├── main.rs           # Application entry point (CLI and web server)
│   ├── giftexchange.rs   # Core exchange pool enum definition
│   ├── exchange/         # Exchange algorithm modules
│   │   ├── mod.rs        # Module exports
│   │   ├── participant.rs # Participant data structure
│   │   └── graph.rs      # Graph-based exchange algorithm
│   ├── data.rs           # Participant data and pool configurations
│   ├── utils.rs          # Utility functions (letter generation, parsing)
│   ├── ui.rs             # Dioxus web UI components
│   └── persist.rs        # Database persistence (currently unused)
├── Cargo.toml            # Project dependencies
├── index.html            # Web UI template
└── drawings.db           # SQLite database (when persistence is enabled)
```

## Module Descriptions

### `main.rs`
- Entry point for both CLI and web application
- Handles command-line argument parsing
- Routes between CLI exchange generation and Dioxus web UI

### `giftexchange.rs`
- Defines the `ExchangePool` enum (IslandLife, Grabergishimazureson, Pets)
- Implements Display trait for pool names

### `exchange/` module
Contains the core gift exchange algorithm components:

#### `exchange/participant.rs`
- `Participant` struct definition
- Stores participant name, exchange pools, and exclusions

#### `exchange/graph.rs`
- `ParticipantGraph` struct for modeling participant relationships
- Implements Hamiltonian cycle algorithm for optimal gift exchanges
- Includes fallback algorithms for when perfect cycles cannot be found

### `data.rs`
- `get_all_participants()`: Returns the complete participant list
- `get_participants_by_pool()`: Filters participants by exchange pool
- Central location for all participant data

### `utils.rs`
- `letter_for_pool()`: Generates year letters for each pool
- `current_year()`: Gets the current year
- `parse_pool_arg()`: Parses CLI arguments into ExchangePool enum

### `ui.rs`
- Dioxus web application components
- `generate_exchange_pairings()`: Generates exchanges for UI display
- `app()`: Main UI component with pool selection and pairing display

### `persist.rs` (currently unused)
- Database schema and operations for persistent storage
- Can save exchange history and participant data

## Usage

### CLI Mode
```bash
cargo run <pool>
# Where <pool> is one of: island, graber, pets
```

### Web UI Mode
```bash
cargo run
# Opens web interface on http://localhost:8080
```

## Key Design Decisions

1. **Modular Structure**: Separates concerns into distinct modules for maintainability
2. **Graph Algorithm**: Uses Hamiltonian cycle finding for optimal gift distribution
3. **Data Centralization**: All participant data in one place (`data.rs`)
4. **Dual Interface**: Supports both CLI and web UI from the same codebase
5. **Extensibility**: Easy to add new exchange pools or modify participant lists