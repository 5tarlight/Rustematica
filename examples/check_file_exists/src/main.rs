use math_fs::is_math_exist;

fn check_exist(path: &str) {
    let exist = is_math_exist(path.to_string());
    println!("Does file {} exist? {}", path, exist);
}

fn main() {
    check_exist("./unknown_file.math");
    check_exist("./unknown_file");
    check_exist("./known.math");
    check_exist("known.math");
}
