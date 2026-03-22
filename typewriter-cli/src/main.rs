fn main() {
    std::process::exit(match typewriter_cli::run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("typewriter: {}", err);
            1
        }
    });
}
