import Foundation

// Extensions to make the OkId class more Swift-friendly
public extension OkId {
    /// The hash type property
    var hashType: String {
        getHashType()
    }
    
    /// The digest property
    var digest: String {
        getDigest()
    }
    
    /// String representation
    var stringValue: String {
        "\(hashType)\(getSeparator())\(digest)"
    }
}