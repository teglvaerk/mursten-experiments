extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;
extern crate rand;

use mursten::{Application, Backend, Data};
use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_blocks::camera::{Camera, CameraUpdater, GetCamera};
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_blocks::input::{Key, KeyboardEvent, OnKeyboard, KeyboardUpdater};
use mursten_blocks::mesh_renderer::{GetMeshes, IntoMesh, MeshRenderer};
use mursten_vulkan_backend::VulkanBackend;
use std::time::Duration;

use rand::distributions::normal::Normal;
use rand::distributions::IndependentSample;

use nalgebra::*;


pub fn main() {
    let backend = VulkanBackend::new();
    let scene = Scene::new();
    Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(CameraUpdater::new())
        .add_updater(KeyboardUpdater::new())
        .add_renderer(MeshRenderer::new())
        .run(scene);
}

struct Scene {
    clock: Clock,
    paused: bool,
    next_change: Duration,
    player: Player,
    floor: Vec<Platform>,
    skybox: Skybox,
    cube: Cube,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            clock: Clock::new(),
            paused: false,
            next_change: Duration::from_secs(1),
            player: Player::new(Point3::origin()),
            floor: {
                let mut v = Vec::new();
                for z in -10..10 {
                    for x in -10..10 {
                        v.push(Platform::new(Point3::new(x as f32, 0.0, z as f32)));
                    }
                }
                v
            },
            skybox: Skybox::new(),
            cube: Cube::new(),
        }
    }
}

struct Player {
    camera: Camera,
    height: f32,
    transform: Matrix4<f32>,

    moving_towards: Vector3<f32>,
    rotating_towards: f32,
}

impl Player {
    pub fn new(position: Point3<f32>) -> Self {
        Player {
            camera: Camera::perspective(),
            height: 1.7,
            moving_towards: Vector3::new(0.0, 0.0, 0.0),
            rotating_towards: 0.0,
            transform: Matrix4::new_translation(&position.coords),
        }
    }
}

struct Platform {
    position: Point3<f32>,
    target_height: f32,
    color: PlatformColor,
}

#[derive(Clone, Copy)]
enum PlatformColor {
    Red(bool),
    Green(bool),
    Blue(bool),
}

impl Into<Vector4<f32>> for PlatformColor {
    fn into(self) -> Vector4<f32> {
        use PlatformColor::*;
        match self {
            Red(bright) => Vector4::new(if bright { 0.8 } else { 0.3 }, 0.1, 0.1, 1.0),
            Green(bright) => Vector4::new(0.1, if bright { 0.8 } else { 0.3 }, 0.1, 1.0),
            Blue(bright) => Vector4::new(0.1, 0.1, if bright { 0.8 } else { 0.3 }, 1.0),
        }
    }
}


impl Platform {
    pub fn new(position: Point3<f32>) -> Self {
        Platform {
            position,
            target_height: 0.0,
            color: PlatformColor::Green(false),
        }
    }
}

impl IntoMesh for Platform {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords)
    }
    fn mesh(&self) -> Mesh {
        let v1 = Vertex::at(Point3::new(-0.5, 0.0, -0.5));
        let v2 = Vertex::at(Point3::new(-0.5, 0.0,  0.5));
        let v3 = Vertex::at(Point3::new( 0.5, 0.0,  0.5));
        let v4 = Vertex::at(Point3::new( 0.5, 0.0, -0.5));

        Mesh {
            triangles: vec![
                Triangle::new(v1, v2, v3).color(self.color.into()),
                Triangle::new(v1, v3, v4).color(self.color.into()),
            ],
        }
    }
}

struct Cube {
    position: Point3<f32>,
    rotation: f32,
    color: Vector4<f32>,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            position: Point3::new(2.0, 1.9, 2.0),
            rotation: 0.0,
            color: Vector4::new(0.7, 0.7, 0.9, 1.0),
        }
    }
}

impl IntoMesh for Cube {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords)
        * Matrix4::from_euler_angles(self.rotation * 0.2, self.rotation, self.rotation * 0.01)
    }
    fn mesh(&self) -> Mesh {
        let v1 = Vertex::at(Point3::new(-1.0,  1.0, -1.0));
        let v2 = Vertex::at(Point3::new(-1.0,  1.0,  1.0));
        let v3 = Vertex::at(Point3::new( 1.0,  1.0,  1.0));
        let v4 = Vertex::at(Point3::new( 1.0,  1.0, -1.0));
        let v5 = Vertex::at(Point3::new(-1.0, -1.0, -1.0));
        let v6 = Vertex::at(Point3::new(-1.0, -1.0,  1.0));
        let v7 = Vertex::at(Point3::new( 1.0, -1.0,  1.0));
        let v8 = Vertex::at(Point3::new( 1.0, -1.0, -1.0));

        Mesh {
            triangles: vec![
                Triangle::new(v1, v2, v3),
                Triangle::new(v1, v3, v4),
                Triangle::new(v5, v1, v4),
                Triangle::new(v5, v4, v8),
                Triangle::new(v8, v4, v3),
                Triangle::new(v8, v3, v7),
                Triangle::new(v7, v3, v2),
                Triangle::new(v7, v2, v6),
                Triangle::new(v6, v2, v1),
                Triangle::new(v6, v1, v5),
                Triangle::new(v5, v8, v7),
                Triangle::new(v5, v7, v6),
            ],
        }.color(self.color)
    }
}

struct Skybox {}

impl Skybox {
    pub fn new() -> Self {
        Self {}
    }
}

impl Data for Scene {}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        use PlatformColor::*;
        if !self.paused {
            self.clock += tick;
        }
        std::thread::sleep_ms(20);

        if self.clock.delta() >= self.next_change {
            self.next_change = Duration::from_secs(1);
            {
                let normal = Normal::new(0.2, 0.2);
                let mut rng = rand::thread_rng();
                for platform in self.floor.iter_mut() {
                    let ph = normal.ind_sample(&mut rng) as f32;
                    platform.target_height = ph;
                    if ph >= 0.3 {
                        platform.color = Red(ph > 0.36);
                    } else if ph >= 0.15 {
                        platform.color = Green(ph > 0.2);
                    } else {
                        platform.color = Blue(ph > 0.05);
                    }
                }
            }
        } else {
            self.next_change -= self.clock.delta();
        }

        for platform in self.floor.iter_mut() {
            let y = platform.position.y;
            let d = (platform.target_height - y) * self.clock.delta_as_sec() * 10.0;
            platform.position.y += d;
        }

        self.cube.rotation += 0.2 * self.clock.delta_as_sec();
        
        const player_speed: f32 = 2.0;
        let translation = self.player.moving_towards * self.clock.delta_as_sec() * player_speed;
        self.player.transform.append_translation_mut(&translation);
        self.player.transform = Matrix4::from_euler_angles(0.0, self.player.rotating_towards * self.clock.delta_as_sec() * player_speed * 0.3, 0.0) * self.player.transform;
    }
}

impl GetMeshes for Scene {
    fn mesh_iter(&self) -> std::vec::IntoIter<&IntoMesh> {
        let mut v: Vec<&IntoMesh> = Vec::new();
        for platform in self.floor.iter() {
            v.push(platform);
        }
        v.push(&self.cube);
        v.into_iter()
    }
}

impl GetCamera for Scene {
    fn get_camera(&self) -> (Matrix4<f32>, &Camera) {
        let camera_v_offset = Vector3::y() * self.player.height;
        (self.player.transform.append_translation(&camera_v_offset), &self.player.camera)
    }
}

impl OnKeyboard for Scene {
    fn handle(&mut self, event: KeyboardEvent) {
        let mt = &mut self.player.moving_towards;
        let rt = &mut self.player.rotating_towards;
        match event {
            KeyboardEvent::Pressed(key, _) => {
                match key {
                    Key::A => mt.x = 1.0, //TODO: Invert this axis
                    Key::S => mt.z = -1.0,
                    Key::D => mt.x = -1.0,
                    Key::W => mt.z = 1.0,
                    Key::Q => *rt = -1.0,
                    Key::E => *rt = 1.0,
                };
            }
            KeyboardEvent::Released(key, _) => {
                match key {
                    Key::A | Key::D => mt.x = 0.0,
                    Key::S | Key::W => mt.z = 0.0,
                    Key::Q | Key::E => *rt = 0.0,
                    _ => (),
                };
            }
        }
    }
}

