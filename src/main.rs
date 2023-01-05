use functions::{linspace::Linspace, polygonising::polygoniseScalarField};

#[macro_use]
extern crate glium;
mod cube;
mod functions;
fn main() {
    let linspace = Linspace::new(0.5, 5.0);
    let mut metaBallsCenters = vec![(1.0,1.0,1.0), (3.0, 3.0, 3.0)];
    let metaBallsRads = vec![3.0, 5.0];
    let velocites = [1.0, -1.0];


    let steps = 3;
    for _ in 0..steps {
        let testVertices = polygoniseScalarField(&linspace, &metaBallsCenters, &metaBallsRads);
        for vertice in testVertices{
        println!("Position: {:#?}", vertice.0.position);
        println!("Index: {}", vertice.1);
        }
        for indx in 0..2{
            metaBallsCenters[indx].0 += velocites[indx];
            metaBallsCenters[indx].1 += velocites[indx];
            metaBallsCenters[indx].2 += velocites[indx];
        }
    }
}