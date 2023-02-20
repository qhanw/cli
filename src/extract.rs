use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub fn unzip(fname: &str, cache_dir: &str) -> i32 {
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => {
                let p = path.strip_prefix("ant-design-pro-master");

                let ps = match p {
                    Ok(a) => Path::new("ant-design-pro").join(a),
                    Err(_) => path.to_owned(),
                };

                println!("ps: {}", &ps.display());

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
