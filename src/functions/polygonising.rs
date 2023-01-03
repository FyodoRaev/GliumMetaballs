use crate::functions::linspace;
use super::linspace::Linspace;
#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);


// I will loop this function and get new and new vertices for polygons
pub fn polygoniseScalarField(linspace: Linspace, metaBallsCenters: Vec<(f32, f32, f32)>, metaBallsRads: Vec<f32>) -> Vec<(Vertex, i32)> {
  // Start marching cubes
  let triangles = linspace.getVerticesCoordsIndexes(metaBallsCenters, metaBallsRads, 40.0);
  return triangles;
}