use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde_json::Value;

pub fn unzip(fname: &str, cache_dir: &str) -> i32 {
    fs::remove_dir_all(Path::new(cache_dir).join("ant-design-pro")).expect("文件删除失败！");

    let ignore_rules = ignore_rules(read_package_json(&fname));
    println!("ignore_rules {:?}", &ignore_rules);

    let file = fs::File::open(fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => {
                // let p = path.to_str().unwrap().to_string().replace("-master", "");

                let p = path.strip_prefix("ant-design-pro-master");

                let ps = match p {
                    Ok(a) => Path::new("ant-design-pro").join(a),
                    Err(_) => path.to_owned(),
                };

                if let Some(x) = ps.to_str() {
                    let mut is_ignore = false;
                    for igr in &ignore_rules {
                        let p = format!("ant-design-pro/{igr}").to_lowercase();

                        if x.to_lowercase().starts_with(&p) {
                            is_ignore = true;
                            break;
                        }
                    }

                    if is_ignore {
                        continue;
                    }
                }

                ps
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

        println!("file: {} ", file.name());

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());

            fs::create_dir_all(&output).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
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

// 忽略文件规则
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
