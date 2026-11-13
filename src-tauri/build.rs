fn main() {
    // Le catalogue est embarqué dans le binaire : le recompiler quand il change.
    println!("cargo:rerun-if-changed=../catalog");
    tauri_build::build()
}
