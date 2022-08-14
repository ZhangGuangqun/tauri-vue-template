use serde::{Deserialize, Serialize};

// 传到前端的通用数据结构体 model
#[derive(Serialize, Deserialize)]
pub struct TransferData<T> {
    pub code: u32,
    pub data: T,
}

// 写文件请求 model
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct WriteFileRequest {
    pub content: String,
    pub dest_path: String
}
