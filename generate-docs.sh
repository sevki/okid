#!/bin/bash

# Script to generate documentation for TypeScript and Swift
# Swift docs are generated in Docker since we're on NixOS

set -e

echo "📚 Generating documentation for OkId..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create docs directory
mkdir -p docs

# TypeScript Documentation
echo -e "${BLUE}📘 Generating TypeScript documentation...${NC}"

# First, ensure we have the latest wasm-pack build
echo "Building WASM package..."
wasm-pack build

# Install TypeDoc if not already installed
if ! command -v typedoc &> /dev/null; then
    echo "Installing TypeDoc..."
    npm install -g typedoc
fi

# Generate TypeScript docs
typedoc --config typedoc.json

echo -e "${GREEN}✅ TypeScript documentation generated in ./docs/typescript${NC}"

# Swift Documentation (using Docker)
echo -e "${BLUE}📙 Generating Swift documentation using Docker...${NC}"

# Build the Docker image
echo "Building Docker image for Swift documentation..."
docker build -f Dockerfile.swift-docs -t okid-swift-docs .

# Run the container to generate docs
echo "Generating Swift documentation..."
CONTAINER_ID=$(docker create okid-swift-docs)

# Copy the generated docs from the container
docker cp $CONTAINER_ID:/output/docs/swift ./docs/swift

# Clean up
docker rm $CONTAINER_ID

echo -e "${GREEN}✅ Swift documentation generated in ./docs/swift${NC}"

# Create an index.html to link both documentations
cat > docs/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>OkId Documentation</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            line-height: 1.6;
            background-color: #f5f5f5;
        }
        .container {
            background: white;
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }
        h1 { 
            color: #333;
            margin-bottom: 0.5rem;
        }
        .subtitle {
            color: #666;
            margin-top: 0;
            margin-bottom: 2rem;
        }
        .doc-links {
            display: flex;
            gap: 2rem;
            margin: 2rem 0;
            flex-wrap: wrap;
        }
        .doc-card {
            flex: 1;
            min-width: 250px;
            border: 1px solid #ddd;
            border-radius: 8px;
            padding: 1.5rem;
            text-decoration: none;
            color: inherit;
            transition: transform 0.2s, box-shadow 0.2s;
            background: #fafafa;
        }
        .doc-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
            background: white;
        }
        .doc-card h2 {
            margin-top: 0;
            color: #0066cc;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }
        .doc-card .icon {
            font-size: 1.5rem;
        }
        .doc-card p {
            color: #666;
            margin-bottom: 0;
        }
        .links {
            margin-top: 2rem;
            padding-top: 2rem;
            border-top: 1px solid #eee;
        }
        .links ul {
            list-style: none;
            padding: 0;
            display: flex;
            gap: 2rem;
            flex-wrap: wrap;
        }
        .links a {
            color: #0066cc;
            text-decoration: none;
        }
        .links a:hover {
            text-decoration: underline;
        }
        code {
            background: #f4f4f4;
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
            font-family: 'SF Mono', Monaco, 'Cascadia Code', monospace;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>OkId Documentation</h1>
        <p class="subtitle">A library for generating double-clickable identifiers</p>
        
        <div class="doc-links">
            <a href="./typescript/index.html" class="doc-card">
                <h2><span class="icon">📘</span> TypeScript / JavaScript</h2>
                <p>API documentation for the TypeScript/JavaScript bindings using WebAssembly.</p>
                <p style="margin-top: 1rem;"><code>npm install okid</code></p>
            </a>
            
            <a href="./swift/documentation/okid/" class="doc-card">
                <h2><span class="icon">📙</span> Swift</h2>
                <p>API documentation for the Swift bindings using FFI.</p>
                <p style="margin-top: 1rem;"><code>.package(url: "https://github.com/sevki/okid", from: "0.14.0")</code></p>
            </a>
        </div>
        
        <div class="links">
            <h3>Resources</h3>
            <ul>
                <li><a href="https://github.com/sevki/okid">GitHub Repository</a></li>
                <li><a href="https://www.npmjs.com/package/okid">npm Package</a></li>
                <li><a href="https://crates.io/crates/okid">Rust Crate</a></li>
                <li><a href="https://docs.rs/okid">Rust Docs</a></li>
            </ul>
        </div>
    </div>
</body>
</html>
EOF

echo -e "${GREEN}✅ Documentation index created at ./docs/index.html${NC}"
echo ""
echo -e "${YELLOW}📋 To publish to GitHub Pages:${NC}"
echo "1. Ensure GitHub Pages is enabled in your repository settings"
echo "2. Set the source to deploy from the /docs folder on your main branch"
echo "3. Commit and push the docs directory:"
echo "   git add docs"
echo "   git commit -m 'Update documentation'"
echo "   git push"
echo ""
echo "Your documentation will be available at:"
echo "  https://[username].github.io/okid/"
echo ""
echo "Direct links:"
echo "  TypeScript: https://[username].github.io/okid/typescript/"
echo "  Swift: https://[username].github.io/okid/swift/documentation/okid/"