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
    // let forward = forward::ForwardBridge::new(
    //     r"I:\dissertation\source\LUPENGmbNEW-PML.exe", r"I:\dissertation\source\PARAMETER.txt", 
    //     r"I:\dissertation\models\三层水平均匀介质", "3-layers");
 
    // forward.model_ready();

    let mut template = forward::ForwardModelTemplate::default();

    template.mod_prefix("graben");
    template.write(r"F:\毕设-研究生\data\地堑");

    // let mut model_vp_grd = grd::Grd::from_grd_file(r"F:\毕设-研究生\data\三层水平均匀介质模型\3-layersvp.grd");
    // model_vp_grd.extract(r"F:\毕设-研究生\测试", "3-layersvp", 0, 48, 1, 2);
    // let paths = std::fs::read_dir(r"F:\毕设-研究生\data\三层水平均匀介质模型").expect("error in read model grd file.");
    // let file_names : Vec<_> = paths
    //     .map(|path| {
    //         path.unwrap().path().to_str().unwrap().to_string()
    //     })
    //     .filter( |file_name| {
    //         file_name.ends_with("vp.grd") || file_name.ends_with("vs.grd") || file_name.ends_with("pp.grd")
    //     })
    //     .collect();
    // dbg!(&file_names);
    // dbg!(rela_file_names);
}
