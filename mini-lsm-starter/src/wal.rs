#![allow(dead_code)] // REMOVE THIS LINE after fully implementing this functionality

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use crossbeam_skiplist::SkipMap;
use parking_lot::Mutex;
// 实现wal操作
pub struct Wal {
    file: Arc<Mutex<BufWriter<File>>>,
}

impl Wal {
    // 创建
    pub fn create(_path: impl AsRef<Path>) -> Result<Self> {
        unimplemented!()
    }
    // 恢复
    pub fn recover(_path: impl AsRef<Path>, _skiplist: &SkipMap<Bytes, Bytes>) -> Result<Self> {
        unimplemented!()
    }
    // 放入
    pub fn put(&self, _key: &[u8], _value: &[u8]) -> Result<()> {
        unimplemented!()
    }
    // 批量放入
    /// Implement this in week 3, day 5.
    pub fn put_batch(&self, _data: &[(&[u8], &[u8])]) -> Result<()> {
        unimplemented!()
    }
    // 同步
    pub fn sync(&self) -> Result<()> {
        unimplemented!()
    }
}
