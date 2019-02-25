use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::process::Command;
use crate::io::grd::Grd;

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
    fn vp_vs_pp_filename(dir: &str) -> Vec<String> {  //vp vs pp顺序不定
        let paths = std::fs::read_dir(dir).expect("error in read model grd file.");
        let file_names : Vec<_> = paths
            .map(|path| {
                path.unwrap().path().to_str().unwrap().to_string()
            })
            .filter( |file_name| {
                file_name.ends_with("vp.grd") || file_name.ends_with("vs.grd") || file_name.ends_with("pp.grd")
            })
            .collect();
//        assert_eq!(file_names.len() == 3);
        file_names
    }

    pub fn model_ready(&self) {  //模型正演前的准备工作。把grd文件分成小部分方便进行运算
        let vp_vs_pp = ForwardBridge::vp_vs_pp_filename(&self.holder_dir);

        let rela_file_names: Vec<_> = vp_vs_pp
            .iter()
            .map(|s| {
                let ix = s.rfind("\\").unwrap();
                &s[(ix+1)..]
            })
            .collect();

        for filename in vp_vs_pp {  
            let mut grd = Grd::from_grd_file(&filename);
            let ix_start = filename.rfind("\\").unwrap();
            let ix_end = filename.rfind(".").unwrap();

            let epicenter_start_x = 0;
            let traces_num = 48;
            let space = 1;
            let offset = 2;

            let mut forward_template = ForwardModelTemplate::default();

            forward_template.mod_prefix(&self.target_prev);

            grd.extract(&self.holder_dir, &filename[ix_start+1..ix_end], &mut forward_template, epicenter_start_x, traces_num, space, offset);
        }


    }

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
            epicentre_x: 1,
            epicentre_z: 1,
            seismometers_z: 1,
            source_type: PointSourceType::Harmomegathus,
            vp_grd: "modvp.grd".to_string(),
            vs_grd: "modvs.grd".to_string(),
            pp_grd: "modpp.grd".to_string(),
            wavelet_bln: "wavelet.bln".to_string(),
            cdp_x2: "mod-point-x2.cdp".to_string(),
            cdp_z2: "mod-point-z2.cdp".to_string(),
            wave_field_x: "mod-point-X.dat".to_string(), // 波场快照
            wave_field_z: "mod-point-Z.dat".to_string(),
            self_start: 80,
            self_end: 120,
        }
    }
}

impl ForwardModelTemplate {
    fn to_str_vec(&self) -> Vec<String> {
        let mut str_vec = Vec::new();
        str_vec.push(format!("{},{}", self.diff_order, self.absorb_boundary_thickness));
        str_vec.push(format!("{},{},{},{}", self.dt, self.points, self.main_freq, self.delay));
        str_vec.push(format!("{},{},{}", self.epicentre_x, self.epicentre_z, self.seismometers_z));
        let source_value = match self.source_type {
            PointSourceType::Harmomegathus => 1,
            PointSourceType::Vertical => 2,
            PointSourceType::Horizontal => 3,
        };
        str_vec.push(format!("{}", source_value));
        str_vec.push(format!("{}", self.vp_grd));
        str_vec.push(format!("{}", self.vs_grd));
        str_vec.push(format!("{}", self.pp_grd));
        str_vec.push(format!("{}", self.wavelet_bln));
        str_vec.push(format!("{}", self.cdp_x2));
        str_vec.push(format!("{}", self.cdp_z2));
        str_vec.push(format!("{}", self.wave_field_x));
        str_vec.push(format!("{}", self.wave_field_z));
        str_vec.push(format!("{},{}", self.self_start, self.self_end));
        str_vec
    }

    pub fn write(&self, dir_name: &str) {
        let path_str = format!("{}\\PARAMETER.txt", dir_name);
        let path = Path::new(&path_str);
        let mut file = fs::File::create(path).expect("error in create parameter file");
        let model_template = vec![
            "!差分阶数，吸收边界厚度",
            "!采样间隔，采样点数，震源子波主频，震源子波延迟时",
            "!震源所在X、Z方向节点序号，检波点所在Z方向节点序号",
            "!点震源的类型：1胀缩震源，2垂直震源，3水平震源",
            "!模型的纵波速度",
            "!模型的横波速度",
            "!模型的密度",
            "!保存震源子波的文件名",
            "!保存共炮点记录水平分量的文件名",
            "!保存共炮点记录垂直分量的文件名",
            "!保存波场水平分量的文件名",
            "!保存波场垂直分量的文件名",
            "!自激自收区域的起点、终点位置",
        ];
        // let model_template: Vec<_> = model_template.iter().map(|x| x.to_string()).collect();
        // dbg!(&model_template);
        let model_values = self.to_str_vec();
        assert!(model_template.len() == model_values.len());

        for i in 0..model_template.len() {
            write!(file, "{}\r\n", model_template[i]);
            write!(file, "{}\r\n", model_values[i]);
        }
    }

    pub fn epicentre_x(&mut self, value: i32) {
        self.epicentre_x = value;
        let start = match (value - 20) {
            i if i < 0 => 1,
            _ => value - 20,
        };

        let end = value + 20;

        self.self_start = start as u32;
        self.self_end = end as u32;

    }

    pub fn mod_prefix(&mut self, prefix: &str) {
        self.vp_grd = format!("{}vp.grd", prefix);
        self.vs_grd = format!("{}vs.grd", prefix);
        self.pp_grd = format!("{}pp.grd", prefix);
        self.cdp_x2 = format!("{}-point-x2.cdp", prefix);
        self.cdp_z2 = format!("{}-point-z2.cdp", prefix);
        self.wave_field_x = format!("{}-point-X.dat", prefix);  
        self.wave_field_z = format!("{}-point-Z.dat", prefix);
    }
}
