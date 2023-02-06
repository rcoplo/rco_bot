use std::path::{Path, PathBuf};
use rbatis::rbdc::uuid;

pub fn get_resources_path(path: Vec<&str>) -> String {
    let mut vec = vec!["resources"];
    for x in path {
        vec.push(x);
    }
    join_paths(vec)
}

pub fn file_tmp_random_image_path(name: &str, ext: &str, file: &[&str]) -> String {
    match file.len() {
        0 => {
            let string = uuid::Uuid::new().0;
            let mut string1 = string.replace("-", "_");
            string1.push_str(format!("_{}.{}", name, ext).as_str());
            get_resources_path(vec!["tmp",string1.as_str()])
        }
        _ => {
            let mut vec = vec!["resources"];
            for x in file {
                vec.push(x);
            }
            let mut string2 = join_paths(vec);
            let string = uuid::Uuid::new().0;
            let mut string1 = string.replace("-", "_");
            string1.push_str(format!("_{}.{}", name, ext).as_str());
            string2.push_str(string1.as_str());
            string2
        }
    }

}

fn join_paths<P: AsRef<Path>>(paths: Vec<P>) -> String {
    match paths.len() {
        0 => String::default(),
        _ => {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            for x in paths {
                path = path.join(x);
            }
            return path.to_str().unwrap().to_string();
        }
    }
}