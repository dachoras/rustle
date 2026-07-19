# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.52] - 2026-07-19

### Changed
- **Rebrand to studio2201**: README, container labels, docker-compose, and Cargo
  metadata now reference `studio2201/rustle`. CI badge URL and GHCR image name
  updated accordingly.
- **Open Graph and Twitter meta tags** replaced placeholder strings
  ("Rustle (Title)", "Rustle (Description)") with real values in
  `frontend/index.html`.
- **Favicon cache-bust query** added `?v=0.1.52` in `frontend/index.html` to
  invalidate stale PWA icon cache and match the Cargo workspace version.
- **Containerfile image description** lowercased to match the other repos
  ("Rustle (UBI9 minimal pilot)" → "rustle (UBI9 minimal pilot)").

## [0.1.0] - 2026-06-22

### Added
- Completed conversion of the application from React/TypeScript to Yew/Rust/WebAssembly.
- Implemented pure Rust Tailwind CSS compiler pipeline (no node_modules or npm dependencies).
- Added unit tests for game logic, local storage, stats persistence, and word lists.
- Dynamically sized the virtual keyboard to occupy exactly 2/3 width and 2/3 of the bottom 2/3 of screen height (`h-[44vh]`).
- Ensured uniform key box sizing across standard and special (`ENTER` / `DELETE`) keys.
- Updated repository workflows and LICENSE file to align with GPL-3.0 copyleft licensing.
