import { invoke } from '@tauri-apps/api/tauri'

// 获取桌面路径
export function getDesktopDir() {
    return invoke('get_desktop_dir_data')
}

// 可以用 tauri 现有的 api 实现： https://tauri.app/v1/api/js/modules/fs#writetextfile
export function writeFile(writeFileRequest) {
    return invoke('wriate_file_action', { writeFileRequest })
}
