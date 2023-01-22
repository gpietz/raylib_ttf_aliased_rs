use std::path::PathBuf;

pub trait FileUtils {
    fn get_path_str(&self) -> String;
}

impl FileUtils for PathBuf {
    fn get_path_str(&self) -> String {
        let full_path = self.canonicalize();
        if let Ok(cp) = full_path {
            return cp.as_path().display().to_string();
        }
        self.as_path().display().to_string()
    }
}
