#![feature(slice_rotate, slice_patterns)]

extern crate image;
use std::fs::File;
use std::path::Path;
use image::GenericImage;


fn main() {
    type Pix = image::Rgba<u8>;

    let file_name = std::env::args().nth(1).unwrap();
    let mut img = image::open(&Path::new(&file_name)).unwrap();
    let (nc, nr) = img.dimensions();
    let w = nc as usize;

    let mut mat: Vec<Vec<_>> = (0..nr)
        .map(|r| (0..nc).map(|c| img.get_pixel(c, r)).collect())
        .collect();

    println!("{:?}", mat);
    let is_color = |&Pix { data: [r, g, b, _] }| r != g || r != b;

    for mut row in mat.iter_mut() {
        let pos = (0..w)
            .find(|&i| is_color(&row[i]) && !is_color(&row[(i + w - 1) % w]))
            .unwrap();
        row.rotate((pos + 3) % w);
    }

    mat.sort_by_key(|r| r[w - 1].data[0]);

    for (r, row) in mat.iter().enumerate() {
        for (c, &p) in row.iter().enumerate() {
            img.put_pixel(c as u32, r as u32, p);
        }
    }

    let mut fout = File::create(&Path::new("result.png")).unwrap();
    img.save(&mut fout, image::PNG).unwrap();
}
