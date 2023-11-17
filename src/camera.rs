// use crate::coordinate_space::Orientation;
use crate::primitives::{Vertex,vertex};
use std::f32::consts::PI as π;
pub struct Camera {
    pub position: Vertex,
    // orientation: Orientation, // some object that implements get_orientation?
    pub lens: Lens,
    pub sensor: Sensor,
}

/// models a camera lens
/// used to capture the zoom
pub struct Lens {
    pub aperture: f32, // is ƒ-stop
    // ƒ-stop is focal length / aperture pupil diameter https://www.wikiwand.com/en/F-number
    pub focal_length: f32, // the field of view, defined in millimeters, degrees is an alternative method
    pub focus_distance: f32, // how far from camera plane the focus is, in meters
}

/// models a camera sensor
/// notably used to accurately model the frequency response
/// I would like to implememnt a number of predefined lenses in addition to offering a generic
pub struct Sensor {
    pub width: f32,  // the width of the sensor in millimeters
    // pub height: f32, // it might be better to define this in terms of a ratio to the width
    pub horizontal_res: u32,
    pub vertical_res: u32,
    // put frequency response here
}

impl Sensor{
    pub fn height(&self) -> f32{
        ((self.vertical_res as f32) / (self.horizontal_res as f32)) * self.width
    }
    pub fn aspect_ratio(&self) -> f32{
        self.width / self.height()
    }
}
impl Camera {
    pub fn horizontal_field_of_view(&self) -> f32 {
        return (self.sensor.width / (2.0 * self.lens.focal_length)).atan() * 2.0 * (180.0 / π);
    }
    pub fn vertical_field_of_view(&self) -> f32 {
        return (self.sensor.height() / (2.0 * self.lens.focal_length)).atan() * 2.0 * (180.0 / π);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_of_view() {
        let lens = Lens {
            aperture: 12.0,
            focal_length: 50.0,
            focus_distance: 20.0,
        };
        let sensor = Sensor {
            width: 36.0,
            // height: 24.0,
            horizontal_res: 1500,
            vertical_res: 1000,
        };
        let camera = Camera {
            position: vertex(0.0, 0.0, 0.0),
            lens: lens,
            sensor: sensor,
        };
        assert_eq!(camera.horizontal_field_of_view().round(), 40.0); // 39.59775
        assert_eq!(camera.vertical_field_of_view().round(), 27.0); // 39.59775
    }
}
