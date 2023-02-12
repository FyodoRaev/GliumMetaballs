extern crate nalgebra_glm as glm;
use glm::{dot, floor, normalize, vec3, TVec3};
extern crate rand;
use rand::{Rng};

use crate::functions::floatIterator::FloatIterator;
use crate::functions::marching_cubes;
static triTable: &[&[usize]] = marching_cubes::TABLE;
static cornerIndexAFromEdge: &[usize] = marching_cubes::cornerIndexAFromEdge;
static cornerIndexBFromEdge: &[usize] = marching_cubes::cornerIndexBFromEdge;

pub struct Linspace {
    step: f64,
    len: f64,
    cubes: Vec<[TVec3<f64>; 8]>,
}

impl Linspace {
    pub fn new(step: f64, len: f64) -> Linspace {
        //creating cubes
        let mut cubes: Vec<[TVec3<f64>; 8]> = Vec::new();
        for i in FloatIterator::new_with_step(-len / 2.0, len / 2.0, step) {
            let x: f64 = i;
            for j in FloatIterator::new_with_step(-len / 2.0, len / 2.0, step) {
                let y: f64 = j;
                for l in FloatIterator::new_with_step(-len / 5.0, len / 10.0, step) {
                    let z: f64 = l;
                    let cube = [
                        vec3(x, y, z),
                        vec3(x + step, y, z),
                        vec3(x + step, y, z + step),
                        vec3(x, y, z + step),
                        vec3(x, y + step, z),
                        vec3(x + step, y + step, z),
                        vec3(x + step, y + step, z + step),
                        vec3(x, y + step, z + step),
                    ];
                    cubes.push(cube);
                }
            }
        }
        return Linspace { step, len, cubes };
    }
}

impl Linspace {
    pub fn getVerticesCoordsIndexes(&self, threshold: f64, time: f64) -> Vec<TVec3<f64>> {
        let mut vertexPositions: Vec<TVec3<f64>> = Vec::new();
        let cubes = &self.cubes;
        for cube in cubes {
            let mut cubeIndex = 0;
            for i in 0..8 {
                let power = i as u32;
                let mut value = noise(cube[i] / 16.0 + vec3(time * 0.37, time * -0.15, time * -0.1828))
                    + noise(cube[i] / 8.0 + vec3(time * -0.2718, time * -0.14182, time * 0.314)) * 0.5 
                    + noise(cube[i] / 4.0 + vec3(time * 0.1, time * -0.1, time * 0.1)) * 0.25; //adding time value to make process more dynamic
                if value > threshold {
                    cubeIndex += u32::pow(2, power) as usize; //
                }
            }
            let triangulation = triTable[cubeIndex];
            for edgeIndex in triangulation {
                // Ищу координаты ребер формирующих ребро, которое должно быть закрашено согласно триангуляции
                let indexA = cornerIndexAFromEdge[*edgeIndex];
                let indexB = cornerIndexBFromEdge[*edgeIndex];

                // точка на ребре которая должна быть включена в треугольник который нужно закрасить
                let vertexPos = (cube[indexA] + cube[indexB]) / 2.0;
                vertexPositions.push(vertexPos);
            }
        }
        return vertexPositions;
    }
}

pub fn grad(coord: TVec3<f64>) -> TVec3<f64> {
    let mut rng = rand::thread_rng();
    let mut v: TVec3<f64> = vec3(rng.gen(), rng.gen(), rng.gen());
    v = normalize(&v);
    return v;
}

pub fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

pub fn noise(p: TVec3<f64>) -> f64 {
    let p0 = floor(&p);
    let p1 = p0 + vec3(1.0, 0.0, 0.0);
    let p2 = p0 + vec3(0.0, 1.0, 0.0);
    let p3 = p0 + vec3(1.0, 1.0, 0.0);
    let p4 = p0 + vec3(0.0, 0.0, 1.0);
    let p5 = p4 + vec3(1.0, 0.0, 0.0);
    let p6 = p4 + vec3(0.0, 1.0, 0.0);
    let p7 = p4 + vec3(1.0, 1.0, 0.0);

    let g0 = grad(p0);
    let g1 = grad(p1);
    let g2 = grad(p2);
    let g3 = grad(p3);
    let g4 = grad(p4);
    let g5 = grad(p5);
    let g6 = grad(p6);
    let g7 = grad(p7);

    let t0: f64 = p.x - p0.x;
    let fade_t0 = fade(t0); /* Used for interpolation in horizontal direction */

    let t1 = p.y - p0.y;
    let fade_t1 = fade(t1); /* Used for interpolation in vertical direction. */

    let t2 = p.z - p0.z;
    let fade_t2 = fade(t2);

    /* Calculate dot products and interpolate.*/
    let p0p1 = (1.0 - fade_t0) * g0.dot(&(p - p0)) + fade_t0 * g1.dot(&(p - p1)); /* between upper two lattice points */
    let p2p3 = (1.0 - fade_t0) * g2.dot(&(p - p2)) + fade_t0 * g3.dot(&(p - p3)); /* between lower two lattice points */

    let p4p5 = (1.0 - fade_t0) * g4.dot(&(p - p4)) + fade_t0 * g5.dot(&(p - p5)); /* between upper two lattice points */
    let p6p7 = (1.0 - fade_t0) * g6.dot(&(p - p6)) + fade_t0 * g7.dot(&(p - p7)); /* between lower two lattice points */

    let y1 = (1.0 - fade_t1) * p0p1 + fade_t1 * p2p3;
    let y2 = (1.0 - fade_t1) * p4p5 + fade_t1 * p6p7;

    /* Calculate final result */
    return (1.0 - fade_t2) * y1 + fade_t2 * y2;
}
