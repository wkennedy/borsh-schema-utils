#!/bin/bash
set -e

# Build the WebAssembly module for different targets
function build_wasm() {
  local target=$1
  local outdir=$2

  echo "Building WebAssembly module for target: $target"
  wasm-pack build --target $target --out-dir $outdir

  # Copy JavaScript wrapper to package directory
  echo "Copying JavaScript wrapper..."
  mkdir -p $outdir/js
  cp js/index.js $outdir/js/
  cp js/index.d.ts $outdir/js/

  # Fix package.json to include the JS wrapper
  echo "Updating package.json..."
  node -e "
    const fs = require('fs');
    const pkg = JSON.parse(fs.readFileSync('$outdir/package.json', 'utf8'));

    // Add JS files to the package
    if (!pkg.files) pkg.files = [];
    if (!pkg.files.includes('js/index.js')) pkg.files.push('js/index.js');
    if (!pkg.files.includes('js/index.d.ts')) pkg.files.push('js/index.d.ts');

    // Update other metadata
    pkg.name = 'unborsh-wasm';
    pkg.description = 'WebAssembly bindings for unborsh - tools for analyzing Borsh serialized data';
    pkg.repository = {
      type: 'git',
      url: 'git+https://github.com/yourusername/unborsh.git'
    };
    pkg.keywords = ['borsh', 'serialization', 'analysis', 'wasm', 'webassembly'];
    pkg.license = 'MIT OR Apache-2.0';

    fs.writeFileSync('$outdir/package.json', JSON.stringify(pkg, null, 2));
  "

  # Copy README to package directory
  echo "Copying README..."
  cp README.md $outdir/
}

# Create directory for all builds
mkdir -p dist

# Build for bundlers (webpack, rollup, etc.)
build_wasm "bundler" "dist/pkg"

# Build for Node.js
build_wasm "nodejs" "dist/pkg-node"

# Build for direct browser use
build_wasm "web" "dist/pkg-web"

# Build a examples package
echo "Creating examples package..."
mkdir -p dist/examples
mkdir -p dist/examples/pkg
cp -r examples/* dist/examples/
cp -r dist/pkg-web/* dist/examples/pkg/

echo "Build complete!"
echo ""
echo "Files are available in the following directories:"
echo "  - dist/pkg      : For bundlers (webpack, rollup, etc.)"
echo "  - dist/pkg-node : For Node.js"
echo "  - dist/pkg-web  : For direct browser use"
echo "  - dist/examples     : Examples application"
echo ""
echo "To run the examples:"
echo "  cd dist/examples"
echo "  npx serve ."