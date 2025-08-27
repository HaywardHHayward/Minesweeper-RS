#[cfg(not(feature = "non-free"))]
const NON_FREE_DIRS: [&str; 1] = ["classic"];

fn exclude_files(entry: &walkdir::DirEntry) -> bool {
    #[cfg(not(feature = "non-free"))]
    if entry.file_type().is_dir() {
        // Exclude directories that are not free
        return !NON_FREE_DIRS.contains(&entry.file_name().to_string_lossy().as_ref());
    }
    entry.file_name().to_string_lossy() != "LICENSE"
        || entry.file_name().to_string_lossy() != "Icon.qoi"
}

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
        .filter_entry(exclude_files)
        .filter_map(|e| e.ok())
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
