# Testing

Tests are written using [AVA](https://github.com/avajs/ava):

```bash
yarn test
# or
npm run test
```

Native functions are tested in Node.js, and some experiments are also validated in browser environments via **WASM** builds.

Example test output:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```