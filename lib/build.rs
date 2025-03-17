fn main() {
    // Tell Cargo to recompile if any SQL files in lib/db/migrations change
    println!("cargo:rerun-if-changed=lib/db/migrations/");
}