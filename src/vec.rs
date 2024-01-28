use raylib_ffi::*;

pub const fn vec3_f32(x: f32, y: f32, z: f32) -> Vector3 {
    return Vector3{x: x, y: y, z: z};
}

pub const fn vec3_usize(x: usize, y: usize, z: usize) -> Vector3 {
    return Vector3{x: x as f32, y: y as f32, z: z as f32};
}

pub const fn vec3_isize(x: isize, y: isize, z: isize) -> Vector3 {
    return Vector3{x: x as f32, y: y as f32, z: z as f32};
}

pub const fn vec3_zero() -> Vector3 {
    return vec3_usize(0, 0, 0);
}

pub const fn vec3_one() -> Vector3 {
    return vec3_usize(1, 1, 1);
}    
