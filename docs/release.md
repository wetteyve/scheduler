# Release

## Releasing the Package

1. Add your **NPM_TOKEN** in GitHub:

    * `Settings → Secrets → Actions → New repository secret`
2. Bump the version:

   ```bash
   npm version patch
   git push
   ```

GitHub Actions will handle building and publishing all artifacts.

> ⚠️ **Do not run `npm publish` manually.**

## Release Strategy

Releasing native Node.js packages used to be painful due to toolchain requirements (`gcc`, `llvm`, `node-gyp`, etc.).

This project uses a modern approach:

* **Prebuilt binaries** are generated via GitHub Actions
* **N-API** ensures ABI compatibility across Node.js versions
* Separate platform-specific npm packages are published
* These are listed as `optionalDependencies` of the main package

`npm` automatically installs the correct binary for the user’s platform without requiring postinstall downloads.

You can explore this setup in the [`npm`](./npm) directory.