use std::{env, path::Path};

fn main() {
    let non_secure_path = Path::new(&env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .with_file_name("libnon_secure.a");

    let copy_output = std::process::Command::new("objcopy")
        .arg(format!(
            "--prefix-symbols=ns_ {}",
            non_secure_path.display()
        ))
        .output()
        .unwrap();

    println!("{copy_output:?}");
}
