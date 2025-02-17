import Foundation
import Testing
@testable import OkId

@Test func test_encode() async throws {
    // test codable
    //{"hash_type":"sha256","digest":"b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"}
    let okid = OkId(hash_type: .sha256, digest: "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
    let json = try JSONEncoder().encode(okid)

}

@Test func test_decode() async throws {
    // test codable
    let json = """
    {"hash_type":"sha256","digest":"b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"}
    """
    let okid = try JSONDecoder().decode(OkId.self, from: Data(json.utf8))
    #expect(okid.hash_type == .sha256)
    #expect(okid.digest == "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
}

@Test func test_encode_decode() async throws {
    // test codable
    let okid = OkId(hash_type: .sha256, digest:"b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
    let json = try JSONEncoder().encode(okid)
    let decoded = try JSONDecoder().decode(OkId.self, from: json)
    #expect(decoded.hash_type == .sha256)
    #expect(decoded.digest == "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
}
