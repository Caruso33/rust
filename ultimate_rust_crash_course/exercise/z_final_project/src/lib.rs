use std::fs::create_dir;
use std::path::Path;

pub fn create_output_if_not_exist(output_path: &Path) -> () {
    if !output_path.exists() {
        create_dir(output_path).expect("Can't create output directory");
    };
}
