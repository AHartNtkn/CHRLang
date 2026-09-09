fn main() {
    if let Err(error) = chr_compiled::experiment::main() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
