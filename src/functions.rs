// return 350.0 + x.tan() + 100.0 * y.tan() + (x + 50.0) * 4.0; 
// return 180.0 + 100.0 * x.sin() + 100.0 * y.cos() + (x + 100.0) * 4.0; 
// return 180.0 + x.tan() + 100.0 * y.sin() + (x + 10.0) * 20.0; 
// return 300.0 + 200.0 * x.sin() + 200.0 * y.cos(); 
// return 450.0 + 5.0 * (_x / 5.0).sin() - std::f64::consts::E.powf((_x * _x.cos()).ln()) + 5.0 * (_y / 5.0).sin() - std::f64::consts::E.powf((_y * _y.cos()).ln()); 

pub struct WavelengthFunction {
    vertical_interval: (f64, f64),
    horizontal_interval: (f64, f64),
}

impl WavelengthFunction {
    pub fn new() -> Self {
        Self {
            vertical_interval: (-100.0, 100.0),
            horizontal_interval: (-100.0, 100.0),
        }
    }

    pub fn to_vertical_interval(&self, x:i32, width: i32) -> f64 {
        return self.horizontal_interval.0 + x as f64 
            * (self.horizontal_interval.1 - self.horizontal_interval.0) / width as f64;
    }    

    pub fn to_horizontal_interval(&self, y:i32, height: i32) -> f64 {
        return self.vertical_interval.0 + y as f64 
            * (self.vertical_interval.1 - self.vertical_interval.0) / height as f64;
    }

    pub fn get_result(&self, _x: f64, _y: f64) -> f64 {
        return 180.0 + 100.0 * _x.sin() + 100.0 * _y.cos() + (_x + 100.0) * 4.0; 
    }
}

pub struct IntensityFunction {
    vertical_interval: (f64, f64),
    horizontal_interval: (f64, f64),
}

impl IntensityFunction {
    pub fn new() -> Self {
        Self {
            vertical_interval: (-0.5, 0.5),
            horizontal_interval: (-0.5, 0.5),
        }
    }

    pub fn to_vertical_interval(&self, x:i32, width: i32) -> f64 {
        return self.horizontal_interval.0 + x as f64 
            * (self.horizontal_interval.1 - self.horizontal_interval.0) / width as f64;
    }    

    pub fn to_horizontal_interval(&self, y:i32, height: i32) -> f64 {
        return self.vertical_interval.0 + y as f64 
            * (self.vertical_interval.1 - self.vertical_interval.0) / height as f64;
    }

    pub fn get_result(&self, _x: f64, _y: f64) -> f64 {
        return (_x * 500.0).sin().abs() + (_y * 500.0).sin().abs();
    }
}

pub struct SaturationFunction {
    vertical_interval: (f64, f64),
    horizontal_interval: (f64, f64),
}

impl SaturationFunction {
    pub fn new() -> Self {
        Self {
            vertical_interval: (-0.5, 0.5),
            horizontal_interval: (-0.5, 0.5),
        }
    }

    pub fn to_vertical_interval(&self, x:i32, width: i32) -> f64 {
        return self.horizontal_interval.0 + x as f64 
            * (self.horizontal_interval.1 - self.horizontal_interval.0) / width as f64;
    }    

    pub fn to_horizontal_interval(&self, y:i32, height: i32) -> f64 {
        return self.vertical_interval.0 + y as f64 
            * (self.vertical_interval.1 - self.vertical_interval.0) / height as f64;
    }

    pub fn get_result(&self, _x: f64, _y: f64) -> f64 {
        return (_x * 500.0).cos().abs() + (_y * 500.0).cos().abs(); 
    }
}
