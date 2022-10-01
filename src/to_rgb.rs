pub fn to_rgb(wavelength: f64, intensity: f64, saturation: f64) -> (u8, u8, u8) {
    let gamma = 0.8;

    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    let attenuation: f64;

    if wavelength >= 380.0 && wavelength <= 440.0 {
        attenuation = 0.3 + 0.7 * (wavelength - 380.0) / (440.0 - 380.0);
        r = (((-(wavelength - 440.0) / (440.0 - 380.0)) * attenuation) as f64).powf(gamma);
        g = 0.0;
        b = (1.0 * attenuation).powf(gamma);
    } else if wavelength >= 440.0 && wavelength <= 490.0 {
        r = 0.0;
        g = (((wavelength - 440.0) / (490.0 - 440.0)) as f64).powf(gamma);
        b = 1.0;
    } else if wavelength >= 490.0 && wavelength <= 510.0 {
        r = 0.0;
        g = 1.0;
        b = ((-(wavelength - 510.0) / (510.0 - 490.0)) as f64).powf(gamma);
    } else if wavelength >= 510.0 && wavelength <= 580.0 {
        r = (((wavelength - 510.0) / (580.0 - 510.0)) as f64).powf(gamma);
        g = 1.0;
        b = 0.0;
    } else if wavelength >= 580.0 && wavelength <= 645.0 {
        r = 1.0;
        g = ((-(wavelength - 645.0) / (645.0 - 580.0)) as f64).powf(gamma);
        b = 0.0;
    } else if wavelength >= 645.0 && wavelength <= 780.0 {
        attenuation = 0.3 + 0.7 * (780.0 - wavelength) / (780.0 - 645.0);
        r = ((1.0 * attenuation) as f64).powf(gamma);
        g = 0.0;
        b = 0.0;
    } else {
        r = 0.0;
        g = 0.0;
        b = 0.0;
    }

    r *= 255.0 * intensity;
    g *= 255.0 * intensity;
    b *= 255.0 * intensity;

    let rgb = saturate(r, g, b, saturation);

    return (rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);
}

fn saturate(r: f64, g: f64, b: f64, saturation: f64) -> (f64, f64, f64) {
    let mut rgb = vec![r, g, b];
    let max_val = rgb.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    for index in 0..rgb.len() {
        rgb[index] += (max_val - rgb[index]) * (1.0 - saturation);
    }

    return (rgb[0], rgb[1], rgb[2]);
}
