use crate::forward::ForwardModelTemplate;
use crate::io::ReadEx;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use na::DMatrix;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, SeekFrom};

use std::path::Path;

pub enum GrdFileType {
    // Binary,
    Ascii,
}

#[derive(Debug, Clone)]
pub struct Grd {
    mark: String, //file type
    rows: i32,    //number of rows
    cols: i32,
    xll: f64, //X coordinate of the lower left corner of the grid
    yll: f64,
    x_size: f64, //spacing between adjacent nodes in the X direction (between columns)
    y_size: f64,
    z_min: f64, //minimum Z value within the grid
    z_max: f64,
    pub data: DMatrix<f64>,
}

impl Grd {
    pub fn ellipse(&mut self, x0: f64, y0: f64, a: f64, b: f64, z: f64) {
        let x_min = x0 - a;
        let x_max = x0 + a;
        let y_min = y0 - a;
        let y_max = y0 + a;
        let x_start = x_min.floor() as usize;
        let x_end = x_max.ceil() as usize;
        let y_start = y_min.floor() as usize;
        let y_end = y_max.ceil() as usize;
        for i in x_start..x_end {
            for j in y_start..y_end {
                let if64 = i as f64;
                let jf64 = j as f64;
                let d = (if64 - x0) * (if64 - x0) / (a * a) + (jf64 - y0) * (jf64 - y0) / (b * b);
                if d < 1.0 {
                    self.data[(i, j)] = z;
                }
            }
        }
    }

    /// 炮间距为一个道距
    /// space: 道间距
    /// offset: 偏移距
    /// 48道
    pub fn extract(
        &mut self,
        out_dir: &str,
        out_name: &str, // 无后缀
        forward_template: &mut ForwardModelTemplate,
        epicenter_start_x: i32,
        traces_num: usize,
        step: usize,
    ) {
        // exclusive_range_pattern
        let data = &self.data;

        for i in 0..data.ncols() {
            let start = epicenter_start_x as usize + i * step;
            let ncols = data.ncols();
            // dbg!(ncols - start);
            let sample = match ncols - start {
                v if v >= traces_num && v <= ncols => data.columns(start, traces_num).clone_owned(),
                v if v >= 48 && v < traces_num => data.columns(start, ncols - start).clone_owned(),
                _ => break,
            };

            let mut new_grd = self.clone();  //TODO:优化
            new_grd.rows = sample.nrows() as i32;
            new_grd.cols = sample.ncols() as i32;
            new_grd.x_size = self.x_size();
            new_grd.y_size = self.y_size();
            new_grd.xll = start as f64;
            new_grd.yll = self.yll;
            new_grd.z_min = sample.min() as f64;
            new_grd.z_max = sample.max() as f64;
            std::mem::replace(&mut new_grd.data, sample);
            let split_point = out_name.len() - 2;
            let out_name_front = &out_name[..split_point]; //模型名字
            let tmp_str = format!("{}\\{}{:03}", out_dir, out_name_front, start);
            let out_dir_path = Path::new(&tmp_str);
            if !out_dir_path.exists() {
                fs::create_dir(out_dir_path).expect("error in create grd outdir");
            }
            let out_sub_dir = out_dir_path.file_name().unwrap().to_str().unwrap();

            let start_str = format!("{:03}", start);
            // let out_name_end = &out_name[split_point..]; //模型类型vp/vs/pp
            let rela_name = format!("{}{}", out_name, start_str);

            let out_path = format!(
                "{}\\{}\\{}.grd",
                out_dir, out_sub_dir, rela_name
            );

            forward_template.set_mod_suffix(&start_str);
            forward_template.write_fkw(&tmp_str);
            // dbg!(&out_path);
            new_grd.write_file(&out_path, GrdFileType::Ascii);
        }
    }
}

impl Grd {
    pub fn from_grd_file(filename: &str) -> Grd {
        let mut grd_file = BufReader::new(File::open(filename).expect("error in opening grd file"));
          
        let mark = grd_file.read_str(4);

        match mark.as_str() {
            "DSAA" => Grd::from_ascii_file(grd_file, mark),
            _ => Grd::from_binary_file(grd_file, mark),
        }
    }

    pub fn write_file(&self, filename: &str, file_type: GrdFileType) {
        // binary file not supported yet.
        let grd_file = File::create(filename).expect("error in creating grd file");

        match file_type {
            GrdFileType::Ascii => self.write_ascii_file(grd_file),
            _ => self.write_ascii_file(grd_file),
        }
    }

    fn write_ascii_file(&self, mut grd_file: File) {
        let x_end = self.x_size * (self.cols - 1) as f64 + self.xll;
        let y_end = self.y_size * (self.rows - 1) as f64 + self.yll;
        writeln!(grd_file, "DSAA");
        writeln!(grd_file, "{} {}", self.cols, self.rows);
        writeln!(grd_file, "{} {}", self.xll, x_end);
        writeln!(grd_file, "{} {}", self.yll, y_end);
        writeln!(grd_file, "{} {}", self.z_min, self.z_max);

        let data = &self.data;
        let rows = data.nrows();
        let cols = data.ncols();

        let mut str_vec = Vec::new();
        for i in (0..rows).rev() {
            let row_data = data.row(i);
            let mut vec_data = vec![0.0f64; cols];
            for j in 0..cols {
                vec_data[j] = row_data[j];
            }
            let tmp_vec: Vec<_> = vec_data.iter().map(|x| x.to_string()).collect();
            let tmp_str = tmp_vec.join("   ");
            str_vec.push(tmp_str);
        }
        // let vec: Vec<String> = self
        //     .data
        //     .as_slice()
        //     .iter()
        //     .map(|x| x.clone().to_string())
        //     .collect();
        writeln!(grd_file, "{}", str_vec.join("\n"));
    }

    fn write_binary_file(&self, mut grd_file: File) {
        // not complete
        grd_file.write(&b"DSRB"[..]);
        grd_file.write(&vec![0u8; 16]);
        grd_file.write_i32::<LittleEndian>(self.rows).unwrap();
        grd_file.write_i32::<LittleEndian>(self.cols).unwrap();
        grd_file.write_f64::<LittleEndian>(self.xll).unwrap();
        grd_file.write_f64::<LittleEndian>(self.yll).unwrap();
        grd_file.write_f64::<LittleEndian>(self.x_size).unwrap();
        grd_file.write_f64::<LittleEndian>(self.y_size).unwrap();
        grd_file.write_f64::<LittleEndian>(self.z_min).unwrap();
        grd_file.write_f64::<LittleEndian>(self.z_max).unwrap();
        grd_file.write(&vec![0u8; 24]);
        let data_slice = self.data.as_slice();
        for i in 0..data_slice.len() {
            grd_file.write_f64::<LittleEndian>(data_slice[i]).unwrap();
        }
    }

    fn from_binary_file(mut grd_file: BufReader<File>, mark: String) -> Grd {
        grd_file
            .seek(SeekFrom::Start(20))
            .expect("Error in seek to 20");

        let rows = grd_file.read_i32::<LittleEndian>().unwrap();

        let cols = grd_file.read_i32::<LittleEndian>().unwrap();

        let xll = grd_file.read_f64::<LittleEndian>().unwrap();

        let yll = grd_file.read_f64::<LittleEndian>().unwrap();

        let x_size = grd_file.read_f64::<LittleEndian>().unwrap();

        let y_size = grd_file.read_f64::<LittleEndian>().unwrap();

        let z_min = grd_file.read_f64::<LittleEndian>().unwrap();

        let z_max = grd_file.read_f64::<LittleEndian>().unwrap();

        grd_file
            .seek(SeekFrom::Start(100))
            .expect("Error in seek file to 100");

        let mut data: DMatrix<f64>;
        unsafe {
            data = DMatrix::new_uninitialized(cols as usize, rows as usize);
        }

        for i in 0usize..rows as usize {
            // column-major matrix
            for j in 0usize..cols as usize {
                let _v_buf = [0u8; 8];
                data[(i, j)] = grd_file.read_f64::<LittleEndian>().unwrap();
            }
        }

        Grd {
            mark,
            rows,
            cols,
            xll,
            yll,
            x_size,
            y_size,
            z_min,
            z_max,
            data,
        }
    }

    fn from_ascii_file(grd_file: BufReader<File>, _mark: String) -> Grd {
        let buf_reader = BufReader::new(grd_file);

        let mut lines = buf_reader.lines();

        let _x = vec![1, 2, 3];
        let mark = Grd::process_error(lines.next());
        let (cols, rows) = Grd::process_split::<i32>(&Grd::process_error(lines.next()));
        let (xll, xend) = Grd::process_split::<f64>(&Grd::process_error(lines.next()));
        let (yll, yend) = Grd::process_split::<f64>(&Grd::process_error(lines.next()));
        let (z_min, z_max) = Grd::process_split::<f64>(&Grd::process_error(lines.next()));
        let x_size = (xend - xll) / (cols - 1) as f64;
        let y_size = (yend - yll) / (rows - 1) as f64;

        let mut r: Vec<_> = lines.map(|f| f.unwrap()).collect();
        r.reverse();
        let data_str = r.join(" ");
        let data_intern: Vec<f64> = data_str
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        let data = DMatrix::from_row_slice(rows as usize, cols as usize, &data_intern); // column-major matrix.

        Grd {
            mark,
            rows,
            cols,
            xll,
            yll,
            x_size,
            y_size,
            z_min,
            z_max,
            data,
        }
    }

    fn process_error(line: Option<Result<String, Error>>) -> String {
        line.unwrap().unwrap()
    }

    fn process_split<U>(s: &str) -> (U, U)
    where
        U: std::str::FromStr,
        <U as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let ss: Vec<&str> = s
            .trim()
            .split(char::is_whitespace)
            .filter(|item| *item != "")
            .collect();
        let v1 = ss[0].parse::<U>().unwrap();
        let v2 = ss[1].parse::<U>().unwrap();
        (v1, v2)
    }

    pub fn rows(&self) -> i32 {
        self.rows
    }

    pub fn cols(&self) -> i32 {
        self.cols
    }

    pub fn xll(&self) -> f64 {
        self.xll
    }

    pub fn yll(&self) -> f64 {
        self.yll
    }

    pub fn x_size(&self) -> f64 {
        self.x_size
    }

    pub fn y_size(&self) -> f64 {
        self.y_size
    }

    pub fn z_min(&self) -> f64 {
        self.z_min
    }

    pub fn z_max(&self) -> f64 {
        self.z_max
    }
}

//弃用
//内部用column-major方式存储数据，因此要进行转换。
//impl Index<(usize, usize)> for Grd {
//    type Output = f64;
//
//    fn index(&self, index: (usize, usize)) -> &f64 {
//        assert!(
//            index.0 < self.rows as usize && index.1 < self.cols as usize,
//            "index out of bounds."
//        );
//        &self.data[(index.1, index.0)]
//    }
//}
//
//impl IndexMut<(usize, usize)> for Grd {
//    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64 {
//        assert!(
//            index.0 < self.rows as usize && index.1 < self.cols as usize,
//            "index out of bounds."
//        );
//        &mut self.data[(index.1, index.0)]
//    }
//}
