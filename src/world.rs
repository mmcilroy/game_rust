use crate::raymath::*;
use crate::vec::*;

use raylib_ffi::*;

pub const WORLD_SIZE: usize = 10;
pub const WORLD_SIZE_SQUARED: usize = WORLD_SIZE * WORLD_SIZE;
pub const WORLD_SIZE_CUBED: usize = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub static mut world_debug: bool = true;

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
    for y in 0 .. h + 1 {
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
    set_world_column(world, 1, 1, 2);
    set_world_column(world, 3, 3, 3);
    set_world_column(world, 4, 4, 5);
    set_world_column(world, 3, 4, 2);
}

pub fn get_distance(origin: f32, direction : f32) -> f32 {
    if direction > 0.0 {
        return (1.0 + origin).floor() - origin;
    } else {
        return origin - (origin - 1.0).ceil()
    }
}

pub fn world_raycast_next(pos: Vector3, dir: Vector3) -> Vector3 {
    // Calculate distances to the next closest grid intersection point on each axis
    let dx = get_distance(pos.x, dir.x) / dir.x;
    let dy = get_distance(pos.y, dir.y) / dir.y;
    let dz = get_distance(pos.z, dir.z) / dir.z;

    // Find the minimum distance
    let min_distance = dx.abs().min(dy.abs().min(dz.abs()));

    // Calculate the position at that distance along the ray
    unsafe {
        return Vector3Add(pos, Vector3Scale(dir, min_distance));
    }
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

pub fn world_raycast(world: &[i8], start: Vector3, end: Vector3) -> Vector3 {
    unsafe {
        if world_debug {
            print!("world_raycast_simple\n");
        }

        let mut pos = start;
        let dir = Vector3Normalize(Vector3Subtract(end, start));

        while pos.x < 10.0 && pos.y < 10.0 && pos.z < 10.0 {
            // Get the next intersection point along the ray
            let next = world_raycast_next(pos, dir);

            // Work out what world space to check
            let wp = vec3_usize(
                round(pos.x.min(next.x), 4) as usize, 
                round(pos.y.min(next.y), 4) as usize, 
                round(pos.z.min(next.z), 4) as usize);

            // Check if we hit something
            let wv = get_world(world, wp.x as usize, wp.y as usize, wp.z as usize);

            if world_debug {
                print!("check world: ({}, {}, {}) / ({}, {}, {}) ? ({}, {}, {}) -> {}\n", pos.x, pos.y, pos.z, next.x, next.y, next.z, wp.x, wp.y, wp.z, wv);
            }

            if wv > 0 {
                if world_debug {
                    print!("hit!\n");
                }
                return pos;
            }

            pos = next;
        }

        return vec3_zero();
    }
}
