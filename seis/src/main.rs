extern crate nalgebra as na;
extern crate seis;

use seis::{
    forward, 
    io::seg,
};
use std::fs;

pub fn run_same_dir() {
    let forward_e = forward::ForwardBridge::new(
        r"G:\毕设数据\Source\FKW-Point-MF.exe",
        r"G:\毕设数据\Model\地堑",
        "graben",
    );

    let mut forward_template = forward::ForwardModelTemplate::default();

    forward_template.set_mod_prefix(forward_e.target_prev());

    let dirs_str = format!("{}\\models", forward_e.holder_dir());

    let dirs = fs::read_dir(&dirs_str).unwrap();

    for dir in dirs {
        let dir_path: std::path::PathBuf = dir.unwrap().path();
        let dir_path_str = dir_path.to_str().unwrap();
        let start = dir_path_str.len();
        let suffix = format!("MF{}", &dir_path_str[start-3..]);
        forward_template.set_mod_suffix(&suffix);
        forward_template.write_fkw(dir_path_str);
    }
}

fn main() {
    // let forward_e = forward::ForwardBridge::new(
    //     r"G:\毕设数据\Source\FKW-Point-MF.exe",
    //     r"G:\毕设数据\Model\地堑",
    //     "graben",
    // );

    // let mut forward_template = forward::ForwardModelTemplate::default();

    // forward.model_ready(forward_template);

    // forward_e.run();

    // let mut forward_template = forward::ForwardModelTemplate::default();

    // forward_template.set_mod_prefix("dx-2m");

    // forward_template.write(r"F:\毕设-研究生\data\dx=2m-Test");

    let seg_cdp = seg::Seg::from_cdp_file(r"G:\毕设数据\Model\地堑\models\graben000\graben-point-z2000.cdp");
    dbg!(seg_cdp);
}
