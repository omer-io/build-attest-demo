fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
    .format_timestamp_millis() // optional, nice timestamps
    .init();
    log::info!("Hello from attest-build-provenance demo with submodule!");
    println!("Program executed successfully");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
