# Release

## Releasing the Package

Ensure you have set your **NPM_TOKEN** in the `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want to release the package:

```bash
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

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