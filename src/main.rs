use functions::{linspace::Linspace, polygonising::polygoniseScalarField};

#[macro_use]
extern crate glium;
mod cube;
mod functions;
fn main() {
    let linspace = Linspace::new(2.0, 6.0);
    let mut metaBallsCenters = vec![(1.0,1.0,1.0), (3.0, 3.0, 3.0)];
    let metaBallsRads = vec![3.0, 5.0];
    let velocites = [1.0, -1.0];

    let cubes = &linspace.cubes;
    for cube in cubes {
        println!("New cube:");
        for point in cube{
            println!("Cube point: {:#?}", point);
        }
    }
    // testVertices = polygoniseScalarField(&linspace, &metaBallsCenters, &metaBallsRads);
        
}