use async_std::task;

use std::fs;
use std::io::{self, Write};
use std::path;
use std::str::FromStr;

fn path() -> &'static path::Path {
    if cfg!(target_os = "linux") {
        path::Path::new("/etc/hosts")
    } else {
        path::Path::new("C:\\Windows\\System32\\drivers\\etc\\hosts")
    }
}

fn install() {
    let content = include_str!("../misc/hosts");
    let path = path();

    fs::write(path, content).expect("Error when write hosts");
}

pub fn is_installed() -> bool {
    fs::read_to_string(path())
        .expect("Error when reading hosts file")
        .contains("# # Bebasin")
}

pub fn backup_path() -> std::path::PathBuf {
    let home_dir = directories::UserDirs::new();
    let mut backup_file_path = home_dir.unwrap().home_dir().to_owned();
    backup_file_path.push("hosts.bak");
    backup_file_path
}

fn backup() {
    fs::copy(path(), backup_path()).expect("Error when backup hosts");
}

fn restore() {
    fs::remove_file(path()).expect("Error when removing hosts file");
    fs::copy(backup_path(), path()).expect("Error when restoring hosts");
}

// Update flow
//             +----------------+
//             |bebasin(.exe)   |
//             |Current version |
//             +--------+-------+
//                      |
//                      | User requesting for update
//                      |       Update coming
//                      |              |
//                      |     +--------+-------+
//                      |     |bebasin.tmp     |
//                      |     |Temporary file  |
//                      |     +--------+-------+
//                      |             /
//                      |            /
//                    /-            /
//                /---             /
//             /--                /
//         /---                  /
// +-----+-----+                /
// |bebasin.old|               /
// |Old version|              /
// +-----+-----+             /
//       |                  /
//       |          +------+-------+
//       |          |bebasin(.exe) |
//       |          |Newer version |
//       |          +--------------+
//       |
//       |
//       X
//    Removed
fn update() {
    task::block_on(async {
        let file_content = surf::get(crate::UPDATE_URL)
            .recv_bytes()
            .await
            .expect(&format!(
                "Error when retrieving {} file content",
                crate::UPDATE_URL
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
