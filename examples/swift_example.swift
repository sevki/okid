// Example of using OkId Swift bindings
import Foundation
import okid

// Create an OkId from a string
let data = "Hello, World!".data(using: .utf8)!

// Create different types of OkIds
let sha256Id = createSha256(data: data)
print("SHA256 OkId: \(sha256Id.value)")

let blake3Id = createBlake3(data: data)
print("Blake3 OkId: \(blake3Id.value)")

let fingerprintId = createFingerprint(data: data)
print("Fingerprint OkId: \(fingerprintId.value)")

// Create UUID and ULID
let uuidId = createUuid()
print("UUID OkId: \(uuidId.value)")

let ulidId = createUlid()
print("ULID OkId: \(ulidId.value)")

// Parse an OkId from string
do {
    let parsedId = try okidFromString(s: sha256Id.value)
    print("Parsed OkId: \(parsedId.value)")
    
    // Convert to path-safe format
    let pathSafe = try okidToPathSafe(okid: parsedId)
    print("Path-safe: \(pathSafe)")
} catch {
    print("Error: \(error)")
}