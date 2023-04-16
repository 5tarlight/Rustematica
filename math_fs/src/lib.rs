use std::fs;

/// Read content of {path}.math file.
/// It will automatically append `.math` if not exists and
/// it won't if exists.
///
/// # Panic
/// When unable to read file
pub fn read_math(path: String) -> String {
    if path.ends_with(".math") {
        fs::read_to_string(path).expect("Failed to read file.")
    } else {
        fs::read_to_string(path + ".math").expect("Failed to read file.")
    }
}
