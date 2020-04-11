use async_std::task;

use std::fs;
use std::io::{self, Write};
use std::path;
use std::str::FromStr;

use crate::os::HOSTS_PATH;
use surf::Exception;

const LATEST_VERSION_URL: &'static str =
    "https://raw.githubusercontent.com/andraantariksa/bebasin/master/misc/version";
// TODO
const UPDATE_URL: &'static str =
    "";

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

pub fn update() {
    task::block_on(async {
        let file_content = surf::get(UPDATE_URL)
            .recv_bytes()
            .await
            .expect(&format!(
                "Error when retrieving {} file content",
                UPDATE_URL
            ));
        let current_file_path =
            std::env::current_exe().expect("Error when retrieving current file path");
        let mut temp_file_path =
            std::env::current_exe().expect("Error when retrieving current file path");
        temp_file_path.set_extension("tmp");
        let current_file_dir_path =
            std::env::current_dir().expect("Error when retrieving current file direcotry path");

        fs::rename(current_file_path, temp_file_path).unwrap();

        fs::File::create("bebasin.tmp").expect("Error when creating temprorary file");
        fs::write("bebasin.tmp", file_content)
            .expect("Error when writing to the hosts temprorary file");
    });
}
