# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: CLI entry using `clap` (commands: `grayscale`, `halftone`, `gamma`, `invert`).
- `src/lib.rs`: library crate root exposing modules below.
- `src/filters/`: filter modules and `Filter<I, O>` trait in `src/filters.rs`.
  - `grayscale.rs`, `dither.rs` (halftone, grayscale-only), `gamma.rs`, `invert.rs`.
- `src/color.rs`: shared color constants (sRGB luminance weights).
- `src/pipeline.rs`: typed composition utilities for chaining filters.
- `assets/`: sample images used in README examples.
- CI: `.github/workflows/build.yml` builds, tests, lints, and uploads artifacts on PRs/tags.
- Dev container: `.devcontainer/devcontainer.json` preinstalls rustfmt/clippy and VS Code settings.

## Build, Test, and Development Commands
- Build: `cargo build` (release: `cargo build --release`).
- Run CLI: `cargo run -- --input assets/parrot.jpg --output /tmp/out.jpg grayscale`.
- Tests: `cargo test` (unit tests live next to code via `#[cfg(test)]`).
- Format: `cargo fmt --all` (CI enforces `-- --check`).
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`.

## Coding Style & Naming Conventions
- Rust 2021 edition; 4-space indentation; keep lines concise and readable.
- Names: modules/functions `snake_case`; types/traits `UpperCamelCase`; constants `SCREAMING_SNAKE_CASE`.
- Public API: favor the lib crate for reusable code; add `///` docs for public items and CLI flags.
- Use `rustfmt` defaults; fix all clippy warnings before pushing.

## Testing Guidelines
- Prefer deterministic, small in-memory images via `ImageBuffer::from_fn` (no file IO).
- Test names: `test_*`; group by module; assert dimensions and pixel values.
- Add tests for pipelines when composing filters; verify type compatibility and results.
- Run `cargo test` locally; cover edge cases (e.g., invalid gamma, weight sums).

## Commit & Pull Request Guidelines
- Commits: follow Conventional Commits (e.g., `feat(filters): add sepia filter`, `fix(deps): bump clap`). Project history uses `fix(deps)`, `chore(deps)`.
- PRs: include description, rationale, and linked issues; show before/after output if behavior changes (attach sample images when helpful).
- Requirements: pass CI; run `cargo fmt` and `cargo clippy` locally; update README/help text for new CLI options; note that halftone outputs grayscale.

## Security & Configuration Tips
- Validate user inputs (paths, numeric ranges like `gamma > 0`); keep error messages clear.
- Avoid panics in library-like code paths; prefer `Result` with context.
