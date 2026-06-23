# Rustle - Wordle Clone in Rust & WebAssembly

Rustle is an optimized, responsive, and secure clone of the popular Wordle game built using Rust, Yew, and WebAssembly, served by a native Axum backend.

---

## 🐳 Container Installation

### Option 1: Docker Compose (Recommended)

1. Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  rustle:
    image: ubermetroid/rustle:latest
    container_name: rustle
    restart: unless-stopped
    ports:
      - 4409:4409
    environment:
      - PORT=4409
```

2. Run the container:

```bash
docker compose up -d
```

3. Open your browser and navigate to `http://localhost:4409`.

### Option 2: Docker CLI

Run the following command to start the container:

```bash
docker run -d \
  --name rustle \
  --restart unless-stopped \
  -p 4409:4409 \
  ubermetroid/rustle:latest
```

---

## 📋 Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | Local host port mapping for the backend. | `4409` |
