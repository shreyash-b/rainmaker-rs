fn main() {
    let mut config = prost_build::Config::new();
    config.default_package_filename("session");
    config
        .compile_protos(
            &[
                "proto/session.proto",
                "proto/wifi_config.proto",
                "proto/wifi_scan.proto",
                "proto/local_ctrl.proto"
            ],
            &["proto/"],
        )
        .unwrap();
}
