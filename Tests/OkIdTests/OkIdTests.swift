import Foundation
import XCTest

@testable import OkId

final class OkIdTests: XCTestCase {
    func test_encode() async throws {
        // test codable
        //{"hash_type":"sha256","digest":"b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"}
        let okid = OkId(hash_type: .sha256, digest: "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
        let json = try JSONEncoder().encode(okid)
    }
    func test_decode() async throws {
        // test codable
        let json = """
        {"hash_type":"sha256","digest":"b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"}
        """
        let okid = try JSONDecoder().decode(OkId.self, from: Data(json.utf8))
        XCTAssert(okid.hash_type == .sha256)
        XCTAssert(okid.digest == "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
    }
    func test_encode_decode() async throws {
        // test codable
        let okid = OkId(hash_type: .sha256, digest:"b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
        let json = try JSONEncoder().encode(okid)
        let decoded = try JSONDecoder().decode(OkId.self, from: json)
        XCTAssert(decoded.hash_type == .sha256)
        XCTAssert(decoded.digest == "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
    }
}
