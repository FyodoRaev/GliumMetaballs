use crate::functions::linspace;
use super::linspace::Linspace;
#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f64, f64, f64)
}

implement_vertex!(Vertex, position);


// I will loop this function and get new and new vertices for polygons
pub fn polygoniseScalarField(linspace: Linspace, metaBallsCenters: Vec<(f64, f64, f64)>, metaBallsRads: Vec<f64>) -> Vec<(Vertex, i32)> {
  // Start marching cubes
  let pointCoordinates = linspace.getVerticesCoordsIndexes(metaBallsCenters, metaBallsRads, 40.0);
  let mut verticesIndexes: Vec<(Vertex, i32)> = Vec::new();
  for point in pointCoordinates {

    verticesIndexes.push((Vertex {position: point.0,}, point.1));
  }
  return verticesIndexes;
}