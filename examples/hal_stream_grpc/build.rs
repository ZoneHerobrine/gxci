use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let proto_files = [
        "proto/camera.proto",
    ];

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("camera_descriptor.bin"))
        .compile(&proto_files, &["proto"])?;

    for proto_file in &proto_files {
        tonic_build::compile_protos(proto_file)?;
    }

    Ok(())
}
