mod raymath;
mod vec;
mod world;

use crate::raymath::*;
use crate::vec::*;
use crate::world::*;

use raylib_ffi::*;

const TOP_LEFT: Vector3 = vec3_isize(-(WORLD_SIZE as isize), WORLD_SIZE as isize, -(WORLD_SIZE as isize));
const TOP_RIGHT: Vector3 = vec3_isize(WORLD_SIZE as isize, WORLD_SIZE as isize, -(WORLD_SIZE as isize));
const BOTTOM_LEFT: Vector3 = vec3_isize(-(WORLD_SIZE as isize), WORLD_SIZE as isize, WORLD_SIZE as isize);
const BOTTOM_RIGHT: Vector3 = vec3_isize(WORLD_SIZE as isize, WORLD_SIZE as isize, WORLD_SIZE as isize);
 
pub fn main() {
    unsafe {
        let mut world: [i8; WORLD_SIZE_CUBED] = [0; WORLD_SIZE_CUBED];
        let mut start_pos = vec3_f32(7.0, 1.0, 9.0);
        let mut end_pos = TOP_LEFT;

        let mut camera = Camera{
            position: vec3_usize(10, 10, 10),
            target: vec3_usize(WORLD_SIZE/2, 0, WORLD_SIZE/2),
            up: vec3_usize(0, 1, 0),
            fovy: 60.0,
            projection: enums::CameraProjection::Perspective as i32
        };

        InitWindow(1600, 900, rl_str!("game"));
        SetTargetFPS(60);
        DisableCursor();

        init_world(&mut world);
        world_raycast(&world, start_pos, end_pos);
        world_debug = false;

        while !(WindowShouldClose()) {  // Detect window close button or ESC key

            UpdateCamera(&mut camera, enums::CameraMode::ThirdPerson as i32);

            if IsKeyPressed(enums::KeyboardKey::J as i32) { start_pos.x -= 0.25 }
            if IsKeyPressed(enums::KeyboardKey::L as i32) { start_pos.x += 0.25 }
            if IsKeyPressed(enums::KeyboardKey::I as i32) { start_pos.z -= 0.25 }
            if IsKeyPressed(enums::KeyboardKey::K as i32) { start_pos.z += 0.25 }

            if IsKeyPressed(enums::KeyboardKey::One as i32) { end_pos = TOP_LEFT }
            if IsKeyPressed(enums::KeyboardKey::Two as i32) { end_pos = TOP_RIGHT }
            if IsKeyPressed(enums::KeyboardKey::Three as i32) { end_pos = BOTTOM_LEFT }
            if IsKeyPressed(enums::KeyboardKey::Four as i32) { end_pos = BOTTOM_RIGHT }

            BeginDrawing();

                ClearBackground(colors::WHITE);

                BeginMode3D(camera);

                for z in 0 .. WORLD_SIZE {
                    for y in 0 .. WORLD_SIZE {
                        for x in 0 .. WORLD_SIZE {
                            if get_world(&mut world, x, y, z) > 0 {
                                let p = Vector3AddValue(vec3_usize(x, y, z), 0.5);
                                DrawCube(p, 1.0, 1.0, 1.0, colors::GREEN);
                                DrawCubeWires(p, 1.0, 1.0, 1.0, colors::BLACK);
                            }
                        }
                    }
                }

                let hit = world_raycast(&world, start_pos, end_pos);
                DrawSphere(hit, 0.2, Color{r: 255, g: 0, b: 0, a: 64});

                DrawRay(Ray{position: vec3_zero(), direction: vec3_usize(1, 0, 0)}, colors::RED);
                DrawRay(Ray{position: vec3_zero(), direction: vec3_usize(0, 1, 0)}, colors::GREEN);
                DrawRay(Ray{position: vec3_zero(), direction: vec3_usize(0, 0, 1)}, colors::BLUE);

                DrawSphere(start_pos, 0.25, colors::BLACK);
                DrawSphere(end_pos, 1.0, colors::YELLOW);

                DrawLine3D(start_pos, end_pos, colors::BLACK);

                EndMode3D();

                DrawText(format!("({}, {}, {})\0", start_pos.x, start_pos.y, start_pos.z).as_ptr() as *const i8, 10, 10, 20, raylib_ffi::colors::BLACK);
                DrawText(format!("({}, {}, {})\0", hit.x, hit.y, hit.z).as_ptr() as *const i8, 10, 30, 20, raylib_ffi::colors::BLACK);

            EndDrawing();
        }

        CloseWindow();
    }
}
