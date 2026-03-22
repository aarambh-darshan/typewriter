fn main() {
    let mut args = vec![std::ffi::OsString::from("typewriter")];
    args.extend(std::env::args_os().skip(1));

    std::process::exit(match typebridge_cli::run_with_args(args) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("cargo-typewriter: {}", err);
            1
        }
    });
}
