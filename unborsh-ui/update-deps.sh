#!/bin/bash
set -e

# Remove old package dependencies
rm -rf node_modules
rm -f package-lock.json

# Install dependencies with specific versions
npm install

# Install tailwind dependencies explicitly
npm install -D tailwindcss@3.3.3 postcss@8.4.27 autoprefixer@10.4.14

# Generate the tailwind config if needed
npx tailwindcss init -p

echo "Dependencies updated successfully!"