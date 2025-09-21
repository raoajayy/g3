//! Build script for G3 ICAP Server

fn main() {
    g3_build_env::check_basic();
    g3_build_env::check_openssl();
    g3_build_env::check_rustls_provider();
}
