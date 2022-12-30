mod marching_cubes;

const triTable: Vec<Obj> = marching_cubes::tri_table();
const cornerIndexAFromEdge: [i32;12] = marching_cubes::cornerIndexAFromEdge();
const cornerIndexBFromEdge: [i32;12] = marching_cubes::cornerIndexBFromEdge();

pub struct Linspace {
  points: Vec<(f32, f32, f32)>,
  step: f32,
  len: i32,
  cubes: Vec<[(f32,f32,f32); 8]>,
}


impl Linspace {
  fn new(step: f32, len: i32) -> Linspace
  {
    let mut points: Vec<f32,f32,f32>; 
    
    //Creating all points
    for i in (0..len).step_by(step) {
      for j in (0..len).step_by(step){
        for l in (0..len).step_by(step) {
          points.push_back((i,j,l));
        }
      }
    }


    //creating cubes 
   let mut cubePoints: Vec<[(f32,f32,f32); 8]>;
   for i in (0..len).step_by(step) {
      let x: f32 = i;
        for j in (0..len).step_by(step){
          let y: f32 = j;
            for l in (0..len).step_by(step) {
              let z: f32 = l;
              let mut cube = [(x,y,z), (x+step,y,z), (x+step,y,z+step), (x,y,z+step), (x,y+step,z), (x+step,y+step,z), (x+step,y+step,z+step), (x,y+step,z+step)]; 
              cube_points.push_back(cube);
    }
  }
}
    return Linspace {
      points: points,
      step: step,
      len: len,
      cubes: cubePoints,
    }
  }
}


impl Linspace {
  fn getVerticesCoords(f: fn((f32,f32,f32)) -> f32 , threshold: f32) -> Vec<(f32,f32,f32)> {
    let mut vertexPositions: Vec<(f32,f32,f32)>;
    for cube in Linspace.cubes {
      let mut cubeIndex = 0;
      for i in (0..8) {
        let value = f(cube[i]);
        if value < threshold {
          cubeIndex = 1 << i;
      }}
      let triangulation = triTable[cubeIndex];
      for edgeIndex in triangulation { // Ищу координаты ребер формирующих ребро, которое должно быть закрашено согласно триангуляции
        let indexA = cornerIndexAFromEdge[edgeIndex];
        let indexB = cornerIndexBFromEdge[edgeIndex];
        
        // точка на ребре которая должна быть включена в треугольник который нужно закрасить
        let vertexPos: (f32,f32,f32) = (cube[indexA] + cube[indexB]) / 2;
        vertexPositions.push_back(vertexPos);
      }

  }
}
}

fn metaball(point: (f32,f32,f32), circleCenters: Vec<(f32,f32,f32)>, circleRads: Vec<f32>) -> f32 {
  let mut meteballFunc: f32;
  for i in (0..circleCenters.len()){
    meteballFunc += 1/(f32::powf((point.1 - circleCenters[i].1),2.0) + f32::powf((point.2 - circleCenters[i].2),2.0) +f32::powf((point.3 - circleCenters[i].3),2.0));
  } 
  return meteballFunc;
} 





