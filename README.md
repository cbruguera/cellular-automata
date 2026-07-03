# Cellular Automata Explorer

A simulation and visualization tool for cellular automata. Built with a Rust/WebAssembly engine and a Svelte frontend.

## Features

- Game of Life with configurable B/S rules, neighborhood type (Moore / Von Neumann), and boundary conditions (toroidal / dead)
- Cell age coloring (optional, via View tab)
- Built-in pattern library — methuselahs, guns, spaceships, oscillators
- Custom RLE pattern loading
- Zoom / pan canvas (scroll + drag, `F` to fit)
- Configurable grid size
- Keyboard shortcuts: `Space` play/pause, `→` step, `R` randomize, `F` fit

## Stack

- **Engine**: Rust → WebAssembly via `wasm-pack`
- **Frontend**: Svelte + Vite

## Getting started

```bash
# requires Rust, wasm-pack, Node.js
make dev
```

Opens at `http://localhost:5173`.

## Build

```bash
make build   # compile Rust → WASM only
make dev     # build + start dev server
```

## Project structure

```
engine/src/
├── lib.rs               # WASM boundary
├── grid/                # N-dimensional grid (ndarray)
├── neighborhood/        # Moore / Von Neumann offsets
└── automata/
    ├── mod.rs           # Automaton trait
    └── life.rs          # Game of Life

web/src/
├── App.svelte           # main shell
├── components/
│   ├── CaDropdown.svelte
│   └── SettingsPanel.svelte
└── lib/patterns.js      # built-in pattern library
```

## Planned

- Wolfram 1D elementary CA
- 3D Game of Life (Three.js)
- N-dimensional slice navigator
- Continuous automata (SmoothLife)
