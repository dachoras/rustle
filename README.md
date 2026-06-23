# Rustle

An optimized, containerized Rust + WebAssembly (Yew) Wordle clone.

## Container Deployment

### 1. Build Production Image
Build the multi-stage, optimized WASM Docker image using the standalone Tailwind CSS compiler:
```bash
docker build -t ubermetroid/rustle:latest -f docker/Dockerfile .
```

### 2. Run Container
Launch the lightweight Nginx server hosting the WASM app under a secure, non-root user:
```bash
docker run -d -p 4409:4409 --name rustle-game ubermetroid/rustle:latest
```
Open [http://localhost:4409](http://localhost:4409) to play.

## File Tree

```text
rustle/
├── Cargo.toml                  # Cargo dependencies & release optimization profile
├── Trunk.toml                  # WebAssembly build tool configuration
├── index.html                  # HTML entry point injecting CSS/WASM target
├── tailwind.config.js          # TailwindCSS configuration rules
└── src/
    ├── main.rs                 # Bootstraps Yew client to DOM
    ├── app.rs                  # Layout view coordinator
    ├── app_state.rs            # Game state reducers machine
    ├── app_effects.rs          # Side effects custom hook
    ├── constants.rs            # Constants module index
    ├── index.css               # Core styling overrides
    ├── tailwind.css            # Compiled output of tailwind class definitions
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
