use std::{fs, path::Path, process};

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

// mod extract;
// use extract::real_main;

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
                let temp_dir = Path::new(&config.cache_dir).is_dir();
                if temp_dir {
                    println!("create app, name is: {}, {}", name, simple);

                    let clone_exec = format!(
                        "cd {} && curl {} --output temp.zip",
                        config.cache_dir, config.download_address
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

                            // real_main(&file_path);
                        }
                        _ => {
                            println!("template files download failed!");
                            println!("{:?}", output);
                        }
                    }
                } else {
                    fs::create_dir(config.cache_dir).expect("文件夹创建失败！！！");
                }

                // println!("create app, name is: {}, {}", name, simple)
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
