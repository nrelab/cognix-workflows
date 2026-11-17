# @cognix/workflows (npm package)

This directory is a **separate npm package** built from the workflow specs in [`../specs/`](../specs/). It is published independently of the Rust crate in [`../workflows/`](../workflows/).

## What it does

`index.ts` uses webpack's `require.context` to dynamically load every YAML spec in `../specs/` and builds a `WORKFLOWS` map keyed by workflow slug. The output is a UMD bundle (`dist/cognix-workflows.js`) consumed by the commands.dev web frontend and other JS consumers.

## Relationship to the Rust crate

| Artifact | Path | Version | Consumer | Build |
|----------|------|---------|----------|-------|
| npm package | `build_ts/` | `1.0.10` (see `package.json`) | commands.dev / web | `npm run build` (webpack + tsc) |
| Rust crate | `workflows/` | `0.1.0` (see `Cargo.toml`) | Cognix desktop app | `cargo build` (build.rs codegen) |

Both are generated from the same source-of-truth: the YAML files in `../specs/`. They are **versioned and released independently** because they serve different consumers with different release cadences.

## Developing

```bash
# Install dependencies
npm install

# Build the bundle and TypeScript declarations
npm run build   # runs tsc && webpack

# Outputs:
#   dist/cognix-workflows.js   (UMD bundle, package main)
#   dist/index.js
#   dist/index.d.ts            (TypeScript declarations)
```

## Releasing

The npm package is published by [`.github/workflows/publish.yaml`](../.github/workflows/publish.yaml) on pushes to `main` that touch `build_ts/*` or `specs/*`. It runs:

1. `npm --prefix build_ts/ install`
2. `npm --prefix build_ts/ run build`
3. Automated version bump (via `gh-action-bump-version`)
4. `npm publish` (requires `NPM_TOKEN` secret)

The version bump is driven by `build_ts/package.json`. Bump that version manually (or let the CI action do it) before a release.
