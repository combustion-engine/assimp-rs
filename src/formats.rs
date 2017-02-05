//! Contains all formats and their file extensions supported by Assimp
//!
//! See http://www.assimp.org/main_features_formats.html

use std::ffi::CString;

/// Calls `aiIsExtensionSupported` to check if Assimp can handle the extension
pub fn is_extension_supported(ext: &str) -> bool {
    let c_str = CString::new(ext).unwrap();

    unsafe { ::ffi::aiIsExtensionSupported(c_str.as_ptr()) == ::ffi::TRUE }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_supported() {
        assert!(is_extension_supported("obj"));
        assert!(is_extension_supported("ply"));
        assert!(is_extension_supported("dae"));
    }
}