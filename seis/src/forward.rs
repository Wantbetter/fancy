use std::env;
use std::fs;
use std::io::prelude::*;

use std::path::Path;
use std::process::Command;
use std::time;

use crate::io::grd::Grd;

pub struct ForwardBridge {
    exe_file: String,    // source exe file path
    holder_dir: String,  // 目标目录
    target_prev: String, // 目标目录中需要进行正演模拟的文件夹的前缀
}

impl ForwardBridge {
    pub fn new(exe_file: &str, holder_dir: &str, target_prev: &str) -> ForwardBridge {
        ForwardBridge {
            exe_file: exe_file.to_string(),
            holder_dir: holder_dir.to_string(),
            target_prev: target_prev.to_string(),
        }
    }
}

impl ForwardBridge {
    fn vp_vs_pp_filename(dir: &str) -> Vec<String> {
        //vp vs pp顺序不定
        let paths = std::fs::read_dir(dir).expect("error in read model grd file.");
        let file_names: Vec<_> = paths
            .map(|path| path.unwrap().path().to_str().unwrap().to_string())
            .filter(|file_name| {
                file_name.ends_with("vp.grd")
                    || file_name.ends_with("vs.grd")
                    || file_name.ends_with("pp.grd")
            })
            .collect();
        //        assert_eq!(file_names.len() == 3);
        file_names
    }

    pub fn model_ready(&self, mut forward_template: ForwardModelTemplate) {
        //模型正演前的准备工作。把grd文件分成小部分方便进行运算
        let vp_vs_pp = ForwardBridge::vp_vs_pp_filename(&self.holder_dir);

        // dbg!(&vp_vs_pp);

        let models_dir = format!("{}\\models", self.holder_dir);
        let models_dir = Path::new(&models_dir);

        if models_dir.exists() {
            fs::remove_dir_all(models_dir);
        }
        fs::create_dir(models_dir).expect("error in create models dir");

        for filename in vp_vs_pp {
            let mut grd = Grd::from_grd_file(&filename);
            let ix_start = filename.rfind('\\').unwrap();
            let ix_end = filename.rfind('.').unwrap();

            let epicenter_start_x = 0;
            let traces_num = 120;
            let step = 2; // 炮间距

            forward_template.set_mod_prefix(&self.target_prev);

            grd.extract(
                models_dir.to_str().unwrap(),
                &filename[ix_start + 1..ix_end],
                &mut forward_template,
                epicenter_start_x,
                traces_num,
                step,
            );
        }
    }

    pub fn run_range(&self, start: usize, end: usize) {
        // 前闭后闭
        let target_dir = format!("{}\\models", self.holder_dir);

        for i in start..end {
            let dir = format!("{}\\{}{:03}", target_dir, self.target_prev, i);
            let path = Path::new(&dir);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with(&self.target_prev) && path.is_dir() {
                let copy_exe_file = format!("{}\\LUPENGmbNEW-PML.exe", path.to_str().unwrap());
                fs::copy(&self.exe_file, &copy_exe_file).expect("error in copy exe file");
                let cur_dir = env::current_dir().expect("error in get current dir");
                env::set_current_dir(&path).expect("error in set dir");
                let mut exe = Command::new(copy_exe_file);
                let now = time::Instant::now();

                let output = exe.output().expect("error in execute forward program");
                let status = output.status;

                let elapsed = now.elapsed();
                let sec =
                    (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);

                if status.success() {
                    println!("{} completed, using {} secends", file_name, sec);
                }
                env::set_current_dir(cur_dir).expect("error in recover current dir");
            }
        }
    }

    pub fn run(&self, exe_file_name: &str) {
        let target_dir = format!("{}\\models", self.holder_dir);

        let dirs = fs::read_dir(target_dir).expect("can't read model dir");
        for dir in dirs {
            if dir.is_ok() {
                let path = dir.unwrap().path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with(&self.target_prev) && path.is_dir() {
                    let copy_exe_file = format!("{}\\{}", path.to_str().unwrap(), exe_file_name);
                    fs::copy(&self.exe_file, &copy_exe_file).expect("error in copy exe file");
                    let cur_dir = env::current_dir().expect("error in get current dir");
                    env::set_current_dir(&path).expect("error in set dir");
                    let mut exe = Command::new(copy_exe_file);
                    let now = time::Instant::now();

                    let output = exe.output().expect("error in execute forward program");
                    let status = output.status;

                    let elapsed = now.elapsed();
                    let sec = (elapsed.as_secs() as f64)
                        + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);

                    if status.success() {
                        println!("{} completed, using {} secends", file_name, sec);
                    }
                    env::set_current_dir(cur_dir).expect("error in recover current dir");
                }
            }
        }
    }
}

pub enum PointSourceType {
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
    save_or_not: i32,
    vp_grd: String,
    vs_grd: String,
    pp_grd: String,
    wavelet_bln: String,
    cdp_x2: String,
    cdp_z2: String,
    wave_field_x: String,
    wave_field_z: String,
    self_start: i32,
    self_end: i32,
    mode_prefix: String,
}

impl Default for ForwardModelTemplate {
    fn default() -> Self {
        ForwardModelTemplate {
            diff_order: 10,
            absorb_boundary_thickness: 200,
            dt: 0.0003,
            points: 2048,
            main_freq: 25.0,
            delay: 0.04,
            epicentre_x: 0,
            epicentre_z: 1,
            seismometers_z: 1,
            source_type: PointSourceType::Vertical,
            save_or_not: 0,
            vp_grd: "modvp.grd".to_string(),
            vs_grd: "modvs.grd".to_string(),
            pp_grd: "modpp.grd".to_string(),
            wavelet_bln: "wavelet.bln".to_string(),
            cdp_x2: "mod-point-x2.cdp".to_string(),
            cdp_z2: "mod-point-z2.cdp".to_string(),
            wave_field_x: "mod-point-X.dat".to_string(), // 波场快照
            wave_field_z: "mod-point-Z.dat".to_string(),
            self_start: 1,
            self_end: 21,
            mode_prefix: "mod".to_string(),
        }
    }
}

impl ForwardModelTemplate {
    fn to_str_vec_fkw(&self) -> Vec<String> {
        let mut str_vec = Vec::new();
        str_vec.push(format!(
            "{},{}",
            self.diff_order, self.absorb_boundary_thickness
        ));
        str_vec.push(format!(
            "{},{},{},{}",
            self.dt, self.points, self.main_freq, self.delay
        ));
        str_vec.push(format!(
            "{},{},{}",
            self.epicentre_x, self.epicentre_z, self.seismometers_z
        ));
        let source_value = match self.source_type {
            PointSourceType::Harmomegathus => 1,
            PointSourceType::Vertical => 2,
            PointSourceType::Horizontal => 3,
        };
        str_vec.push(format!("{}", source_value));
        str_vec.push(format!("{}", self.save_or_not));
        str_vec.push(format!("{}", self.vp_grd));
        str_vec.push(format!("{}", self.vs_grd));
        str_vec.push(format!("{}", self.pp_grd));
        str_vec.push(format!("{}", self.wavelet_bln));
        str_vec.push(format!("{}", self.cdp_x2));
        str_vec.push(format!("{}", self.cdp_z2));
        str_vec.push(format!("{}", self.wave_field_x));
        str_vec.push(format!("{}", self.wave_field_z));
        str_vec
    }

    pub fn write_fkw(&self, dir_name: &str) {
        let path_str = format!("{}\\fkwPARAMETER.txt", dir_name);
        let path = Path::new(&path_str);
        let mut file = fs::File::create(path).expect("error in create parameter file");
        let model_template = vec![
            "!差分阶数，吸收边界厚度",
            "!采样间隔，采样点数，震源子波主频，震源子波延迟时",
            "!震源所在X、Z方向节点序号，检波点所在Z方向节点序号",
            "!点震源的类型：1胀缩震源，2垂直震源，3水平震源",
            "!1保存波场数据，0不保存波场数据",
            "!模型的纵波速度",
            "!模型的横波速度",
            "!模型的密度",
            "!保存震源子波的文件名",
            "!保存共炮点记录水平分量的文件名",
            "!保存共炮点记录垂直分量的文件名",
            "!保存波场水平分量的文件名",
            "!保存波场垂直分量的文件名",
        ];
        // let model_template: Vec<_> = model_template.iter().map(|x| x.to_string()).collect();
        // dbg!(&model_template);
        let model_values = self.to_str_vec_fkw();
        assert!(model_template.len() == model_values.len());

        for i in 0..model_template.len() {
            write!(file, "{}\r\n", model_template[i]);
            write!(file, "{}\r\n", model_values[i]);
        }
    }

    fn to_str_vec(&self) -> Vec<String> {
        let mut str_vec = Vec::new();
        str_vec.push(format!(
            "{},{}",
            self.diff_order, self.absorb_boundary_thickness
        ));
        str_vec.push(format!(
            "{},{},{},{}",
            self.dt, self.points, self.main_freq, self.delay
        ));
        str_vec.push(format!(
            "{},{},{}",
            self.epicentre_x, self.epicentre_z, self.seismometers_z
        ));
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
        let path_str = format!("{}\\fkwPARAMETER.txt", dir_name);
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

    pub fn set_epicentre_x(&mut self, value: i32) {
        self.epicentre_x = value;
    }

    pub fn set_points(&mut self, value: u32) {
        self.points = value;
    }

    pub fn set_dt(&mut self, value: f64) {
        self.dt = value;
    }

    pub fn set_mod_prefix(&mut self, prefix: &str) {
        self.mode_prefix = prefix.to_string();
        self.vp_grd = format!("{}vp.grd", prefix);
        self.vs_grd = format!("{}vs.grd", prefix);
        self.pp_grd = format!("{}pp.grd", prefix);
        self.cdp_x2 = format!("{}-point-x2.cdp", prefix);
        self.cdp_z2 = format!("{}-point-z2.cdp", prefix);
        self.wave_field_x = format!("{}-wave-X.dat", prefix);
        self.wave_field_z = format!("{}-wave-Z.dat", prefix);
    }

    pub fn set_mod_suffix(&mut self, suffix: &str) {
        self.vp_grd = format!("{}vp{}.grd", self.mode_prefix, suffix);
        self.vs_grd = format!("{}vs{}.grd", self.mode_prefix, suffix);
        self.pp_grd = format!("{}pp{}.grd", self.mode_prefix, suffix);
        self.cdp_x2 = format!("{}-point-x2{}.cdp", self.mode_prefix, suffix);
        self.cdp_z2 = format!("{}-point-z2{}.cdp", self.mode_prefix, suffix);
        self.wave_field_x = format!("{}-wave-X{}.dat", self.mode_prefix, suffix);
        self.wave_field_x = format!("{}-wave-Z{}.dat", self.mode_prefix, suffix);
    }
}
