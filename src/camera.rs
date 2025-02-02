// use crate::coordinate_space::Orientation;
use crate::primitives::{vertex, Vertex,ORIGIN};

#[derive(Clone)]
pub struct Camera {
    pub position: Vertex,
    // orientation: Orientation, // some object that implements get_orientation?
    pub lens: Lens,
    pub sensor: Sensor,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
    pub shutter_speed: f32, // shutterspeed in seconds
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
    pub fn horizontal_field_of_view(&self) -> f32 {
        return ((self.sensor.width / (2.0 * self.lens.focal_length)).atan() * 2.0).to_degrees();
    }
    pub fn vertical_field_of_view(&self) -> f32 {
        return ((self.sensor.height() / (2.0 * self.lens.focal_length)).atan() * 2.0).to_degrees();
    }
}

/// models a camera lens
/// used to capture the zoom
#[derive(Clone)]
pub struct Lens {
    pub aperture: f32, // is ƒ-stop
    // ƒ-stop is focal length / aperture pupil diameter https://www.wikiwand.com/en/F-number
    pub focal_length: f32, // the field of view, defined in millimeters, degrees is an alternative method
    pub focus_distance: f32, // how far from camera plane the focus is, in meters
}

/// models a camera sensor
/// notably used to accurately model the frequency response
/// I would like to implement a number of predefined lenses in addition to offering a generic
#[derive(Clone)]
pub struct Sensor {
    pub width: f32, // the width of the sensor in millimeters
    // pub height: f32, // it might be better to define this in terms of a ratio to the width
    pub horizontal_res: u32,
    pub vertical_res: u32,
    // put frequency response here
}

impl Sensor {
    pub fn height(&self) -> f32 {
        ((self.vertical_res as f32) / (self.horizontal_res as f32)) * self.width
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height()
    }
    pub fn res(&self) -> (u32, u32) {
        (self.horizontal_res, self.vertical_res)
    }
    pub fn sensor_area(&self) -> f32 { // the area of the sensor in square millimeters
         self.width * self.height()
    }
    pub fn pixel_area(&self) -> f32  { // the area of a pixel in square millimeters
        let pixel_count = self.horizontal_res * self.vertical_res;
        self.sensor_area() / pixel_count as f32
    }
}



const LENS: Lens = Lens {
    aperture: 12.0,
    focal_length: 50.0,
    focus_distance: 20.0,
};
const SENSOR: Sensor = Sensor {
    width: 36.0,
    // height: 24.0,
    horizontal_res: 1500,
    vertical_res: 1000,
};
#[cfg(test)]
mod tests {
    use super::*;

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn field_of_view() {
 
        let mut camera = Camera {
            position: vertex(0.0, 0.0, 0.0),
            lens: LENS,
            sensor: SENSOR,
            near_clipping_plane: 1e-1,
            far_clipping_plane: 1e6,
            shutter_speed: 1.,
        };
        assert_eq!(camera.horizontal_field_of_view().round(), 40.0); // 39.59775
        assert_eq!(camera.vertical_field_of_view().round(), 27.0); //

        camera.lens.focal_length = 30.0;

        assert_eq!(camera.horizontal_field_of_view().round(), 62.0); //
        assert_eq!(camera.vertical_field_of_view().round(), 44.0); //

        camera.lens.focal_length = 18.0;

        assert_eq!(camera.horizontal_field_of_view().round(), 90.0); //
        assert_eq!(camera.vertical_field_of_view().round(), 67.0); //
    }
    #[test]
    fn pixel_size() {
        assert_eq!(SENSOR.sensor_area(), 864.0);
        assert_eq!(SENSOR.pixel_area(), 0.000576);

    }
}
