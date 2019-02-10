use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::process::Command;

pub struct ForwardBridge {
    exe_file: String,    // source exe file path
    config_file: String, // config file. eg: PARAMETER.txt
    holder_dir: String,  // 目标目录
    target_prev: String, // 目标目录中需要进行正演模拟的文件夹的前缀
    count: usize,
}

impl ForwardBridge {
    pub fn new(
        exe_file: &str,
        config_file: &str,
        holder_dir: &str,
        target_prev: &str,
    ) -> ForwardBridge {
        ForwardBridge {
            exe_file: exe_file.to_string(),
            config_file: config_file.to_string(),
            holder_dir: holder_dir.to_string(),
            target_prev: target_prev.to_string(),
            count: 0,
        }
    }
}

impl ForwardBridge {
    pub fn run(&self) {
        let target_relative_dir = format!("{}{}", self.target_prev, self.count);
        let target_dir = format!("{}\\{}", self.holder_dir, target_relative_dir);

        let path = Path::new(&target_dir);
        dbg!(path);
        if !path.exists() {
            fs::create_dir(path).expect("can't create dir");
        }

        let copy_exe_file = format!("{}\\yzq-point.exe", path.to_str().unwrap());
        let copy_config_file = format!("{}\\PARAMETER.txt", path.to_str().unwrap());
        fs::copy(&self.exe_file, &copy_exe_file).expect("error in copy exe file");
        fs::copy(&self.config_file, &copy_config_file).expect("error in copy config file");
        println!("{}", copy_exe_file);

        let cur_dir = env::current_dir().expect("error in get current dir");
        env::set_current_dir(path).expect("error in set dir");
        let mut exe = Command::new(copy_exe_file);
        let status = exe.status().expect("error in execute forward program");
        dbg!(status);
    }
}

enum PointSourceType {
    Harmomegathus,
    Vertical,
    Horizontal,
}

pub struct ForwardModelTemplate {
    diff_order: u32,
    absorb_boundary_thickness: u32,
    dt: f64,
    points: u32,
    main_freq: f64,
    delay: f64,
    epicentre_x: i32,
    epicentre_z: i32,
    seismometers_z: i32,
    source_type: PointSourceType,
    vp_grd: String,
    vs_grd: String,
    pp_grd: String,
    wavelet_bln: String,
    cdp_x2: String,
    cdp_z2: String,
    wave_field_x: String,
    wave_field_z: String,
    self_start: u32,
    self_end: u32,
}

impl Default for ForwardModelTemplate {
    fn default() -> Self {
        ForwardModelTemplate {
            diff_order: 10,
            absorb_boundary_thickness: 200,
            dt: 0.00025,
            points: 4096,
            main_freq: 10.0,
            delay: 0.1,
            epicentre_x: 101,
            epicentre_z: 1,
            seismometers_z: 1,
            source_type: PointSourceType::Harmomegathus,
            vp_grd: "modvp.grd".to_string(),
            vs_grd: "modvs.grd".to_string(),
            pp_grd: "modpp.grd".to_string(),
            wavelet_bln: "wavelet.bln".to_string(),
            cdp_x2: "mod-point-x2.cdp".to_string(),
            cdp_z2: "mod-point-z2.cdp".to_string(),
            wave_field_x: "mod-point-X.dat".to_string(),
            wave_field_z: "mod-point-Z.dat".to_string(),
            self_start: 80,
            self_end: 120,
        }
    }
}

impl ForwardModelTemplate {
    // pub fn to_str_vec(&self) -> Vec<String> {
    //     let r = Vec::new();

    // }

    fn make_line(args: &[&str]) -> String {
        args.join(",").to_string()
    }

    pub fn write(&self, dir_name: &str) {
        let path = format!("{}\\PARAMETER.txt", dir_name);
        let mut file = fs::File::create(path).expect("error in create parameter file");
        let model_template = vec![
            "!差分阶数，吸收边界厚度\n",
            "!采样间隔，采样点数，震源子波主频，震源子波延迟时\n",
            "!震源所在X、Z方向节点序号，检波点所在Z方向节点序号\n",
            "!点震源的类型：1胀缩震源，2垂直震源，3水平震源\n",
            "!模型的纵波速度\n",
            "!模型的横波速度\n",
            "!模型的密度\n",
            "!保存震源子波的文件名\n",
            "!保存共炮点记录水平分量的文件名\n",
            "!保存共炮点记录垂直分量的文件名\n",
            "!保存波场水平分量的文件名\n",
            "!保存波场垂直分量的文件名\n",
            "!自激自收区域的起点、终点位置\n",
        ];
        dbg!(&model_template);
        let forward_template = ForwardModelTemplate::default();
    }
}
