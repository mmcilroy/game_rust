use crate::raymath::*;
use crate::vec::*;
use crate::world::*;

use raylib_ffi::*;

pub fn dda(world: &[i8], start: Vector3, end: Vector3) {
    unsafe {
        let dir = Vector3Normalize(Vector3Subtract(end, start));
        let mut pos: Vector3 = start;

        while pos.x < 9.0 && pos.y < 9.0 && pos.z < 9.0 {
            // how far do we have to travel along dir until we get to next xyz grid position
            let dist_x = ((pos.x as i32 + 1) as f32 - pos.x) / dir.x;
            let dist_y = ((pos.y as i32 + 1) as f32 - pos.y) / dir.y;
            let dist_z = ((pos.z as i32 + 1) as f32 - pos.z) / dir.z;

            // travel along the shortest distance
            if dist_x <= dist_y && dist_x <= dist_z {
                pos = Vector3Add(pos, Vector3Scale(dir, dist_x));
            } else if dist_y <= dist_x && dist_y <= dist_z {
                pos = Vector3Add(pos, Vector3Scale(dir, dist_y));
            } else {
                pos = Vector3Add(pos, Vector3Scale(dir, dist_z));
            }

            // check if we hit something
            let wv = vec3_usize(pos.x as usize, pos.y as usize, pos.z as usize);
            if get_world(world, wv.x as usize, wv.y as usize, wv.z as usize) > 0 {
                //DrawSphere(pos, 0.25, Color{r: 0, g: 0, b: 255, a: 128});
                break;
            }
        }
    }
}
