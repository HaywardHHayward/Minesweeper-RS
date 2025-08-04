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

pub(crate) mod simple_light {
    use std::sync::LazyLock;

    use super::*;
    pub(crate) static OPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("simple_light/OpenedCell.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_light/OpenedCell.svg"))
                    .expect("Failed to read OpenedCell from cache")
            }
            Err(e) => panic!("Failed to read OpenedCell from cache: {e:?}"),
        }
    });
    pub(crate) static UNOPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result =
            get_data_from_cache(std::path::Path::new("simple_light/UnopenedCell.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_light/UnopenedCell.svg"))
                    .expect("Failed to read UnopenedCell from cache")
            }
            Err(e) => panic!("Failed to read UnopenedCell from cache: {e:?}"),
        }
    });
    pub(crate) static MINE: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("simple_light/Mine.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_light/Mine.svg"))
                    .expect("Failed to read Mine from cache")
            }
            Err(e) => panic!("Failed to read Mine from cache: {e:?}"),
        }
    });
    pub(crate) static FLAG: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("simple_light/Flag.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_light/Flag.svg"))
                    .expect("Failed to read Flag from cache")
            }
            Err(e) => panic!("Failed to read Flag from cache: {e:?}"),
        }
    });
}

pub(crate) mod simple_dark {
    use std::sync::LazyLock;

    use super::*;
    pub(crate) static OPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("simple_dark/OpenedCell.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_dark/OpenedCell.svg"))
                    .expect("Failed to read OpenedCell from cache")
            }
            Err(e) => panic!("Failed to read OpenedCell from cache: {e:?}"),
        }
    });
    pub(crate) static UNOPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result =
            get_data_from_cache(std::path::Path::new("simple_dark/UnopenedCell.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_dark/UnopenedCell.svg"))
                    .expect("Failed to read UnopenedCell from cache")
            }
            Err(e) => panic!("Failed to read UnopenedCell from cache: {e:?}"),
        }
    });
    pub(crate) static MINE: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("simple_dark/Mine.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_dark/Mine.svg"))
                    .expect("Failed to read Mine from cache")
            }
            Err(e) => panic!("Failed to read Mine from cache: {e:?}"),
        }
    });
    pub(crate) static FLAG: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("simple_dark/Flag.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("simple_dark/Flag.svg"))
                    .expect("Failed to read Flag from cache")
            }
            Err(e) => panic!("Failed to read Flag from cache: {e:?}"),
        }
    });
}

#[cfg(feature = "non-free")]
pub(crate) mod classic {
    use std::sync::LazyLock;

    use super::*;
    pub(crate) static OPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("classic/OpenedCell.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("classic/OpenedCell.svg"))
                    .expect("Failed to read OpenedCell from cache")
            }
            Err(e) => panic!("Failed to read OpenedCell from cache: {e:?}"),
        }
    });
    pub(crate) static UNOPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("classic/UnopenedCell.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("classic/UnopenedCell.svg"))
                    .expect("Failed to read UnopenedCell from cache")
            }
            Err(e) => panic!("Failed to read UnopenedCell from cache: {e:?}"),
        }
    });
    pub(crate) static MINE: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("classic/Mine.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("classic/Mine.svg"))
                    .expect("Failed to read Mine from cache")
            }
            Err(e) => panic!("Failed to read Mine from cache: {e:?}"),
        }
    });
    pub(crate) static FLAG: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("classic/Flag.svg"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("classic/Flag.svg"))
                    .expect("Failed to read Flag from cache")
            }
            Err(e) => panic!("Failed to read Flag from cache: {e:?}"),
        }
    });
}
