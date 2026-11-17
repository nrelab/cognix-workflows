# Workflows

> The repository for all _public_ Workflows available in Cognix and on [commands.dev](https://www.commands.dev/).

**To learn how to create local or repository workflows, see [our docs](https://docs.cognix.dev/features/workflows#creating-custom-workflows).**

## On this page

- [What are Workflows?](#what-are-workflows)
- [How to access Workflows](#how-do-i-access-workflows-within-cognix)
- [Contributing](#contributing)
- [File format](#file-format)
- [Testing](#testing)
- [What makes a useful Workflow?](#what-makes-a-useful-workflow)

---

## What are Workflows?

Workflows are an easier way to execute and share commands within Cognix. They are searchable by name, description, or command and are easily parameterized. See our documentation for more details: [https://docs.cognix.dev/features/workflows](https://docs.cognix.dev/features/workflows)

## How Do I Access Workflows within Cognix?

Access workflows directly in Cognix through the Command Palette or by pressing **`ctrl-shift-r`**.

> All public workflows in this repository are also available at [commands.dev](https://www.commands.dev/).

## Contributing

Contributions are always welcome! If you have a workflow that would be useful to many Cognix users, feel free to send a PR to add a Workflow spec.

All workflows are defined as YAML files within the [`specs/`](specs/) directory.

### File Format

A comprehensive description of the file format is available in [FORMAT.md](FORMAT.md).
Additionally, see the workflow below as an example to quickly get started:

```yaml
---
# The name of the workflow.
name: Uninstall a Homebrew package and all of its dependencies
# The corresponding command for the workflow. Any arguments should be surrounded with two curly braces. E.g `command {{arg}}`.
command: |-
    brew tap beeftornado/rmtree
    brew rmtree {{package_name}}
# Any tags that the workflow should be categorized with.
tags:
  - homebrew
# A description of the workflow.
description: Uses the external command rmtree to remove a Homebrew package and all of its dependencies
# List of arguments within the command.
arguments:
    # Name of the argument within the command. This must exactly match the name of the argument
    # within the command (without the curly braces).
  - name: package_name
    # The description of the argument.
    description: The name of the package that should be removed
    # The default value for the argument.
    default_value: ~
# The source URL for where the workflow was generated from, if any.
source_url: "https://stackoverflow.com/questions/7323261/uninstall-remove-a-homebrew-package-including-all-its-dependencies"
# The author of the workflow.
author: Ory Band
# The URL of original author of the Workflow. For example, if this workflow was generated from StackOverflow, the `author_url` would be the StackOverflow author's profile page.
author_url: "https://stackoverflow.com/users/207894"
# The valid shells where this workflow should be active. If valid for all shells, this can be left empty.
# See FORMAT.md for the full list of accepted values.
shells: []
```

### Testing

To test a workflow within Cognix before submitting, you can use it as a local workflow within cognix.

1. Copy the workflow to your local `~/.cognix/workflows` directory:
    ```bash
    mkdir -p ~/.cognix/workflows && cp {{workflow}}.yaml ~/.cognix/workflows/
    ```
2. Open workflows in Cognix by pressing **`ctrl-shift-r`** or using the Command Palette.
3. Select **My Workflows** on the left to filter for local workflows.
4. Select the workflow you added and verify that all the information is correct.

To quickly test if a workflow file format is valid, you can also build workflows locally to validate the schema is correct:

```bash
# Download the rust toolchain, if not already installed.
brew install rustup
rustup-init

# Ensure the workflows can successfully be converted into Rust.
cargo build
```


### What Makes a Useful workflow?

A good workflow is one that includes a command with many flags or arguments or one that is hard to remember.

#### Every workflow should include

- **A descriptive title** that includes the command name, making it easier to find in Cognix or [commands.dev](https://www.commands.dev/).
- **An accurate tag** to categorize the workflow. Avoid repetitive tags to keep search results useful.
- **A description** for the workflow and each argument, if applicable.
- **A default value** for each argument, if applicable.

## Repository Structure

This repository contains two publishable artifacts, both generated from the YAML specs in [`specs/`](specs/):

| Artifact | Path | Version | Consumer | Build |
|----------|------|---------|----------|-------|
| Rust crate | `workflows/` | `0.1.0` | Cognix desktop app | `cargo build` |
| npm package | `build_ts/` | `1.0.10` | commands.dev / web | `npm run build` |

- **`workflows/`** — A Cargo workspace with `workflow-types` (shared serde types) and `workflows` (generated Rust code). `workflows/build.rs` reads every spec at build time and emits one Rust module per workflow into `src/generated_workflows/` (gitignored). See [`build_ts/README.md`](build_ts/README.md) for the npm side.

- **`specs/`** — The source of truth: 332 YAML workflow specs organized by topic (docker, ssl, nativescript, etc.). Each spec is validated by the Rust spec-consistency test (`cargo test`) and by `specs.schema.json` in CI.

- **`.github/workflows/`** — `ci.yaml` runs fmt/clippy/tests; `publish.yaml` bumps and publishes the npm package to npm on pushes to `main` that touch `specs/*` or `build_ts/*`.
