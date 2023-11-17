use crate::primitives::{Vector, Vertex, vector, vertex};

pub fn compile_transforms(transforms: &Vec<Transform>) -> Transform {
    // TO BE REPLACED, THIS IS A SHITTY WAY TO DO THIS
    transforms.first().unwrap().clone()
}

pub fn apply_transform(vertices: &mut Vec<Vertex>, transform: Transform) {
    match transform {
        Transform::Translation(vector) => {
            for vertex in vertices {
                vertex.add(&vector);
                // println!("{:?}",vertex);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Transform {
    Translation(Vector), // will be associated with a Matrix object instead
                         // Rotation,
                         // Scale
}


#[cfg(test)]
mod tests {
    use crate::primitives::vertex;

    use super::*;
    #[test]
    fn test_translate() {
        let myvertex = vertex(1.0, 2.0, 3.0);
        let mut myvertexlist = vec![myvertex];
        let offset = vector(-1.0,3.0,-7.3);
        let mytransform = Transform::Translation(offset);
        apply_transform(&mut myvertexlist, mytransform);
        assert_eq!(myvertexlist[0],vertex(0.0, 5.0, -4.3));
        assert_ne!(myvertexlist[0],vertex(1.0, 2.0, 3.0));
    }
}
