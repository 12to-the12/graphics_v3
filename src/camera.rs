#![allow(nonstandard_style)]
// use crate::coordinate_space::Orientation;
use crate::geometry::primitives::Vector;

#[derive(Clone)]
pub struct Camera {
    pub position: Vector,
    // orientation: Orientation, // some object that implements get_orientation?
    pub lens: Lens,
    pub sensor: Sensor,
    pub _near_clipping_plane: f32,
    pub _far_clipping_plane: f32,
    /// shutterspeed in seconds
    pub _exposure_time: f32,
}

pub fn _camera(position: Vector, lens: Lens, sensor: Sensor) -> Camera {
    Camera {
        position,
        lens,
        sensor,
        ..CAMERA
    }
}
// impl Default for Camera {
//     fn default() -> Camera {
//         Camera {
//             position: ORIGIN,
//             lens: LENS,
//             sensor: SENSOR,
//             near_clipping_plane: 1e-1,
//             far_clipping_plane: 1e6,
//             shutter_speed: 1.,
//         }
//     }
// }

impl Camera {
    /// get the horizontal field of view in degrees
    pub fn horizontal_field_of_view(&self) -> f32 {
        return ((self.sensor.width / (2.0 * self.lens.focal_length)).atan() * 2.0).to_degrees();
    }
    /// get the vertical field of view in degrees
    pub fn _vertical_field_of_view(&self) -> f32 {
        return ((self.sensor.height() / (2.0 * self.lens.focal_length)).atan() * 2.0).to_degrees();
    }
    /// get the solid angle captured by the lens in steradians
    #[allow(non_snake_case)]
    pub fn _frustrum_solid_angle(&self) -> f32 {
        let a = self.sensor.width;
        let b = self.sensor.height();
        let h = self.lens.focal_length;
        let Ω = 4. * f32::asin((a * b) / (a.powi(2) + h.powi(2) * (b.powi(2) + h.powi(2))));
        return Ω;
    }
    /// the solid angle covered by the average pixel in the view frustrum in steradians
    /// THIS IS AN APPROXIMATION BECAUSE I AM LAZY, in reality pixels on the edges would subtend a smaller solid angle being more oblique
    pub fn _pixel_solid_angle(&self) -> f32 {
        return self._frustrum_solid_angle() / self.sensor._pixels() as f32;
    }
}

/// models a camera lens
/// defines the field of view
#[derive(Clone)]
pub struct Lens {
    /// ƒ-stop is focal length / aperture pupil diameter https://www.wikiwand.com/en/F-number
    pub _aperture: f32,
    /// the field of view, defined in millimeters, degrees is an alternative method
    pub focal_length: f32,
    /// how far from camera plane the focus is, in meters
    /// currently unused
    pub _focus_distance: f32,
}

pub fn _lens(focal_length: f32) -> Lens {
    Lens {
        _aperture: 8.,
        focal_length,
        _focus_distance: 1.,
    }
}

/// models a camera sensor
/// notably used to accurately model the frequency response
/// I would like to implement a number of predefined lenses in addition to offering a generic
#[derive(Clone)]
pub struct Sensor {
    /// the width of the sensor in millimeters
    pub width: f32,
    pub horizontal_res: u32,
    pub vertical_res: u32,
    // put frequency response here
}

pub fn _sensor(width: f32, horizontal_res: u32, vertical_res: u32) -> Sensor {
    Sensor {
        width,
        horizontal_res,
        vertical_res,
    }
}

impl Sensor {
    /// get height in mm
    pub fn height(&self) -> f32 {
        ((self.vertical_res as f32) / (self.horizontal_res as f32)) * self.width
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height()
    }
    pub fn res(&self) -> (u32, u32) {
        (self.horizontal_res, self.vertical_res)
    }
    pub fn _pixels(&self) -> u32 {
        self.horizontal_res * self.vertical_res
    }
    pub fn _sensor_area(&self) -> f32 {
        // the area of the sensor in square millimeters
        self.width * self.height()
    }
    pub fn _pixel_area(&self) -> f32 {
        // the area of a pixel in square millimeters
        let pixel_count = self.horizontal_res * self.vertical_res;
        self._sensor_area() / pixel_count as f32
    }
}

pub const LENS: Lens = Lens {
    _aperture: 12.0,
    focal_length: 50.0,
    _focus_distance: 20.0,
};

pub const SENSOR: Sensor = Sensor {
    width: 36.0,
    // height: 24.0,
    horizontal_res: 1500,
    vertical_res: 1000,
};

pub const CAMERA: Camera = Camera {
    position: Vector {
        x: 0.,
        y: 0.,
        z: 0.,
    },
    lens: LENS,
    sensor: SENSOR,
    _near_clipping_plane: 1e-1,
    _far_clipping_plane: 1e6,
    _exposure_time: 1.,
};

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::geometry::primitives::vector;

    use super::*;
    #[test]
    fn lens_solid_angle() {
        // a camera with infinitesimal focal length
        let cam = _camera(vector(0.0, 0.0, 0.0), _lens(1e-6), _sensor(1., 10, 10));
        // subtends a hemisphere
        assert_eq!(cam._frustrum_solid_angle(), 2. * PI);

        assert_eq!(cam._pixel_solid_angle(), 2. * PI / 100.);

        // let diameter_sun = 865_370.; // miles
        // let au = 9.296e+7; // miles
        // let cam = camera(vertex(0.0, 0.0, 0.0), lens(au), sensor(diameter_sun, 10, 10));
        // assert_eq!(cam.solid_angle(), 6.79e-5);
    }
    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn field_of_view() {
        let mut camera = CAMERA;
        assert_eq!(camera.horizontal_field_of_view().round(), 40.0); // 39.59775
        assert_eq!(camera._vertical_field_of_view().round(), 27.0); //

        camera.lens.focal_length = 30.0;

        assert_eq!(camera.horizontal_field_of_view().round(), 62.0); //
        assert_eq!(camera._vertical_field_of_view().round(), 44.0); //

        camera.lens.focal_length = 18.0;

        assert_eq!(camera.horizontal_field_of_view().round(), 90.0); //
        assert_eq!(camera._vertical_field_of_view().round(), 67.0); //
    }
    #[test]
    fn pixel_size() {
        assert_eq!(SENSOR._sensor_area(), 864.0);
        assert_eq!(SENSOR._pixel_area(), 0.000576);
    }
}
