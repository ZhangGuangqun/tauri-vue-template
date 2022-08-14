use biz_module::get_desktop_dir;
use biz_module::wriate_file;

#[tauri::command]
pub fn get_desktop_dir_data() -> String {
    get_desktop_dir()
}

#[tauri::command]
pub fn wriate_file_action(write_file_request: String) -> String {
    wriate_file(write_file_request)
}
