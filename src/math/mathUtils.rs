use std::ops::{Add, Mul};
use glm::{dot, min, normalize, sqrt, Vec3, vec3, Vec4};
use rand::{Rng, thread_rng};



pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - *n * dot(*v,*n) * 2.0;
}

pub fn vectorLengthSquared(vec: &Vec3) -> f32 {
    return vec.x * vec.x + vec.y * vec.y + vec.z * vec.z;
}

pub fn isNearlyZero_Vec4(vec: &Vec4, tolerance: Option<f32>) -> bool {
    let zeroTolerance: f32 = tolerance.unwrap_or(1e-8);
    
    return (vec.x < zeroTolerance && vec.x > -zeroTolerance) &&
        (vec.y < zeroTolerance && vec.y > -zeroTolerance) &&
        (vec.z < zeroTolerance && vec.z > -zeroTolerance) &&
        (vec.w < zeroTolerance && vec.w > -zeroTolerance);
}

pub fn isNearlyZero_Vec3(vec: &Vec3, tolerance: Option<f32>) -> bool {
    let zeroTolerance: f32 = tolerance.unwrap_or(1e-8);
    
    return (vec.x < zeroTolerance && vec.x > -zeroTolerance) &&
        (vec.y < zeroTolerance && vec.y > -zeroTolerance) &&
        (vec.z < zeroTolerance && vec.z > -zeroTolerance);
}

pub fn randomVec3() -> Vec3 {
    return Vec3 {
        x: thread_rng().gen(),
        y: thread_rng().gen(),
        z: thread_rng().gen(),
    };
}

pub fn randomInUnitSphere() -> Vec3 {
    loop {
        let p: Vec3 = vec3(thread_rng().gen_range(-1.0..1.0),
                           thread_rng().gen_range(-1.0..1.0),
                           thread_rng().gen_range(-1.0..1.0));
        if vectorLengthSquared(&p) >= 1.0 { continue; }
        return p;
    }
}

pub fn randomUnitVector3() -> Vec3 {
    let vec: Vec3 = randomInUnitSphere();
    let norm: Vec3 = normalize(vec);
    return norm;
}

pub fn refract(uv: &Vec3, n: &Vec3, etaiOverEtat: &f32) -> Vec3 {
    let cosTheta: f32 = f32::min(dot( -*uv,*n), 1.0);
    let rOutPersp: Vec3 = (*uv + *n * cosTheta) * *etaiOverEtat;
    let rOutParallel: Vec3 = *n * -sqrt(f32::abs(1.0 - vectorLengthSquared(&rOutPersp)));
    return rOutParallel + rOutPersp;
}

