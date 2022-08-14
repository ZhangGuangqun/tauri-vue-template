mod model;

use std::io::Write;
use std::path;
use std::fs;
use std::fs::File;

use directories::UserDirs;

use model::*;

const SUCCESS_RESPONSE_CODE: u32 = 200;
const SUCCESS_RESPONSE_DATA: &str = "Success";

const FILE_NAME: &str = "tauri-test.txt";

/// 获取桌面目录
pub fn get_desktop_dir() -> String {
    let user_dirs = UserDirs::new();
    let desktop_dir = match &user_dirs {
        Some(user_dirs) => {
            user_dirs.desktop_dir()
        },
        None => {
            None
        },
    };

    let desktop_dir_string = match desktop_dir {
        Some(desktop_dir) => {
            desktop_dir.to_str().unwrap().to_string()
        },
        None => {
            "".to_string()
        },
    };

    let transfer_data = TransferData {
        code: SUCCESS_RESPONSE_CODE,
        data: desktop_dir_string,
    };

    serde_json::to_string(&transfer_data).unwrap()
}

/// 写文件到指定目录
pub fn wriate_file(write_file_request_string: String) -> String {
    let mut transfer_data = TransferData {
        code: SUCCESS_RESPONSE_CODE,
        data: SUCCESS_RESPONSE_DATA.to_string(),
    };

    let write_file_request : WriteFileRequest = serde_json::from_str(&write_file_request_string).unwrap();
    
    let dest_path_string = write_file_request.dest_path;

    if !path::Path::new(dest_path_string.as_str()).exists() {
        let create_dir_result = fs::create_dir_all(dest_path_string.as_str());

        match create_dir_result {
            Ok(_) => {},
            Err(e) => {
                transfer_data = TransferData {
                    code: 400,
                    data: format!("选择的生成目录没有写入权限，{}", e.to_string()),
                };

                return serde_json::to_string(&transfer_data).unwrap();
            }
        }
    }

    let dest_path = path::Path::new(dest_path_string.as_str());
    let file_result = File::create(dest_path.join(FILE_NAME));

    let mut license_file = match file_result {
        Ok(file) => {
            file
        },
        Err(_) => {
            transfer_data = TransferData {
                code: 400,
                data: "未获取写入文件的权限".to_string(),
            };
            return serde_json::to_string(&transfer_data).unwrap();
        }
    };

    let write_result = license_file.write_all(&write_file_request.content.as_bytes());

    match write_result {
        Ok(_) => (),
        Err(_) => {
            transfer_data = TransferData {
                code: 400,
                data: "未获取写入文件的权限".to_string(),
            };
            return serde_json::to_string(&transfer_data).unwrap();
        }
    }

    serde_json::to_string(&transfer_data).unwrap()
}
