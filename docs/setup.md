# Development environment

## Usage

### Getting Started

1. Clone the repository.
2. Install dependencies:

   ```bash
   yarn install
   ```
3. Build the native module:

   ```bash
   yarn build
   ```

---

## Installation

To install the package (once published):

```bash
yarn add @wetteyve/rusty
```

---

## Capabilities

### Build

After running:

```bash
yarn build
# or
npm run build
```

You will find platform-specific native binaries such as:

```
rusty.[darwin|win32|linux].node
```

These binaries are built from the Rust source in [`src/lib.rs`](../src/lib.rs) using **napi-rs**.

The package is compiled for all supported napi-rs targets.

## Development Requirements

* Latest stable **Rust**
* **Node.js 10+** (Node-API compatible)
* **Yarn 1.x**