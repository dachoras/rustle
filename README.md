# Rustle

An optimized Rust + WebAssembly (Yew) Wordle clone with an Axum backend.

## Overview & Rules

Rustle is a lightweight, responsive, and secure clone of the popular Wordle game built using Rust, Yew, and WebAssembly, served by a native Axum backend.

### Game Rules:
- Guess the **5-letter word** in 6 tries.
- Each guess must be a valid 5-letter word.
- After each guess, the color of the tiles will change to show how close your guess was to the word:
  - **Green (Correct)**: The letter is in the word and in the correct spot.
  - **Yellow (Present)**: The letter is in the word but in the wrong spot.
  - **Gray (Absent)**: The letter is not in the word in any spot.
- **Hard Mode**: Any revealed hints must be used in subsequent guesses.

---

## Native Server (Rust & Axum)

### 1. Prerequisites
Ensure you have the Rust toolchain, WASM target, and Trunk compiler installed:
```bash
# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk compiler
cargo install --locked trunk
```

### 2. Build Frontend & Start Axum Server
To compile the frontend and start the backend server natively on port `4409`:
```bash
# 1. Compile the WASM frontend assets into the /dist directory
trunk build --release

# 2. Start the Axum server to serve the assets
cargo run --release --bin server
```
Open [http://localhost:4409](http://localhost:4409) in your browser to play.

---

## Docker Configuration

### Run with Docker Compose

To quickly run Rustle using Docker Compose, create a `docker-compose.yml` file with the following configuration:

```yaml
version: "3.8"

services:
  rustle:
    image: ubermetroid/rustle:latest
    container_name: rustle
    restart: unless-stopped
    ports:
      - 4409:4409
```

Run the container:
```bash
docker compose up -d
```

### Build and Run with Dockerfile

If you want to build and run the image locally:

1. **Build the production image**:
   ```bash
   docker build -t ubermetroid/rustle:latest -f Dockerfile .
   ```

2. **Run the container**:
   ```bash
   docker run -d -p 4409:4409 --name rustle ubermetroid/rustle:latest
   ```

Open [http://localhost:4409](http://localhost:4409) in your browser to play.

---

## File Tree

```text
rustle/
├── Cargo.toml                  # Cargo dependencies & release optimization profile
├── Trunk.toml                  # WebAssembly build tool configuration
├── index.html                  # HTML entry point injecting CSS/WASM target
├── tailwind.config.js          # TailwindCSS configuration rules
├── index.css                   # Core styling overrides
├── Dockerfile                  # Multi-stage WASM + Axum Docker build
└── src/
    ├── main.rs                 # Bootstraps Yew client to DOM
    ├── app.rs                  # Layout view coordinator
    ├── app_state.rs            # Game state reducers machine
    ├── app_effects.rs          # Side effects custom hook
    ├── constants.rs            # Constants module index
    ├── bin/
    │   └── server.rs           # Native Axum backend server
    ├── constants/
    │   ├── config.rs           # Game settings & localization text
    │   └── word_db.rs          # O(log N) binary search database
    ├── components/
    │   ├── mod.rs              # UI components index
    │   ├── alerts.rs           # Toast style event alerts
    │   ├── grid.rs             # Cell tiles container grid
    │   ├── keyboard.rs         # Virtual key inputs listener & styling
    │   ├── navbar.rs           # Navigation header controls
    │   ├── stat_bar.rs         # Streaks/Tries dashboard indicators
    │   ├── stat_histogram.rs   # Guess distributions chart
    │   ├── app_modals.rs       # Coordination backdrop wrapper
    │   ├── modal_base.rs       # Reusable overlay layout
    │   ├── modal_info.rs       # Instructions & rules modal
    │   ├── modal_settings.rs   # Difficulty, contrast, dark-mode settings
    │   ├── modal_stats.rs      # Win histograms, count-downs, share triggers
    │   ├── modal_date_picker.rs# Historical date picker dialog
    │   └── modal_migrate.rs    # User profile transfers encryption coder
    └── helpers/
        ├── mod.rs              # Helper module index
        ├── browser.rs          # Embedded browser client checking
        ├── encryption.rs       # Blowfish cryptology coder
        ├── local_storage.rs    # Browser local storage interface
        ├── share.rs            # Social copy-to-clipboard formatters
        ├── stats.rs            # Streak calculators
        ├── statuses.rs         # Letter placement evaluation engine
        └── words.rs            # Solution lookups & game calendars
```
