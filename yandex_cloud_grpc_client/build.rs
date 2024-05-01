// Copy code from https://docs.rs/yandex-cloud/2023.9.4/yandex_cloud/index.html
use std::fs::create_dir_all;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn proto_files(proto_dir: &str) -> Vec<PathBuf> {
    let mut out = vec![];

    fn is_proto(entry: &DirEntry) -> bool {
        entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map(|e| e.to_string_lossy() == "proto")
                .unwrap_or(false)
    }

    for entry in WalkDir::new(format!("{}/yandex", proto_dir)).into_iter() {
        let entry = entry.expect("failed to list proto files");

        if is_proto(&entry) {
            out.push(entry.into_path())
        }
    }

    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "protos";
    let out_dir = "src/client";

    create_dir_all(out_dir)?;

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        .include_file("includes.rs")
        .compile(
            &proto_files(proto_dir),
            &[
                format!("{}", proto_dir),
                format!("{}/third_party/googleapis", proto_dir),
            ],
        )
        .expect("failed to generate gRPC clients for Yandex Cloud");

    Ok(())
}
