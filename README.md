# Rustle — Word Guessing Game <img src="https://raw.githubusercontent.com/UberMetroid/unraid-templates/main/icons/rustle.png" width="48" height="48" alt="rustle logo" align="right">

Rustle is a clean, secure, and optimized word guessing game (Wordle clone) built in Rust and WebAssembly, served by a high-performance Axum backend.

---

## 🏛️ Architecture & Stack
*   **Frontend**: Yew (WASM)
*   **Backend**: Axum (Rust) / Tokio
*   **Deployment**: UBI container (Red Hat UBI9) on Docker Hub / Unraid / Podman / Docker Compose

---

## 🟢 Key Features
*   **Standardized UI Alignment**: Completely integrated with `shared-assets` for a uniform theme engine, navigation header, footer, and authentication layout.
*   **Super Metroid Atmospheric Environments**: Selectable map location themes (Crateria, Brinstar, Norfair, Wrecked Ship, Maridia, Tourian) with custom color schemes and interactive ambient weather particle effects.
*   **Classic Gameplay Rules**: Standard Wordle guess validation, semantic green/yellow grid cell styling, high-contrast settings, and an optional hard-mode toggle.
*   **Secure PIN Access**: Optional lock screen gate with client IP rate-limiting, timing-attack protections, and session cookie validation.
*   **Performance First**: Tiny resource footprint, zero external JS engine dependencies, and rapid page load speeds.

---

## 💾 Deployment & Installation

### Container images (Docker Hub)

Images are **UBI9-minimal** based (Red Hat Universal Base Image). Tags:

| Tag | Meaning |
| :--- | :--- |
| `latest` | Current recommended build |
| `ubi` | Explicit UBI image (same lineage as `latest`) |
| `0.1.36` | Immutable release pin |

```bash
# Pull examples
podman pull docker.io/ubermetroid/rustle:latest
podman pull docker.io/ubermetroid/rustle:ubi
podman pull docker.io/ubermetroid/rustle:0.1.36
```

Hub: [https://hub.docker.com/r/ubermetroid/rustle](https://hub.docker.com/r/ubermetroid/rustle)

### Docker Compose
Create a `docker-compose.yml` file with the following service definition:

```yaml
services:
  rustle:
    image: ubermetroid/rustle:latest
    container_name: rustle
    restart: unless-stopped
    ports:
      - ${RUSTLE_PORT:-4502}:4502
    environment:
      PORT: 4502
      SITE_TITLE: ${RUSTLE_SITE_TITLE:-Rustle}
      RUSTLE_PIN: ${RUSTLE_PIN:-}
      BASE_URL: ${RUSTLE_BASE_URL:-http://localhost:4502}
      ALLOWED_ORIGINS: ${RUSTLE_ALLOWED_ORIGINS:-*}
      TZ: ${TZ:-UTC}
      ENABLE_TRANSLATION: ${ENABLE_TRANSLATION:-false}
      MAX_ATTEMPTS: ${MAX_ATTEMPTS:-5}
```

### Build the UBI image locally

Requires [Podman](https://podman.io/) (or Docker) and network access to pull base images and crates.

```bash
# From the repository root
podman build --format docker -f Containerfile.ubi \
  -t docker.io/ubermetroid/rustle:0.1.36 \
  -t docker.io/ubermetroid/rustle:latest \
  -t docker.io/ubermetroid/rustle:ubi \
  .

# Optional: push all three tags
podman push docker.io/ubermetroid/rustle:0.1.36
podman push docker.io/ubermetroid/rustle:latest
podman push docker.io/ubermetroid/rustle:ubi
```

---

## ⚙️ Configuration Options

| Environment Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4502` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Rustle` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies. | `http://localhost:4502` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). | `*` |
| `RUSTLE_PIN` | Optional 4–10 digit numerical PIN to lock access to the interface. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header. | `false` |
| `ENABLE_THEMES` | Enable the Super Metroid theme selector in the navigation header. | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header. | `false` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before rate lockout. | `5` |
| `LOCKOUT_TIME_MINUTES` | Lockout duration in minutes for IPs exceeding `MAX_ATTEMPTS`. | `15` |
| `COOKIE_MAX_AGE_HOURS` | Duration in hours that the user's PIN session cookie remains valid. | `24` |
| `SHUTDOWN_DRAIN_SECONDS` | Seconds to wait for active connections to finish before shutting down. | `5` |
| `SHOW_VERSION` | Display the application version number in the footer. | `true` |
| `SHOW_GITHUB` | Display the GitHub repository link in the footer. | `true` |
| `TRUST_PROXY` | Set `true` if backend is hosted behind a reverse proxy. | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated IP/CIDR list of trusted upstream proxies. | None |

---

## 🛠️ Local Development

Ensure you have the Rust toolchain and Trunk installed.

```bash
# 1. Run workspace tests
cargo test

# 2. Run clippy workspace checks
cargo clippy --workspace --all-targets

# 3. Start frontend Yew dev server (from frontend/)
cd frontend && trunk serve

# 4. Start backend Axum server (from backend/)
cd backend && cargo run
```

---

## 📄 License
Licensed under the [Apache License, Version 2.0](LICENSE). Copyright 2026 UberMetroid.
