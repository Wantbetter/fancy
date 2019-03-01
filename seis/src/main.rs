extern crate seis;
extern crate nalgebra as na;

use seis::io::grd;
use seis::io::grd::GrdFileType;
use seis::forward;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

struct ForwardBridge {
    exe_file: String,      // source exe file path
    config_file: String,   // config file. eg: PARAMETER.txt
    target_dir: String,    // 目标目录
    generate_prev: String, // 目标目录中需要进行正演模拟的文件夹的前缀
}

fn main() {
    let forward = forward::ForwardBridge::new(
        r"F:\毕设-研究生\地震波理论课相关软件\地震波理论模拟实习\LUPENGmbNEW-PML.exe", 
        r"F:\毕设-研究生\地震波理论课相关软件\地震波理论模拟实习\PARAMETER.txt", 
        r"F:\毕设-研究生\data\地堑", 
        "garden");

    let mut forward_template = forward::ForwardModelTemplate::default();

    forward_template.set_points(2048);

    forward.model_ready(forward_template);

    forward.run();
    // let dirs = fs::read_dir(r"F:\毕设-研究生\data\地堑\garden").expect("can't read model dir");
    // for dir in dirs {
    //     if dir.is_ok() {
    //         let path = dir.unwrap().path();
    //         if path.file_name().unwrap().to_str().unwrap().starts_with("graben") && path.is_dir() {
    //             dbg!(path.to_str());
    //         }
    //     }
    // }
}
