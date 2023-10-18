fn main() {
    if let Err(e) = common::get_args().and_then(common::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
