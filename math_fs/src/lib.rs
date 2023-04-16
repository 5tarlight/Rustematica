use std::{fs, path::Path};

/// Read content of {path}.math file.
/// It will automatically append `.math` if not exists and
/// it won't if exists.
///
/// # Panic
/// When unable to read file
pub fn read_math(path: String) -> String {
    if path.ends_with(".math") {
        fs::read_to_string(path)
            .expect("Failed to read file.")
            .trim()
            .to_string()
    } else {
        fs::read_to_string(path + ".math")
            .expect("Failed to read file.")
            .trim()
            .to_string()
    }
}

/// Check does math file eixsts.
/// It will automatically append `.math` if given path argument
/// doesn't end with `.math`.
///
/// # Panics
/// When unable to read file (when access denied by permission lackage or etc.)
///
/// # See Also
/// This functin internally use `try_exists` <br>
/// [`try_exists()`](std::path::Path::try_exists)
pub fn is_math_exist(path: String) -> bool {
    let mut path = path.clone();

    if !path.ends_with(".math") {
        path += ".math";
    }

    if let Ok(exist) = Path::new(&path).try_exists() {
        exist
    } else {
        panic!("Unable to read file")
    }
}
