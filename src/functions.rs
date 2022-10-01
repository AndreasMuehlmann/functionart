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

    pub fn get_result(&self, x: f64, y: f64) -> f64 {
        return x.powf(2.0) * y; 
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

    pub fn get_result(&self, x: f64, y: f64) -> f64 {
        return 1.0; 
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

    pub fn get_result(&self, x: f64, y: f64) -> f64 {
        return 1.0; 
    }
}
