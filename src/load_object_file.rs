use crate::geometry::primitives::{Mesh, Vertex};
use std::fs;
pub fn load_wavefront_obj(path: String) -> Mesh {
    let data = fs::read_to_string(path).expect("Unable to read file");

    let mut vertices = Vec::new();
    let mut polygons = Vec::new();

    // println!("{}", data);
    for line in data.split("\n") {
        // discards if it's a comment
        if line.starts_with("#") {
        }
        // adds a vertex
        else if line.starts_with("v ") {
            let mut numbers = line
                .trim()
                .strip_prefix("v ")
                .unwrap()
                .split_ascii_whitespace();
            let x: f32 = numbers.next().unwrap().parse().unwrap();
            let y: f32 = numbers.next().unwrap().parse().unwrap();
            let z: f32 = numbers.next().unwrap().parse().unwrap();
            vertices.push(Vertex::new(x, y, z));
        }
        // adds a face
        else if line.starts_with("f ") {
            let numbers = line
                .trim()
                .strip_prefix("f ")
                .unwrap()
                .split_ascii_whitespace();
            let mut polygon: Vec<usize> = Vec::new();
            for number in numbers {
                let number: &str = number.split("/").next().unwrap();
                let mut number: usize = number.parse().unwrap();
                number -= 1; // because obj starts at 1 which is stupid
                polygon.push(number)
            }
            polygons.push(polygon);
        }
    }
    Mesh::new(vertices, polygons)
}
