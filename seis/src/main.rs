extern crate seis;
extern crate nalgebra as na;

use seis::io::grd;
use seis::io::grd::GrdFileType;
use seis::forward;
use seis::io::seg;

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
    // let forward = forward::ForwardBridge::new(
    //     r"F:\毕设-研究生\地震波理论课相关软件\地震波理论模拟实习\LUPENGmbNEW-PML.exe", 
    //     r"F:\毕设-研究生\地震波理论课相关软件\地震波理论模拟实习\PARAMETER.txt", 
    //     r"F:\毕设-研究生\data\地堑", 
    //     "garden");

    // let mut forward_template = forward::ForwardModelTemplate::default();

    // forward_template.set_points(2048);

    // forward.model_ready(forward_template);

    // forward.run();
    
    // dbg!(seg_data);

    let mut forward_template = forward::ForwardModelTemplate::default();

    forward_template.mod_prefix("dx-2m");

    forward_template.write(r"F:\毕设-研究生\data\dx=2m-Test");

}
