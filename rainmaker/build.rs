fn main() {
    let mut config = prost_build::Config::new();
    config.default_package_filename("user_mapping");
    config
        .compile_protos(&[
                "proto/esp_rmaker_user_mapping.proto",
                "proto/local_ctrl.proto",
            ],
             &["proto/"])
        .unwrap();
}
