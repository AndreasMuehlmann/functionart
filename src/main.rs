extern crate raster;

use raster::Image;
use raster::Color;

use raster::editor;
use raster::BlendMode;
use raster::PositionMode;

use raster::transform;
use raster::TransformMode;

mod functions;
mod to_rgb;


fn main() {
    let width: i32 = 1000;
    let height: i32 = 1000;

    let mut wavelengths: Vec<f64> = Vec::new();
    let wave_func = functions::WavelengthFunction::new();

    let mut intensity: Vec<f64> = Vec::new();
    let inten_func = functions::IntensityFunction::new();

    let mut saturation: Vec<f64> = Vec::new();
    let sat_func = functions::SaturationFunction::new();

    for x in 0..width {
        for y in 0..height {
            let horizontal_input = wave_func.to_horizontal_interval(x, width);
            let vertical_input = wave_func.to_vertical_interval(y, height);
            wavelengths.push(wave_func.get_result(horizontal_input, vertical_input));

            let vertical_input = inten_func.to_horizontal_interval(y, height);
            let horizontal_input = inten_func.to_vertical_interval(x, width);
            intensity.push(inten_func.get_result(horizontal_input, vertical_input));

            let horizontal_input = sat_func.to_horizontal_interval(x, width);
            let vertical_input = sat_func.to_vertical_interval(y, height);
            saturation.push(sat_func.get_result(horizontal_input, vertical_input));
        }
    }
    let mut rgbs: Vec<(u8, u8, u8)> = Vec::new();
    for i in 0..wavelengths.len() {
        let rgb = to_rgb::to_rgb(wavelengths[i],
                                 intensity[i], saturation[i]);
        rgbs.push(rgb);
    }

    save_image(rgbs, width, height);

    let image1 = raster::open("results/sin_cos.png").unwrap();
    let mut image2 = raster::open("results/sin_tan_lin.png").unwrap();
    transform::flip(&mut image2, TransformMode::Vertical).unwrap();

    let image3 = editor::blend(&image1, &image2, BlendMode::Screen, 0.3, PositionMode::Center, 0, 0).unwrap();
    raster::save(&image3, "results/functionart_blended.png").unwrap();
}

fn save_image(rgbs: Vec<(u8, u8, u8)>, width: i32, height: i32) {
    let mut canvas = Image::blank(width, height);

    let mut count: usize = 0;
    for x in 0..width {
        for y in 0..height {
            let rgb = rgbs[count];
            canvas.set_pixel(x, y, Color::rgba(
                    rgb.0, rgb.1, rgb.2, 255)).unwrap();
            count += 1;
        }
    }
    raster::save(&canvas, "results/functionart_result.png").unwrap();
}
