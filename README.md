# numlang

Numlang is a collection of libraries for converting numbers into their linguistic equivalents (words, ordinals, etc.).

This monorepo will support multiple implementations:

- [`numlang-js`](./packages/numlang-js): TypeScript/JavaScript library
- [`numlang-rs`](./packages/numlang-rs): Rust library (coming soon)

---

## Features (JS/TS)

- Convert numbers to words (cardinal form), including floats (e.g., `12.34` → `"twelve point three four"`)
- Convert numbers to ordinal form (1st, 2nd, 3rd) — **floats not supported**
- Convert numbers to ordinal words (first, second, third) — **floats not supported**
- Supports negative numbers
- Configurable formatting options
- Written in TypeScript with full type definitions
- Zero dependencies
- Comprehensive test coverage

## Installation (JS/TS)

See [`packages/numlang-js/README.md`](./packages/numlang-js/README.md) for installation and usage instructions.

## Usage (JS/TS)

See [`packages/numlang-js/README.md`](./packages/numlang-js/README.md) for API details and examples.

---

## Roadmap

- [x] TypeScript/JavaScript implementation
- [ ] Rust implementation (planned)

---

## License

MIT
