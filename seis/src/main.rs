extern crate seis;
use seis::io::grd;
use seis::io::grd::GrdFileType;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
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
