use std::env;
use std::path::Path;

fn main() {
    let agvs: Vec<String> = env::args().skip(1).collect();

    println!("{:?}", agvs);

    println!("{}", agvs[0].as_str());

    let image_path = Path::new(&agvs[0]);

    let image = image::open(image_path).unwrap();

    let rotated = image.rotate90();

    rotated.save(image_path).unwrap();
}
