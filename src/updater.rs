use async_std::task;
use std::fs;
use std::io::{self, Write};
use std::path;
use std::str::FromStr;
use crate::os::{HOSTS_PATH, HOSTS_BACKUP_PATH};
use crate::{LATEST_VERSION_URL, UPDATE_URL, CURRENT_VERSION};
use surf::Exception;
use std::path::Path;
use crate::parser::{parse_from_file, write_to_file, ErrorKind};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

pub fn is_backed() -> bool {
    Path::new(HOSTS_BACKUP_PATH).exists()
}

pub fn backup() -> Result<(), ErrorKind> {
    match parse_from_file(HOSTS_PATH) {
        Ok(hosts_local) => match write_to_file(HOSTS_BACKUP_PATH, &hosts_local, include_str!("../misc/header-backup")) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        },
        Err(err) => Err(err)
    }
}

pub fn get_latest_version() -> Result<u64, String> {
    task::block_on(async {
        match surf::get(LATEST_VERSION_URL)
            .recv_string()
            .await {
            Ok(str) => match str.parse::<u64>() {
                Ok(num) => Ok(num),
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string())
        }
    })
}

pub fn is_updatable() -> Result<bool, String> {
    match get_latest_version() {
        Ok(version_latest) => Ok(version_latest > CURRENT_VERSION),
        Err(err) => Err(err)
    }
}

#[derive(Serialize, Deserialize)]
struct ReleaseAssets {
    name: String,
    size: u32,
    browser_download_url: String
}

#[derive(Serialize, Deserialize)]
struct Release {
    assets: Vec<ReleaseAssets>
}

fn md5_digest<R: std::io::Read>(mut reader: R) -> Result<md5::Digest, ()> {
    let mut context = md5::Context::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.consume(&buffer[..count]);
    }

    Ok(context.compute())
}

fn get_md5_digest() {
    let file = std::fs::File::open("test").unwrap();
    let digest = md5_digest(file);
    println!("Digest {:?}", digest);
}

pub fn update() -> Result<(), ErrorKind> {
    task::block_on(async {
        match surf::get("https://api.github.com/repos/andraantariksa/anime4k-rs/releases/latest")
            .recv_json::<Release>()
            .await {
            Ok(data) => {
                if cfg!(target_os = "windows") {

                } else {
                    for asset in data.assets {
                        if !asset.name.contains(".exe") {
                            // let redir = surf::get(asset.browser_download_url).recv_string().await.unwrap();
                            // let url = redir.split('"').collect::<Vec<&str>>()[1];
                            let mut binary = Arc::new(Mutex::new(Vec::new()));
                            let mut curli = curl::easy::Easy::new();
                            curli.url(&asset.browser_download_url).unwrap();
                            // curli.cookie_file("cookie").unwrap();
                            // curli.cookie_session(true).unwrap();
                            curli.follow_location(true).unwrap();
                            {
                                let mut handler = curli.transfer();
                                handler.write_function(|data| {
                                    binary.lock().unwrap().extend_from_slice(data);
                                    Ok(data.len())
                                }).unwrap();
                                handler.perform().unwrap();
                            }
                            // let binary = surf::get(url).recv_bytes().await.unwrap();
                            let file_path_update = "test";
                            let mut file_created = fs::File::create(file_path_update).unwrap();
                            file_created.write(binary.lock().unwrap().as_slice());

                            let exe_name = &std::env::current_exe().unwrap();

                            let permission = nix::sys::stat::stat(file_path_update).unwrap().st_mode;
                            let mut permission_mode: nix::sys::stat::Mode = nix::sys::stat::Mode::from_bits_truncate(permission);
                            permission_mode.insert(nix::sys::stat::Mode::S_IRWXU);

                            nix::unistd::unlink(exe_name);

                            use std::os::unix::io::IntoRawFd;
                            let f_fd = std::fs::File::open(file_path_update).unwrap().into_raw_fd();
                            nix::sys::stat::fchmod(f_fd, permission_mode);

                            get_md5_digest();
                        }
                    }
                }
                // let current_file_path =
                //     std::env::current_exe().expect("Error when retrieving current file path");
                // let mut temp_file_path =
                //     std::env::current_exe().expect("Error when retrieving current file path");
                // temp_file_path.set_extension("tmp");
                // let current_file_dir_path =
                //     std::env::current_dir().expect("Error when retrieving current file direcotry path");
                //
                // fs::rename(current_file_path, temp_file_path).unwrap();
                //
                // fs::File::create("bebasin.tmp").expect("Error when creating temprorary file");
                // fs::write("bebasin.tmp", file_content)
                //     .expect("Error when writing to the hosts temprorary file");
                Ok(())
            },
            Err(err) => Err(ErrorKind::SurfException(err))
        }
    })
}
