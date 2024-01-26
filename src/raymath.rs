use raylib_ffi::*;

extern "C" {
    pub fn Vector3Add(v1: Vector3, v2: Vector3) -> Vector3;
    pub fn Vector3AddValue(v: Vector3, add: f32) -> Vector3;
    pub fn Vector3Subtract(v1: Vector3, v2: Vector3) -> Vector3;
    pub fn Vector3Normalize(v: Vector3) -> Vector3;
    pub fn Vector3Scale(v: Vector3, scalar: f32) -> Vector3;
}
