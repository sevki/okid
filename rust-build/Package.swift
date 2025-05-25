// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "rust-build",
    products: [
        // Products can be used to vend plugins, making them visible to other packages.
        .plugin(
            name: "rust-build",
            targets: ["rust-build"]),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .plugin(
            name: "rust-build",
            capability: .command(
                intent: .custom(verb: "generate-swift-bindings", description: "Generate Swift bindings from Rust library"),
                permissions: [.writeToPackageDirectory(reason: "Generate Swift bindings")]
            )
        ),
    ]
)
