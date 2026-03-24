fn main() {
    // Tell cargo to recompile when typewriter.toml changes
    println!("cargo::rerun-if-changed=typewriter.toml");
}
