use crate::drivers::BlockDriverWrapper;
use alloc::sync::Arc;
use inode_ext::INodeExt;
use lazy_static::*;
use rcore_fs::vfs::INode;
use rcore_fs_mountfs::MountFS;
use rcore_fs_sfs::SimpleFileSystem;

pub mod inode_ext;

lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        // 选择第一个块设备
        let device = {
            let blc_dvice = crate::drivers::BLK_DRIVERS
                .read().iter()
                .next().expect("Block device not found")
                .clone();
            let driver = BlockDriverWrapper(
                blc_dvice
            );
            Arc::new(driver)
        };

        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        let rootfs = MountFS::new(sfs);
        let root = rootfs.root_inode();
        test!("Init FileSystem");
        root
    };
}

pub fn init() {
    ROOT_INODE.fs();
}

pub fn read_file(path: &str) {
    if ROOT_INODE.lookup(path).is_err() {
        println!("File not exist!");
    } else {
        let rs = ROOT_INODE.lookup(path).unwrap().read_as_string().unwrap();
        print!(91; "{} \n", rs);
    }
}
