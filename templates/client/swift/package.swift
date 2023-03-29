// swift-tools-version:5.5

import PackageDescription

let package = Package(
    name: "Teo",
    platforms: [
        .macOS(.v13),
        .iOS(.v16)
    ],
    products: [
        .library(
            name: "Teo",
            targets: ["Teo"]),
    ],
    targets: [
        .target(
            name: "API",
            dependencies: [])
    ]
)
