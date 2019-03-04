use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use na::DMatrix;

use std::fs::File;
use std::io::{BufReader, Cursor, Error, Read, SeekFrom, Write};
use std::mem;
use std::path::Path;

#[derive(Debug)]
pub struct Seg {
    shot: f32,
    offset: f32,
    dx: f32,
    n: i16,                 // 道数n
    m: i16,                 // 点数m
    dt: f32,                // 采样间隔
    pub data: DMatrix<f32>, //for debug
}

impl Seg {
    pub fn from_cdp_file<P: AsRef<Path>>(path: P) -> Seg {
        let mut file = BufReader::new(File::open(path).expect("error in opening grd file"));
        file.seek_relative(2);
        let shot = file.read_f32::<LittleEndian>().unwrap();
        let offset = file.read_f32::<LittleEndian>().unwrap();
        let dx = file.read_f32::<LittleEndian>().unwrap();
        let n = file.read_i16::<LittleEndian>().unwrap();
        let m = file.read_i16::<LittleEndian>().unwrap();
        let dt = file.read_f32::<LittleEndian>().unwrap();

        let nn = n as usize; // i16 type multiply cause overflow.
        let mm = m as usize;

        let mut data_vec = vec![0.0f32; nn * mm];
        let mut bytes_vec = vec![0; nn * mm * mem::size_of::<f32>()];
        file.read_exact(&mut bytes_vec).expect("error in read cdp bytes");
        let mut bytes_cursor = Cursor::new(bytes_vec);

        for v_mut in data_vec.iter_mut() {
            *v_mut = bytes_cursor.read_f32::<LittleEndian>().unwrap();
        }

        let data = DMatrix::from_column_slice(m as usize, n as usize, &data_vec);

        Seg {
            shot,
            offset,
            dx,
            n,
            m,
            dt,
            data,
        }
    }

    pub fn write_cdp_file<P: AsRef<Path>>(&self, path: P) {
        let mut file = File::create(path).expect("error in create cdp file");
        file.write_all(&b"TX"[..]).unwrap();
        file.write_f32::<LittleEndian>(self.shot).unwrap();
        file.write_f32::<LittleEndian>(self.offset).unwrap();
        file.write_f32::<LittleEndian>(self.dx).unwrap();
        file.write_i16::<LittleEndian>(self.n).unwrap();
        file.write_i16::<LittleEndian>(self.m).unwrap();
        file.write_f32::<LittleEndian>(self.dt).unwrap();
        let data = self.data.as_slice();
        let data_bytes_pointer = &data[0] as *const f32 as *const u8;
        let data_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(data_bytes_pointer, data.len() * mem::size_of::<f32>())
        };
        file.write_all(data_bytes).unwrap();
    }
}

impl Seg {
    // pub fn extract()
}
