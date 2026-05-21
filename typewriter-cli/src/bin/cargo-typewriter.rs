fn main() {
    let mut rest: Vec<_> = std::env::args_os().skip(1).collect();
    if rest.first().and_then(|arg| arg.to_str()) == Some("typewriter") {
        rest.remove(0);
    }

    let mut args = vec![std::ffi::OsString::from("typebridge")];
    args.extend(rest);

    std::process::exit(match typebridge_cli::run_with_args(args) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("cargo-typewriter: {}", err);
            1
        }
    });
}
