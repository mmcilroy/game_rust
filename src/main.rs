mod vec;
mod raymath;
mod world;
mod dda;

use crate::vec::*;
use crate::raymath::*;
use crate::world::*;
use crate::dda::*;

use raylib_ffi::*;

pub fn main() {
    unsafe {
        let mut world: [i8; WORLD_SIZE_CUBED] = [0; WORLD_SIZE_CUBED];
        let mut start_pos = vec3_usize(0, 1, 0);
        let mut end_pos = vec3_usize(WORLD_SIZE, WORLD_SIZE, WORLD_SIZE);

        init_world(&mut world);

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

        while !(WindowShouldClose()) {  // Detect window close button or ESC key

            UpdateCamera(&mut camera, enums::CameraMode::ThirdPerson as i32);

            if IsKeyDown(enums::KeyboardKey::J as i32) { start_pos.x -= 0.1 }
            if IsKeyDown(enums::KeyboardKey::L as i32) { start_pos.x += 0.1 }
            if IsKeyDown(enums::KeyboardKey::I as i32) { start_pos.z -= 0.1 }
            if IsKeyDown(enums::KeyboardKey::K as i32) { start_pos.z += 0.1 }

            BeginDrawing();

                ClearBackground(colors::WHITE);

                BeginMode3D(camera);

                dda(&world, start_pos, end_pos);

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

                DrawRay(Ray{position: vec3_zero(), direction: vec3_usize(1, 0, 0)}, colors::RED);
                DrawRay(Ray{position: vec3_zero(), direction: vec3_usize(0, 1, 0)}, colors::GREEN);
                DrawRay(Ray{position: vec3_zero(), direction: vec3_usize(0, 0, 1)}, colors::BLUE);

                DrawSphere(start_pos, 0.25, colors::BLACK);
                DrawSphere(end_pos, 1.0, colors::YELLOW);

                DrawLine3D(start_pos, end_pos, colors::BLACK);

                EndMode3D();

            EndDrawing();
        }

        CloseWindow();
    }
}
