extern crate seis;
use seis::io::grd;
use seis::io::grd::GrdFileType;

fn main() {
    let grd = grd::Grd::from_grd_file(r"F:\seism\廊坊\木里音频大地电磁-靶区预测数据\剖面及解释\网格数据\L2.grd");
    grd.write_file("hello.grd", GrdFileType::Binary);
    println!("{}", grd[(0, 1)]);
}
