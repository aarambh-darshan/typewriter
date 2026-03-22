fn main() {
    std::process::exit(match typebridge_cli::run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("typewriter: {}", err);
            1
        }
    });
}
