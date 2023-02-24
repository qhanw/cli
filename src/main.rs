use std::{fs, path::Path};

use clap::{Parser, Subcommand};

mod extract;
use cli::{download_template, Config};
use extract::unzip;

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

mod copy_template;
use copy_template::copy_template;

fn main() {
    let args = Args::parse();

    let cfg = Config::new();

    bootstraps(args, cfg);
}

fn bootstraps(args: Args, cfg: Config) {
    match &args.command {
        Commands::New { name, simple } => {
            if let Some(name) = name {
                // 创建缓存文件夹
                let cache_dir_exists = Path::new(&cfg.cache_dir).exists();
                if !cache_dir_exists {
                    fs::create_dir(&cfg.cache_dir).expect("文件夹创建失败！！！");
                }

                let fname = format!("{}/{}.zip", cfg.cache_dir, cfg.temp_origin.0);

                if let Some(..) = download_template(&fname, &cfg) {
                    let status = unzip(&fname, &cfg);
                    if status == 0 {
                        println!("Starting create app, name is: {}", name);

                        let from_path = format!("{}/{}", cfg.cache_dir, cfg.temp_origin.0);

                        let finished = copy_template(from_path.to_string(), name.to_owned());

                        if finished {
                            println!("completed to create application  {}", name);
                        } else {
                            println!("Failed to create application  {}", name);
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
