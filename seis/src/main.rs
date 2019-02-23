extern crate seis;
extern crate nalgebra as na;

use seis::io::grd;
use seis::io::grd::GrdFileType;
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

fn run_forward(exe_file: &str, config_file: &str) {
    let m_path =
    r"F:\毕设-研究生\地震波理论课相关软件\地震波理论模拟实习\model1";
    let exe_file = r"F:\毕设-研究生\地震波理论课相关软件\地震波理论模拟实习\yzq-point.exe";
    let config_file = r"F:\毕设-研究生\地震波理论课相关软件\地震波理论模拟实习\PARAMETER.txt";

    let path = Path::new(m_path);
    if !path.exists() {
        fs::create_dir(path).expect("can't create dir");
    }
    println!("{}", path.to_str().unwrap());

    let copy_exe_file = format!("{}\\yzq-point.exe", path.to_str().unwrap());
    let copy_config_file = format!("{}\\PARAMETER.txt", path.to_str().unwrap());
    fs::copy(exe_file, &copy_exe_file).expect("error in copy exe file");
    fs::copy(config_file, &copy_config_file).expect("error in copy config file");
    println!("{}", copy_exe_file);
    //TODO: recover dir
    env::set_current_dir(path).expect("error in set dir");
    let mut exe = Command::new(copy_exe_file);
    println!("{:?}", exe.status());
}

fn main() {
//     let mut model_vp_grd = grd::Grd::from_grd_file(r"F:\毕设-研究生\data\三层水平均匀介质模型\3-layersvp.grd");
//     model_vp_grd.extract(r"F:\毕设-研究生\测试", "3-layersvp", 0, 48, 1, 2);
    let paths = std::fs::read_dir(r"F:\毕设-研究生\data\三层水平均匀介质模型").expect("error in read model grd file.");
    let file_names : Vec<_> = paths
        .map(|path| {
            path.unwrap().path().to_str().unwrap().to_string()
        })
        .filter( |file_name| {
            file_name.ends_with("vp.grd") || file_name.ends_with("vs.grd") || file_name.ends_with("pp.grd")
        })
        .collect();
    dbg!(&file_names);

    let rela_file_names: Vec<_> = file_names
        .iter()
        .map(|s| {
            let ix = s.rfind("\\").unwrap();
            &s[(ix+1)..]
        })
        .collect();

    dbg!(rela_file_names);
}
