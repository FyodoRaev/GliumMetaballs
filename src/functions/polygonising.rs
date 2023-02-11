use crate::functions::linspace;
use super::linspace::Linspace;
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f64, f64, f64)
}

implement_vertex!(Vertex, position);

// I will loop this function and get new and new vertices for polygons
pub fn polygoniseScalarField(linspace: &Linspace) -> Vec<Vertex> {
  // Start marching cubes
  let pointCoordinates = linspace.getVerticesCoordsIndexes(0.0);
  let mut verticesIndexes: Vec<Vertex> = Vec::new();
  for point in pointCoordinates {
    let coordinates:(f64, f64, f64) = (point.x, point.y, point.z); 
    verticesIndexes.push(Vertex {position: coordinates,});
  }
  return verticesIndexes;
}