// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "OkId",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "OkId",
            targets: ["OkId"]),
    ],
    dependencies: [
        // Swift-DocC plugin for generating documentation
        .package(url: "https://github.com/apple/swift-docc-plugin", from: "1.3.0"),
        // Swift Testing framework
        .package(url: "https://github.com/swiftlang/swift-testing.git", from: "6.1.1"),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "OkId",
            dependencies: [],
            publicHeadersPath: "include",
            linkerSettings: [
                .linkedLibrary("okid"),
                .unsafeFlags(["-L", "target/release"]),
                .unsafeFlags(["-Xlinker", "-rpath", "-Xlinker", "target/release"])
            ]
        ),
        .testTarget(
            name: "OkIdTests",
            dependencies: [
                "OkId",
                .product(name: "Testing", package: "swift-testing")
            ],
            linkerSettings: [
                .linkedLibrary("okid"),
                .unsafeFlags(["-L", "../target/release"]),
                .unsafeFlags(["-Xlinker", "-rpath", "-Xlinker", "../target/release"])
            ]
        ),
    ]
)
