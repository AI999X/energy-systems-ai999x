# Energy Systems (energy-systems-ai999x)

[![tests](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/energy-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/energy-systems-ai999x)](https://pypi.org/project/energy-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/energy-systems-ai999x)

Energy Systems covers power systems and load flow, three-phase power, transformers and machines, solar photovoltaics, wind power, batteries, fuel cells and hydrogen: 83 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Energy Systems desktop app: Incidence angle modifier (ASHRAE), with its worked example and live scene](https://www.ai999x.com/?ax_shot=energy-systems-ai999x/hero.webp)](https://www.ai999x.com/models/energy-systems-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `energy-systems` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/energy-systems-ai999x/#try) |
| **Python** 3.10+ | `pip install energy-systems-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import energy_systems_ai999x as energy_systems
a = energy_systems.ask("Three-phase power: VL = 400 V, IL = 50 A, pf = 0.8, connection = star")
print(a.outcome, a.value, a.unit)
```

Output:

```text
SOLVED 27712.8129211 W
```

Every calculation, with its formula, a worked example and its figures: **[Energy Systems on ai999x.com](https://www.ai999x.com/models/energy-systems-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/energy-systems-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/energy-systems-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `energy-systems` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/energy-systems`) and its data (`rust/models/energy_systems/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p energy-systems-ai999x
./target/release/energy-systems --help
cargo test -p energy-systems-ai999x
```
