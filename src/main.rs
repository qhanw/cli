use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 子命令配置
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Create new app", long_about = "new create xxx xx xxxx xxxx")]
    New {
        name: Option<String>,

        #[arg(short, long, default_value_t = false)]
        simple: bool,
    },
    #[command(about = "Delete an app")]
    Del { name: Option<String> },

    #[command(about = "Update options")]
    Up {
        #[arg(long, default_value_t = false)]
        auth: bool,
    },
}

fn main() {
    let args = Args::parse();

    handle_args(args);
}

struct Config {
    cache_dir: String,
    temp_address: String,
    cli_address: String,
    download_address: String,
}

mod extract;
use extract::unzip;

mod clear;
use clear::*;

fn handle_args(args: Args) {
    let config = Config {
        cache_dir: String::from("./.temp"),
        temp_address: String::from("https://github.com/ant-design/ant-design-pro.git"),
        cli_address: String::from("https://github.com/biz-kits/cli.git"),
        download_address: String::from(
            "https://codeload.github.com/ant-design/ant-design-pro/zip/refs/heads/master",
        ),
    };

    match &args.command {
        Commands::New { name, simple } => {
            if let Some(name) = name {
                // 创建缓存文件夹
                let cache_dir_exists = Path::new(&config.cache_dir).exists();
                if !cache_dir_exists {
                    fs::create_dir(&config.cache_dir).expect("文件夹创建失败！！！");
                }

                println!("create app, name is: {}, {}", name, simple);

                let temp_exists = Path::new(&config.cache_dir).join("temp.zip").exists();

                if temp_exists {
                    let file_path = format!("{}/temp.zip", config.cache_dir);
                    let status = unzip(&file_path, &config.cache_dir);

                    if status == 0 {
                        remove_files(Path::new(&config.cache_dir).join("ant-design-pro"));

                        //  根据版本号下载最新模板
                        // fs::remove_file(&file_path).expect("文件删除失败！");
                    }
                } else {
                    let clone_exec = format!(
                        "cd {} && curl {} --output temp.zip",
                        &config.cache_dir, &config.download_address
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

                            let file_path = format!("{}/temp.zip", config.cache_dir);
                            let status = unzip(&file_path, &config.cache_dir);

                            if status == 0 {

                                // fs::remove_file(&file_path).expect("文件删除失败！");
                            }
                        }
                        _ => {
                            println!("template files download failed!");
                            println!("{:?}", output);
                        }
                    }
                }
            } else {
                println!("please enter an application name.");
            }
        }
        Commands::Del { name } => {
            println!("delete app, name is: {:?}", name)
        }
        Commands::Up { auth } => {
            println!("update auth: {:?}", auth)
        }
    }

    println!("{:?}", args);
}
