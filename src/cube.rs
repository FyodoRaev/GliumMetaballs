#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32)
}

implement_vertex!(Vertex, position);

pub const VERTICES: [Vertex; 9] = [
    Vertex{position: (0.0, 0.0, 0.0)},

    Vertex {position: (-50.5, 0.0, -50.0) }, //1 far left bottom
    Vertex {position: (-50.5, 50.5, -50.0) }, //2
    Vertex {position: (50.5, 50.5, -50.0) }, //3 передний левый верх
    Vertex {position: (50.5, 0.0, -50.0) }, //4
    Vertex {position: (50.5, 0.0, 1.0) }, //5
    Vertex {position: (50.5, 50.5, 1.0) }, //6
    Vertex {position: (-50.5, 0.0, 1.0) }, //7
    Vertex {position: (-50.5, 50.5, 1.0) }, //8
];

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

pub const NORMALS: [Normal; 9] = [
    Normal { normal: (0.0, 0.0, 0.0)},

    Normal { normal: (-0.576, -0.576, -0.576) },  // 1 // I divide all 1s by sqrt(3)
    Normal { normal: (-0.576, 0.576, -0.576) },    // 2
    Normal { normal: (0.576, 0.576, -0.576) },     // 3
    Normal { normal: (0.576, -0.576, -0.576) },    // 4
    Normal { normal: (0.576, -0.576, 0.576) },    // 5
    Normal { normal: (0.576, 0.576, 0.576) },     // 6
    Normal { normal: (-0.576, -0.576, 0.576) },  // 7 
    Normal { normal: (-0.576, 0.576, 0.576) },  // 8 
];

pub const INDICES: [u16; 39] = [
    1, 3, 2,  
    3, 1, 4,
    3, 1, 4,   
    5, 4, 6,
    6, 4, 3,
    5, 7, 6,
    6, 7, 8,
    8, 7, 1,
    1, 8, 2,
    2, 8, 3,
    8, 6, 3,
    1, 7, 4,
    7, 5, 4u16,
];


