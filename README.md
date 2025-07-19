# Christmas Gift Exchange

A Rust application for organizing Christmas gift exchanges with both CLI and web UI interfaces.

## Features

- Organize gift exchanges for multiple pools (Island Life, Grabergishimazureson, Pets)
- Respect exclusion rules (people who shouldn't give to each other)
- Generate unique year letters for tracking
- Store exchange history in SQLite database
- Beautiful web UI for viewing pairings

## Running the Application

### Web UI Mode

To run the interactive web interface:

```bash
cargo run
```

This will start a local web server. Open your browser to `http://localhost:8080` to view the gift exchange interface.

### CLI Mode

To generate exchanges via command line:

```bash
# Generate Island Life exchange
cargo run -- island

# Generate Grabergishimazureson exchange
cargo run -- graber

# Generate Pets exchange
cargo run -- pets
```

The CLI mode will:
- Generate the exchange pairings
- Display them in the terminal
- Save them to the SQLite database
- Show the year's letter code

## Building for Production

To build the web application for production:

```bash
# Install dioxus CLI if you haven't already
cargo install dioxus-cli

# Build the web app
dx build --release
```

The built files will be in the `dist/` directory.

## Database

Exchange history is stored in `drawings.db` (SQLite). This file is created automatically on first run.

## Development

### Prerequisites

- Rust (latest stable)
- For web development: `cargo install dioxus-cli`

### Project Structure

- `src/main.rs` - Main application entry point and CLI logic
- `src/ui.rs` - Dioxus web UI components
- `src/giftexchange.rs` - Exchange pool definitions
- `src/persist.rs` - Database persistence layer
- `index.html` - Web app HTML template