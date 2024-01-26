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
    set_world_column(world, 4, 4, 4);
    set_world_column(world, 3, 4, 2);
}
