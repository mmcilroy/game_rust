use crate::raymath::*;
use crate::vec::*;

use raylib_ffi::*;

pub const WORLD_SIZE: usize = 10;
pub const WORLD_SIZE_SQUARED: usize = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_SIZE_CUBED: usize = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub fn world_index(x: usize, y: usize, z: usize) -> usize {
    return z * WORLD_SIZE_SQUARED + y * WORLD_SIZE + x;
}

pub fn set_world(world: &mut [i8], x: usize, y: usize, z: usize, v: i8) {
    let i = world_index(x, y, z);
    if i < WORLD_SIZE_CUBED {
        world[i] = v;
    }
}

pub fn set_world_column(world: &mut [i8], x: usize, z: usize, h: usize) {
    for y in 0 .. h {
        set_world(world, x, y, z, 1);
    }
}

pub fn get_world(world: &[i8], x: usize, y: usize, z: usize) -> i8 {
    let i = world_index(x, y, z);
    if i >= WORLD_SIZE_CUBED {
        return 0;
    }
    return world[i];
}

pub fn init_world(world: &mut [i8]) {
    for z in 0 .. WORLD_SIZE {
        for x in 0 .. WORLD_SIZE {
            set_world(world, x, 0, z, 1);
        }
    }
    set_world_column(world, 3, 3, 3);
    set_world_column(world, 4, 4, 5);
    set_world_column(world, 3, 4, 2);
}

pub fn world_raycast(world: &[i8], start: Vector3, end: Vector3) -> Vector3 {
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
                return pos;
            }
        }

        // this isn't really correct
        // should create a proper struct to hold the result and return
        return vec3_zero();
    }
}
