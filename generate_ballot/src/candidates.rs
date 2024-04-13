use image::{imageops, DynamicImage};
use rand::Rng;

fn h_concat(mut base: DynamicImage, img: &DynamicImage, x: i64, y: i64) -> DynamicImage {
    imageops::overlay(&mut base, img, x, y);
    base
}

pub fn candidate1(img: DynamicImage) {
    let x0 = 233;
    let y0 = 80;
    let rect_width = 645 - x0; // with img 829
    let rect_height = 135 - y0;
    let overlay = image::open("ballot/vote.png").unwrap();
    let mut rng = rand::thread_rng();

    let random_x: i64 = rng.gen_range(x0..(x0 + rect_width));
    let random_y: i64 = rng.gen_range(y0..(y0 + rect_height));

    let base = h_concat(img, &overlay, random_x, random_y);

    let _ = base.save("cropped_image.png");
}
pub fn candidate2(img: DynamicImage) {
    let x0 = 233;
    let y0 = 204;
    let rect_width = 645 - x0; // with img 829
    let rect_height = 250 - y0;
    let overlay = image::open("ballot/vote.png").unwrap();
    let mut rng = rand::thread_rng();

    let random_x: i64 = rng.gen_range(x0..(x0 + rect_width));
    let random_y: i64 = rng.gen_range(y0..(y0 + rect_height));

    let base = h_concat(img, &overlay, random_x, random_y);

    let _ = base.save("cropped_image.png");
}
pub fn candidate3(img: DynamicImage) {
    let x0 = 233;
    let y0 = 315;
    let rect_width = 645 - x0; // with img 829
    let rect_height = 370 - y0;
    let overlay = image::open("ballot/vote.png").unwrap();
    let mut rng = rand::thread_rng();

    let random_x: i64 = rng.gen_range(x0..(x0 + rect_width));
    let random_y: i64 = rng.gen_range(y0..(y0 + rect_height));

    let base = h_concat(img, &overlay, random_x, random_y);

    let _ = base.save("cropped_image.png");
}
pub fn candidate4(img: DynamicImage) {
    let x0 = 233;
    let y0 = 427;
    let rect_width = 645 - x0; // with img 829
    let rect_height = 482 - y0;
    let overlay = image::open("ballot/vote.png").unwrap();
    let mut rng = rand::thread_rng();

    let random_x: i64 = rng.gen_range(x0..(x0 + rect_width));
    let random_y: i64 = rng.gen_range(y0..(y0 + rect_height));

    let base = h_concat(img, &overlay, random_x, random_y);

    let _ = base.save("cropped_image.png");
}
pub fn candidate5(img: DynamicImage) {
    let x0 = 233;
    let y0 = 540;
    let rect_width = 645 - x0; // with img 829
    let rect_height = 600 - y0;
    let overlay = image::open("ballot/vote.png").unwrap();
    let mut rng = rand::thread_rng();

    let random_x: i64 = rng.gen_range(x0..(x0 + rect_width));
    let random_y: i64 = rng.gen_range(y0..(y0 + rect_height));

    let base = h_concat(img, &overlay, random_x, random_y);

    let _ = base.save("cropped_image.png");
}
