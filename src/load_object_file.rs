use crate::primitives::{mesh, vector, vertex, Mesh};
use std::fs;
pub fn load_obj(path: String) -> Mesh {
    let data = fs::read_to_string(path).expect("Unable to read file");

    let mut vertices = Vec::new();
    let mut polygons = Vec::new();

    // println!("{}", data);
    for line in data.split("\n") {
        // discards if it's a comment
        if line.starts_with("#") {
            // println!("discarding comment")
        }
        // adds a vertex
        else if line.starts_with("v ") {
            // println!("{}", line);

            let mut numbers = line
                .trim()
                .strip_prefix("v ")
                .unwrap()
                .split_ascii_whitespace();
            let x: f32 = numbers.next().unwrap().parse().unwrap();
            let y: f32 = numbers.next().unwrap().parse().unwrap();
            let z: f32 = numbers.next().unwrap().parse().unwrap();
            vertices.push(vertex(x, y, z));
        }
        // adds a face
        else if line.starts_with("f ") {
            // println!("{}", line);

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
    mesh(vector(0., 0., 0.), vertices, polygons)
}
