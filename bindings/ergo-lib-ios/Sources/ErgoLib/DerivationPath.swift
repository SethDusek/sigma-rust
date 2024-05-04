import Foundation
import ErgoLibC

class DerivationPath {
    internal var pointer: DerivationPathPtr

    /// Create DerivationPath from string
    /// String should be in the form of: m/44/429/acc'/0/addr
    init(derivationPathStr: String) throws {
        var ptr: DerivationPathPtr?
        let error = derivationPathStr.withCString { cs in
            ergo_lib_derivation_path_from_str(cs, &ptr)
        }
        try checkError(error)
        self.pointer = ptr!
    }

    deinit {
        ergo_lib_derivation_path_delete(self.pointer)
    }
}
