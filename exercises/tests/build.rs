fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:rustc-env=FEATURE_PASS=1");
    println!("cargo:rustc-cfg=pass");
}
