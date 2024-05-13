
import XCTest
@testable import ErgoLib
@testable import ErgoLibC

final class SecretKeyTests: XCTestCase {
    func testSecretKey() throws {
        let key = SecretKey()
        let bytes = key.toBytes()
        let newKey = try SecretKey(fromBytes: bytes)
        XCTAssertEqual(bytes, newKey.toBytes())
    }
    // Test that SecretKey constructor fails for arrays that are too short
    func testInvalidSecretKey() {
        let bytes = [UInt8](repeating: UInt8(0), count: 30)
        XCTAssertThrowsError(try SecretKey(fromBytes: bytes))
    }
    func testInvalidExtSecretKey() {
            let bytes = [UInt8](repeating: UInt8(0), count: 30)
            XCTAssertThrowsError(try ExtSecretKey(seedBytes: bytes))
    }
}
