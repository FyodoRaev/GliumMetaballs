use crate::functions::linspace;
use super::linspace::Linspace;
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f64, f64, f64)
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

// I will loop this function and get new and new vertices for polygons
pub fn polygoniseScalarField(linspace: &Linspace, metaBallsCenters: &Vec<(f64, f64, f64)>, metaBallsRads: &Vec<f64>) -> Vec<Vertex> {
  // Start marching cubes
  let pointCoordinates = linspace.getVerticesCoordsIndexes(metaBallsCenters, metaBallsRads, 1.0);
  let mut verticesIndexes: Vec<Vertex> = Vec::new();
  for point in pointCoordinates {

    verticesIndexes.push(Vertex {position: point,});
  }
  return verticesIndexes;
}