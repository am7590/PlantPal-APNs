use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/store.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")  // for older systems
        .build_client(true)
	.build_server(true)
	.file_descriptor_set_path(out_dir.join("plant_descriptor.bin"))
<<<<<<< HEAD
	.out_dir(".")
=======
	.out_dir("./src")
>>>>>>> b7e971a (womp)
	.compile(&[proto_file], &["proto"])?;

    Ok(())
}
