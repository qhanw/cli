use std::{path::Path, process};

pub struct Config {
    // 缓存文件地址
    pub cache_dir: String,
    // 模板下载地址
    pub temp_origin: (String, String),

    // 脚手架下载地址 (name, address)
    pub cli_origin: (String, String),
}

impl Config {
    pub fn new() -> Config {
        Config {
            cache_dir: String::from("./.temp"),
            // 仓库地址: https://github.com/ant-design/ant-design-pro.git
            temp_origin: (
                String::from("ant-design-pro"),
                String::from(
                    "https://codeload.github.com/ant-design/ant-design-pro/zip/refs/heads/master",
                ),
            ),
            cli_origin: (
                String::from("cli"),
                String::from("https://github.com/biz-kits/cli.git"),
            ),
        }
    }
}

pub fn download_template(fname: &str, cfg: &Config) -> Option<bool> {
    // TODO: 根据版本号更新最新模板文件
    if Path::new(&fname).exists() {
        return Some(true);
    }

    let clone_exec = format!(
        "cd {} && curl {} --output {}.zip",
        &cfg.cache_dir, &cfg.temp_origin.1, &cfg.temp_origin.0
    );

    // 执行模版文件下载
    let output = process::Command::new("sh")
        .arg("-c")
        .arg(clone_exec)
        .status()
        .expect("failed to execute process");

    // 完成下载后进行文件解压
    match output.code() {
        Some(0) => {
            println!("template files download completed!");
            Some(true)
        }
        _ => {
            println!("template files download failed!");
            println!("{:?}", output);
            None
        }
    }
}
