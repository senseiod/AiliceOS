use super::*;
use alloc::str::from_utf8;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub trait INodeExt {
    /// 打印当前目录的文件
    fn ls(&self);
    fn ls_as_vec(&self) -> Result<Vec<String>, usize>;
    /// 读取文件内容到
    fn read_as_vec(&self) -> Result<Vec<u8>, usize>;
    /// 读文件到字符串
    fn read_as_string(&self) -> Result<String, usize>;
}

impl INodeExt for dyn INode {
    fn ls(&self) {
        let mut id = 0;
        while let Ok(name) = self.get_entry(id) {
            print!(91; "{} \n", name);
            id += 1;
        }
    }
    fn ls_as_vec(&self) -> Result<Vec<String>, usize> {
        let mut res = Vec::new();
        let mut id = 0;
        while let Ok(name) = self.get_entry(id) {
            res.push(name);
            id += 1;
        }
        Ok(res)
    }

    fn read_as_vec(&self) -> Result<Vec<u8>, usize> {
        let size = self.metadata().unwrap().size;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        self.read_at(0, buf.as_mut_slice()).unwrap();
        Ok(buf)
    }

    fn read_as_string(&self) -> Result<String, usize> {
        let data = self.read_as_vec().unwrap();
        match from_utf8(&data) {
            Ok(v) => Ok(v.to_string()),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }
}
