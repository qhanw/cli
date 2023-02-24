// extern crate fs_extra;
// use fs_extra::{copy_items_with_progress, dir, TransitProcess};

// pub fn example_copy() {
//     let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
//     let handle = |process_info: TransitProcess| {
//         println!("{} {}", process_info.total_bytes, process_info.copied_bytes);
//         fs_extra::dir::TransitProcessResult::ContinueOrAbort
//     };

//     let mut from_paths = Vec::new();
//     from_paths.push(".temp/ant-design-pro");
//     copy_items_with_progress(&from_paths, "./", &options, handle).unwrap();
// }

use std::{fs, path::Path};

fn copy_all(from: &str, to: String, root_from: &str) -> bool {
    let entries = fs::read_dir(Path::new(from)).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            if let Some(p) = path.to_str() {
                let path_str = &p.replace(&root_from, &to);
                let c_dir = Path::new(path_str);

                if !c_dir.exists() {
                    fs::create_dir_all(c_dir).unwrap();
                }

                copy_all(p, to.clone(), root_from);
            }
        } else if path.is_file() {
            if let Some(p) = path.to_str() {
                let path_str = &p.replace(root_from, &to);
                fs::copy(&path, Path::new(path_str)).unwrap();
            }
        }
    }

    return true;
}

pub fn copy_template(from: String, to: String) -> bool {
    let to_dir = Path::new(&to);
    let is_exists = to_dir.exists();

    if !is_exists {
        fs::create_dir(to_dir).unwrap();

        return copy_all(&from, to, &from);
    } else {
        println!("application {} already exists!", &to);
        return false;
    }
}
