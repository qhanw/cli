use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use cli::Config;
use serde_json::Value;

pub fn unzip(fname: &str, cfg: &Config) -> i32 {
    let Config {
        temp_origin,
        cache_dir,
        ..
    } = &cfg;

    // 删除缓存目录
    let dir = Path::new(&temp_origin.0);
    let is_exists = dir.exists();

    if is_exists {
        fs::remove_dir_all(dir).expect("文件删除失败！")
    }

    // 忽略文件配置
    let ignore_rules = ignore_rules(read_package_json(&fname));

    let file = fs::File::open(fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => {
                // 更改导出文件名，并过滤掉忽略文件
                let path_arr: Vec<&str> = path.to_str().unwrap().split("-master/").collect();

                let mut is_ignore = false;

                for igr in &ignore_rules {
                    if path_arr[1].to_lowercase().starts_with(&igr.to_lowercase()) {
                        is_ignore = true;
                        break;
                    }
                }

                if is_ignore {
                    continue;
                }

                PathBuf::from(path_arr.join("/"))
            }
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        let output = get_path(cache_dir, &outpath);

        // println!("file: {} ", file.name());

        if (*file.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.display());

            fs::create_dir_all(&output).unwrap();
        } else {
            // println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(get_path(cache_dir, &p.to_owned())).unwrap();
                }
            }
            let mut outfile = fs::File::create(&output).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}

fn get_path(cache_dir: &str, outpath: &PathBuf) -> String {
    format!("{}/{}", cache_dir, &outpath.display())
}

// 获取模板文件 package json 配置文件
fn read_package_json(fname: &str) -> Value {
    let zipfile = std::fs::File::open(fname).unwrap();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    let mut file = archive
        .by_name("ant-design-pro-master/package.json")
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str(&contents).unwrap()
}

// 解析忽略文件规则
fn ignore_rules(package: Value) -> Vec<String> {
    let mut ignores = vec![
        String::from("pnpm-lock.yaml"),
        String::from("public/CNAME"),
        // readme.md
        String::from("README"),
        String::from("src/locales/bn-BD"),
        String::from("src/locales/fa-IR"),
        String::from("src/locales/id-ID"),
        String::from("src/locales/ja-JP"),
        String::from("src/locales/pt-BR"),
        String::from("src/locales/zh-TW"),
    ];

    let ignore = &package["create-umi"];

    for igr in vec!["ignore", "ignoreScript", "ignoreDependencies"] {
        if let Value::Array(ig) = &ignore[igr] {
            for v in ig {
                if let Value::String(p) = v {
                    let next = p.replace(".*", "").replace("*", "");
                    if !ignores.contains(&next) {
                        ignores.push(next);
                    }
                }
            }
        }
    }

    ignores
}
