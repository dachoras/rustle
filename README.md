# Rustle - Word Guessing Game

<p align="center">
  <img src="https://raw.githubusercontent.com/UberMetroid/unraid-templates/main/icons/rustle.png" alt="Rustle Logo" width="128" height="128">
</p>

Rustle is a clean, secure, and optimized word guessing game (Wordle clone) built in Rust and WebAssembly, served by a high-performance Axum backend.

---

## Key Features

*   **Standardized UI Alignment**: Completely integrated with `shared-assets` for a uniform theme engine, navigation header, footer, and authentication layout.
*   **Super Metroid Atmospheric Environments**: Selectable map location themes (Crateria, Brinstar, Norfair, Wrecked Ship, Maridia, Tourian) with custom color schemes and interactive ambient weather particle effects.
*   **Classic Gameplay Rules**: Standard Wordle guess validation, semantic green/yellow grid cell styling, high-contrast settings, and an optional hard-mode toggle.
*   **Secure PIN Access**: Optional lock screen gate with client IP rate-limiting, timing-attack protections, and session cookie validation.
*   **Performance First**: Tiny resource footprint, zero external JS engine dependencies, and rapid page load speeds.

---

## Container Registry

The Docker image is built with **Nix** (no Alpine, fully reproducible) and published to Docker Hub:

*   **Docker Hub**: [ubermetroid/rustle](https://hub.docker.com/r/ubermetroid/rustle)

---

## Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4502` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Rustle` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies to ensure redirect and websocket links are resolved correctly. | `http://localhost:4502` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). Use `*` to allow all origins. | `*` |
| `RUSTLE_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. Leave empty for public mode. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header (true/false). | `false` |
| `ENABLE_THEMES` | Enable the Super Metroid theme selector in the navigation header (true/false). | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header (true/false). | `false` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before locking out the user client IP address. | `5` |
| `LOCKOUT_TIME_MINUTES` | Lockout duration in minutes for IPs exceeding `MAX_ATTEMPTS`. | `15` |
| `COOKIE_MAX_AGE_HOURS` | Duration in hours that the user's PIN session cookie remains valid. | `24` |
| `SHUTDOWN_DRAIN_SECONDS` | Seconds to wait for active connections to finish before shutting down. | `5` |
| `SHOW_VERSION` | Display the application version number in the footer (true/false). | `true` |
| `SHOW_GITHUB` | Display the GitHub repository link in the footer (true/false). | `true` |
| `TRUST_PROXY` | Set `true` if backend is hosted behind a reverse proxy. | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated IP/CIDR list of trusted upstream proxies. | None |
