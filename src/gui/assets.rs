use std::io::Read;

use zip::{ZipArchive, result};

static ASSET_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/assets.zip"));

#[derive(Debug)]
enum CacheError {
    NotFound,
    IoError(std::io::Error),
    ZipError(result::ZipError),
}
fn get_data_from_cache(path: &std::path::Path) -> Result<Vec<u8>, CacheError> {
    let cached_asset_file = crate::Application::app_dirs()
        .cache_dir()
        .to_path_buf()
        .join("assets")
        .join(path);
    if !cached_asset_file.exists() {
        return Err(CacheError::NotFound);
    }
    let mut buffer = Vec::new();
    let mut asset_file = std::fs::File::open(&cached_asset_file).map_err(CacheError::IoError)?;
    asset_file
        .read_to_end(&mut buffer)
        .map_err(CacheError::IoError)?;
    Ok(buffer)
}

fn create_cache() -> Result<(), CacheError> {
    let cached_asset_dir = crate::Application::app_dirs()
        .cache_dir()
        .to_path_buf()
        .join("assets");
    if !cached_asset_dir.exists() {
        std::fs::create_dir_all(&cached_asset_dir).map_err(CacheError::IoError)?;
    }
    let archive_data = std::io::Cursor::new(ASSET_DATA);
    let mut archive = ZipArchive::new(archive_data).map_err(CacheError::ZipError)?;
    archive
        .extract_unwrapped_root_dir(cached_asset_dir, zip::read::root_dir_common_filter)
        .map_err(CacheError::ZipError)?;
    Ok(())
}

macro_rules! create_assets {
    ($([$name:ident, $extension:literal$(, $attr:meta)?]),*) => {
        $(
            $(#[$attr])?
            pub mod $name {
                use std::sync::LazyLock;

                use super::*;
                pub static OPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
                    let cache_result = get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/OpenedCell.", $extension)));
                    match cache_result {
                        Ok(data) => data,
                        Err(CacheError::NotFound) => {
                            if let Err(e) = create_cache() {
                                panic!("Failed to create cache: {e:?}");
                            }
                            get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/OpenedCell.", $extension)))
                                .expect("Failed to read OpenedCell from cache")
                        }
                        Err(e) => panic!("Failed to read OpenedCell from cache: {e:?}"),
                    }
                });

                pub static UNOPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
                    let cache_result = get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/UnopenedCell.", $extension)));
                    match cache_result {
                        Ok(data) => data,
                        Err(CacheError::NotFound) => {
                            if let Err(e) = create_cache() {
                                panic!("Failed to create cache: {e:?}");
                            }
                            get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/UnopenedCell.", $extension)))
                                .expect("Failed to read UnopenedCell from cache")
                        }
                        Err(e) => panic!("Failed to read UnopenedCell from cache: {e:?}"),
                    }
                });

                pub static MINE: LazyLock<Vec<u8>> = LazyLock::new(|| {
                    let cache_result = get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/Mine.", $extension)));
                    match cache_result {
                        Ok(data) => data,
                        Err(CacheError::NotFound) => {
                            if let Err(e) = create_cache() {
                                panic!("Failed to create cache: {e:?}");
                            }
                            get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/Mine.", $extension)))
                                .expect("Failed to read Mine from cache")
                        }
                        Err(e) => panic!("Failed to read Mine from cache: {e:?}"),
                    }
                });

                pub static FLAG: LazyLock<Vec<u8>> = LazyLock::new(|| {
                    let cache_result = get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/Flag.", $extension)));
                    match cache_result {
                        Ok(data) => data,
                        Err(CacheError::NotFound) => {
                            if let Err(e) = create_cache() {
                                panic!("Failed to create cache: {e:?}");
                            }
                            get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/Flag.", $extension)))
                                .expect("Failed to read Flag from cache")
                        }
                        Err(e) => panic!("Failed to read Flag from cache: {e:?}"),
                    }
                });

                pub static INCORRECT_FLAG: LazyLock<Vec<u8>> = LazyLock::new(|| {
                    let cache_result = get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/IncorrectFlag.", $extension)));
                    match cache_result {
                        Ok(data) => data,
                        Err(CacheError::NotFound) => {
                            if let Err(e) = create_cache() {
                                panic!("Failed to create cache: {e:?}");
                            }
                            get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/IncorrectFlag.", $extension)))
                                .expect("Failed to read IncorrectFlag from cache")
                        }
                        Err(e) => panic!("Failed to read IncorrectFlag from cache: {e:?}"),
                    }
                });

                pub static EXPLODED_MINE: LazyLock<Vec<u8>> = LazyLock::new(|| {
                    let cache_result = get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/ExplodedMine.", $extension)));
                    match cache_result {
                        Ok(data) => data,
                        Err(CacheError::NotFound) => {
                            if let Err(e) = create_cache() {
                                panic!("Failed to create cache: {e:?}");
                            }
                            get_data_from_cache(std::path::Path::new(concat!(stringify!($name), "/ExplodedMine.", $extension)))
                                .expect("Failed to read ExplodedMine from cache")
                        }
                        Err(e) => panic!("Failed to read ExplodedMine from cache: {e:?}"),
                    }
                });
            }
        )*
    };
}

create_assets!(
    [simple_light, "svg"],
    [simple_dark, "svg"],
    [classic, "svg", cfg(feature = "non-free")]
);
