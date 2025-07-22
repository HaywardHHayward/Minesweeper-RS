fn compress_assets() {
    use std::io::{Read, Write};

    use walkdir::WalkDir;
    use zip::*;
    let out_dir = std::env::var("OUT_DIR").expect("No OUT_DIR env var");
    let zip_path = std::path::Path::new(&out_dir).join("assets.zip");
    let zip_file = std::fs::File::create(&zip_path).expect("Failed to create zip file");
    let mut zip = ZipWriter::new(zip_file);
    let options = write::SimpleFileOptions::default();
    let assets_dir = WalkDir::new(std::path::Path::new("assets"));
    let mut buffer = Vec::new();
    for entry in assets_dir
        .contents_first(false)
        .into_iter()
        .filter_map(|e| {
            e.ok().and_then(|e| {
                if e.file_name().to_string_lossy() != "LICENSE" {
                    Some(e)
                } else {
                    None
                }
            })
        })
    {
        if entry.file_type().is_dir() {
            zip.add_directory_from_path(entry.path(), options)
                .unwrap_or_else(|e| panic!("Failed to add directory to zip. Error: {e:?}"));
        } else if entry.file_type().is_file() {
            zip.start_file_from_path(entry.path(), options)
                .unwrap_or_else(|e| {
                    panic!("Failed to add file to zip archive. Error: {e:?}");
                });
            let mut file = std::fs::File::open(entry.path()).unwrap_or_else(|e| {
                panic!("Failed to open file for zipping. Error: {e:?}");
            });
            file.read_to_end(&mut buffer).unwrap_or_else(|e| {
                panic!("Failed to read file for zipping. Error: {e:?}");
            });
            zip.write_all(&buffer).unwrap_or_else(|e| {
                panic!("Failed to write file to zip archive. Error: {e:?}");
            });
            buffer.clear();
        }
    }
    zip.finish().unwrap_or_else(|e| {
        panic!("Failed to finalize zip archive. Error: {e:?}");
    });
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=assets");
}

fn main() {
    compress_assets();
}
