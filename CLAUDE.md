# Claude Code Guide for @wetteyve/rusty

## Project Overview

This is a **napi-rs** project that provides native Node.js addons written in Rust. The project demonstrates cross-platform native module development with support for both Node.js runtime and browser environments (via WASM).

**Primary Goals:**
- Experiment with Rust and napi-rs
- Build reusable utility functions in Rust with Node.js bindings
- Support all major platforms (Windows, macOS, Linux, FreeBSD, Android, WASM)
- Maintain high performance through native code
- Provide seamless integration for both server and client environments

## Technology Stack

### Core Technologies
- **Rust** (Edition 2021) - Core implementation language
- **napi-rs** (v3.0.0) - Node.js Native API bindings framework
- **Node.js** - Runtime environment (v12.22.0+, v14.17.0+, v15.12.0+, v16.0.0+)
- **TypeScript** - Type definitions and tooling

### Build & Development Tools
- **@napi-rs/cli** - Build and package management for napi-rs projects
- **Cargo** - Rust package manager and build tool
- **AVA** - Testing framework
- **oxlint** - Fast JavaScript/TypeScript linter
- **Prettier** - Code formatter
- **Taplo** - TOML formatter
- **Husky** - Git hooks
- **lint-staged** - Pre-commit linting

### WASM Support
- **@emnapi/core** - Emscripten Node-API implementation for WASM
- **@tybys/wasm-util** - WASM utilities
- **wasm32-wasip1-threads** target support

## Project Structure

```
rusty/
├── src/
│   └── lib.rs              # Main Rust source code (native functions)
├── __test__/
│   └── index.spec.ts       # AVA test suite
├── benchmark/
│   └── bench.ts            # Performance benchmarks
├── docs/
│   ├── overview.md         # Project overview
│   ├── setup.md            # Development setup
│   ├── testing.md          # Testing documentation
│   ├── ci-cd.md            # CI/CD documentation
│   └── release.md          # Release process
├── .github/
│   └── workflows/
│       └── CI.yml          # GitHub Actions CI/CD pipeline
├── Cargo.toml              # Rust package configuration
├── package.json            # Node.js package configuration
├── build.rs                # Rust build script
├── index.js                # Auto-generated Node.js entry point
├── index.d.ts              # Auto-generated TypeScript definitions
├── browser.js              # Browser entry point
├── wasi-worker.mjs         # WASM worker for Node.js
└── wasi-worker-browser.mjs # WASM worker for browser
```

## Architecture

### Native Module Flow
1. **Rust Implementation** (`src/lib.rs`):
   - Functions are marked with `#[napi]` macro
   - napi-derive handles code generation
   - Supports various data types (primitives, arrays, objects)

2. **Build Process**:
   - `build.rs` uses `napi-build` for setup
   - Cargo compiles Rust to native binaries
   - Platform-specific `.node` files are generated
   - TypeScript definitions auto-generated

3. **Node.js Integration** (`index.js`):
   - Auto-generated loader detects platform
   - Loads appropriate native binary
   - Handles musl vs glibc on Linux
   - Graceful fallback handling

4. **WASM Support**:
   - Alternative compilation target for browsers
   - Uses emnapi for Node-API compatibility
   - Async loading required in browser contexts

### Supported Platforms

The project targets 14 platforms:
- **Windows**: x86_64, i686, aarch64
- **macOS**: x86_64, aarch64 (Apple Silicon)
- **Linux**: x86_64 (glibc/musl), aarch64 (glibc/musl), armv7 (gnueabihf)
- **FreeBSD**: x86_64
- **Android**: aarch64, armv7
- **WASM**: wasm32-wasip1-threads

## Development Workflow

### Initial Setup
```bash
# Install dependencies
yarn install

# Build native module for current platform
yarn build

# Run tests
yarn test
```

### Common Commands
```bash
# Development build (with debug symbols)
yarn build:debug

# Format all code
yarn format              # Format all (prettier, Rust, TOML)
yarn format:prettier     # Format JS/TS/JSON/YAML
yarn format:rs          # Format Rust code
yarn format:toml        # Format TOML files

# Linting
yarn lint               # Run oxlint

# Testing
yarn test               # Run AVA tests

# Benchmarking
yarn bench              # Run performance benchmarks

# Build artifacts for all platforms
yarn artifacts

# Prepare for npm publish
yarn prepublishOnly
```

### Git Workflow
- **Husky** runs pre-commit hooks
- **lint-staged** automatically formats and lints staged files:
  - `*.{js,ts,tsx}` - oxlint --fix
  - `*.{js,ts,tsx,yml,yaml,md,json}` - prettier --write
  - `*.toml` - taplo format

## Adding New Native Functions

### Step 1: Implement in Rust (`src/lib.rs`)
```rust
#[napi]
pub fn your_function_name(param: Type) -> ReturnType {
    // Implementation
}
```

### Step 2: Build
```bash
yarn build
```
This auto-generates TypeScript definitions and Node.js bindings.

### Step 3: Test
Create test in `__test__/index.spec.ts`:
```typescript
import test from 'ava'
import { yourFunctionName } from '../index'

test('description', (t) => {
  t.is(yourFunctionName(input), expected)
})
```

### Step 4: Run Tests
```bash
yarn test
```

## Testing

### Test Framework
- Uses **AVA** with TypeScript support via `@oxc-node/core/register`
- 2-minute timeout per test
- Worker threads disabled for native module compatibility
- Custom tsconfig at `__test__/tsconfig.json`

### Running Tests
```bash
yarn test           # Run all tests
ava --verbose       # Run with verbose output
ava --watch         # Watch mode
```

## CI/CD Pipeline

### GitHub Actions (`.github/workflows/CI.yml`)
The pipeline runs on:
- **Push to main branch**
- **Pull requests**
- **Manual workflow dispatch**

### Build Matrix
- **Node.js versions**: 20, 22
- **Operating Systems**: macOS, Linux, Windows
- **All 14 target platforms** are built in dedicated jobs

### Steps
1. Checkout code
2. Setup Rust toolchain
3. Setup Node.js
4. Install dependencies
5. Build for specific platform
6. Run tests
7. Upload artifacts
8. Publish to npm (on release)

## Release Process

### Creating a Release
```bash
# Bump version (patch/minor/major)
npm version patch   # or minor, or major

# Push to GitHub (triggers CI/CD)
git push
git push --tags
```

### What Happens
1. GitHub Actions builds for all platforms
2. Native binaries uploaded as artifacts
3. Separate platform-specific npm packages published
4. Main package published with optionalDependencies

**NEVER run `npm publish` manually** - let GitHub Actions handle it.

## Key Files Reference

### Configuration Files
- **Cargo.toml** (`/Cargo.toml:1`): Rust package configuration
- **package.json** (`/package.json:1`): Node.js package configuration, scripts, and napi config
- **tsconfig.json** (`/tsconfig.json:1`): TypeScript configuration
- **rustfmt.toml** (`/rustfmt.toml:1`): Rust formatting rules
- **.taplo.toml** (`/.taplo.toml:1`): TOML formatting configuration
- **.prettierignore** (`/.prettierignore:1`): Prettier ignore patterns

### Source Code
- **src/lib.rs** (`/src/lib.rs:1`): Main Rust implementation
- **build.rs** (`/build.rs:1`): Rust build script (calls napi-build)

### Generated Files (DO NOT EDIT)
- **index.js** (`/index.js:1`): Auto-generated Node.js loader
- **index.d.ts** (`/index.d.ts:1`): Auto-generated TypeScript definitions
- **rusty.darwin-arm64.node**: Platform-specific native binary
- **rusty.wasi.cjs**: WASM build for Node.js
- **rusty.wasi-browser.js**: WASM build for browsers

## Browser/WASM Usage

### Vite Configuration
Add to `vite.config.js`:
```javascript
export default defineConfig({
  optimizeDeps: {
    exclude: ['@wetteyve/rusty']
  }
})
```

### Package Manager Setup
Follow napi-rs WASM guide for your package manager:
https://napi.rs/docs/concepts/webassembly

### Client-Side Loading
```typescript
// Must load asynchronously
const wasm = await import('@wetteyve/rusty')
wasm.getArrayLength([1, 2, 3]) // Use functions
```

## Common Development Tasks

### Adding Dependencies
```bash
# Rust dependencies (edit Cargo.toml)
cargo add <crate-name>

# Node.js dependencies
yarn add <package-name>
yarn add -D <dev-package>
```

### Debugging
```bash
# Build with debug symbols
yarn build:debug

# Check Rust compilation
cargo check

# Run Rust tests (if any)
cargo test
```

### Formatting Before Commit
```bash
# Format everything
yarn format

# Or let git hooks handle it automatically
git add .
git commit -m "message"  # Hooks run automatically
```

## Performance Considerations

1. **Native Code Benefits**: Rust functions execute at native speed, ideal for:
   - CPU-intensive computations
   - Data processing
   - Array/buffer operations
   - Cryptographic operations

2. **N-API Overhead**: Small overhead for data marshalling between JS and Rust
   - Minimize calls with large data transfers
   - Process data in batches when possible

3. **Benchmarking**: Use `yarn bench` to measure performance
   - Compare against pure JS implementations
   - Test with realistic data sizes

## Troubleshooting

### Build Issues
```bash
# Clean build artifacts
cargo clean
rm -rf target/
rm *.node

# Rebuild
yarn build
```

### Platform-Specific Builds
```bash
# Build for specific target
yarn build --target x86_64-apple-darwin
```

### WASM Issues
- Ensure package manager supports WASM (see napi-rs docs)
- Verify async loading in browser contexts
- Check browser console for WASM loading errors

## Resources

- **napi-rs Documentation**: https://napi.rs
- **Rust Book**: https://doc.rust-lang.org/book/
- **Node-API Documentation**: https://nodejs.org/api/n-api.html
- **Project Docs**: `docs/` directory
- **AVA Documentation**: https://github.com/avajs/ava

## Development Tips for Claude Code

When working on this project:

1. **Always rebuild after Rust changes**: Run `yarn build` after modifying `src/lib.rs`
2. **Check generated files**: Verify `index.d.ts` reflects your function signatures
3. **Test both Node and WASM**: Ensure functions work in both environments
4. **Follow Rust conventions**: Use `cargo fmt` and address `clippy` warnings
5. **Update docs**: Keep `docs/` in sync with code changes
6. **Platform compatibility**: Consider cross-platform implications for new features
7. **Type safety**: Leverage napi-rs type mapping for better JS/TS integration
8. **Performance**: Benchmark critical functions with `yarn bench`

## Current Exported Functions

As of the latest version, the module exports:

- **plus100(input: number): number** - Adds 100 to the input (`src/lib.rs:7`)
- **getArrayLength(arr: any[]): number** - Returns array length (`src/lib.rs:12`)

These are examples for learning. Add more utility functions as needed.
