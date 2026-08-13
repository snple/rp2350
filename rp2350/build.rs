fn main() {
    #[cfg(all(feature = "rp2350a", feature = "rp2350b"))]
    compile_error!("Only one of the features 'rp2350a' or 'rp2350b' can be enabled at a time.");
}
