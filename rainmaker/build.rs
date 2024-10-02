use prost_build::compile_protos;

fn main() {
    compile_protos(
        &[
            "proto/esp_rmaker_claim.proto",
            "proto/esp_rmaker_user_mapping.proto",
        ],
        &["proto/"],
    )
    .unwrap();
}
