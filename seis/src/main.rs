extern crate nalgebra as na;
extern crate seis;

use seis::forward;

struct ForwardBridge {
    exe_file: String,      // source exe file path
    config_file: String,   // config file. eg: PARAMETER.txt
    target_dir: String,    // 目标目录
    generate_prev: String, // 目标目录中需要进行正演模拟的文件夹的前缀
}

fn main() {
    let forward = forward::ForwardBridge::new(
        r"G:\毕设数据\Source\FKW-Point-MB-R.exe",
        r"G:\毕设数据\Model\地堑",
        "graben",
    );

    let mut forward_template = forward::ForwardModelTemplate::default();

    forward.model_ready(forward_template);

    forward.run("FKW-Point-MB-R.exe");

    // let mut forward_template = forward::ForwardModelTemplate::default();

    // forward_template.set_mod_prefix("dx-2m");

    // forward_template.write(r"F:\毕设-研究生\data\dx=2m-Test");
}
