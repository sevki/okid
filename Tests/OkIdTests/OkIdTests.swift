import Foundation
import Testing

@testable import OkId

@Suite("OkId Tests")
struct OkIdTests {
    init() {
        uniffiEnsureOkidInitialized()
    }
    
    @Test("Simple hello world test")
    func testHelloWorld() throws {
        let testData = "hello world".data(using: .utf8)!
        let okid = createSha256(data: testData)
        
        // Just verify we got something back
        #expect(okid.hashType.count > 0)
        #expect(okid.digest.count > 0)
    }
}