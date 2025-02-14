use directories::ProjectDirs;
use std::path::PathBuf;

pub struct AppPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub temp_dir: PathBuf,
}

impl AppPaths {
    pub fn new() -> Option<Self> {
        let proj_dirs = ProjectDirs::from("com", "mc-pixie", "mclauncher")?;
        Some(Self {
            config_dir: proj_dirs.config_dir().to_path_buf(),
            data_dir: proj_dirs.data_dir().to_path_buf(),
            cache_dir: proj_dirs.cache_dir().to_path_buf(),
            temp_dir: std::env::temp_dir(),
        })
    }
}
