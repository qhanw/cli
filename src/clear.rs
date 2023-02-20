use serde_json::Value;

use std::{fs, path::PathBuf};

pub fn remove_files(dir: PathBuf) -> bool {
    let mut ignores = vec![
        String::from("pnpm-lock.yaml"),
        String::from("public/CNAME"),
        // readme.md
        String::from("README.zh-CN.md"),
        String::from("README.tr-TR.md"),
        String::from("README.ru-RU.md"),
        String::from("README.pt-BR.md"),
        String::from("README.ja-JP.md"),
        String::from("README.fr-FR.md"),
        String::from("README.ar-DZ.md"),
        String::from("src/locales/zh-TW"),
        String::from("src/locales/tr-TR"),
        String::from("src/locales/ru-RU"),
        String::from("src/locales/pt-BR"),
        String::from("src/locales/ja-JP"),
        String::from("src/locales/fr-FR"),
        String::from("src/locales/ar-DZ"),
        String::from("src/locales/bn-BD"),
        String::from("src/locales/fa-IR"),
        String::from("src/locales/id-ID"),
        String::from("src/locales/zh-TW.ts"),
        String::from("src/locales/tr-TR.ts"),
        String::from("src/locales/ru-RU.ts"),
        String::from("src/locales/pt-BR.ts"),
        String::from("src/locales/ja-JP.ts"),
        String::from("src/locales/fr-FR.ts"),
        String::from("src/locales/ar-DZ.ts"),
        String::from("src/locales/bn-BD.ts"),
        String::from("src/locales/fa-IR.ts"),
        String::from("src/locales/id-ID.ts"),
    ];

    let contents = fs::read_to_string(dir.join("package.json")).unwrap();

    let package: Value = serde_json::from_str(&contents).unwrap();

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
    println!("ignores: {:#?}", ignores);

    // 开始删除文件
    for igr in ignores {
        let path = dir.join(igr);

        println!("path: {:?}", path);

        if path.exists() {
            if path.is_dir() {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_file(path).unwrap();
            }
        }
    }

    true
}

pub fn replace_package() {}
