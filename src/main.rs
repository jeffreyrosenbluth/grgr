use image::*;
use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::{Distribution, Normal};

const SCALE: f32 = 20.0;
const BIAS: f32 = 0.0;

#[allow(dead_code)]
enum Style {
    Darkest,
    Lightest,
    Always,
}

const STYLE: Style = Style::Always;

fn main() {
    let mut rng = SmallRng::seed_from_u64(0);
    let in_img = image::open("images/pond.png").unwrap();
    let width = in_img.width() as i32;
    let height = in_img.height() as i32;
    let normal = Normal::new(BIAS, SCALE * width as f32 / 4000.0).unwrap();
    let mut out_image = image::RgbaImage::new(in_img.width(), in_img.height());
    for x in 0..width {
        for y in 0..height {
            let delta_x = normal.sample(&mut rng).round() as i32;
            let delta_y = normal.sample(&mut rng).round() as i32;
            let x1 = if x + delta_x >= width {
                (x - delta_x).clamp(0, width - 1)
            } else {
                (x + delta_x).clamp(0, width - 1)
            };
            let y1 = if y + delta_y >= height {
                (y - delta_y).clamp(0, height - 1)
            } else {
                (y + delta_y).clamp(0, height - 1)
            };

            let old_pixel = in_img.get_pixel(x as u32, y as u32);
            let new_pixel = in_img.get_pixel(x1 as u32, y1 as u32);
            let pixel = match STYLE {
                Style::Darkest => {
                    if new_pixel.to_luma()[0] < old_pixel.to_luma()[0] {
                        new_pixel
                    } else {
                        old_pixel
                    }
                }
                Style::Lightest => {
                    if new_pixel.to_luma()[0] > old_pixel.to_luma()[0] {
                        new_pixel
                    } else {
                        old_pixel
                    }
                }
                Style::Always => new_pixel,
            };
            out_image.put_pixel(x as u32, y as u32, pixel);
        }
    }
    out_image.save("out_img.png").unwrap();
}
