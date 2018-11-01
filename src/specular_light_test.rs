extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;
extern crate rand;

use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_blocks::camera::{Camera, CameraUpdater, GetCamera};
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_blocks::input::{Key, KeyboardEvent, OnKeyboard, KeyboardUpdater, MouseEvent, OnMouse, MouseUpdater};
use mursten_blocks::mesh_renderer::{GetMeshes, IntoMesh, MeshRenderer};
use mursten_blocks::light::{Light, GetLights, LightUpdater};
use mursten_vulkan_backend::VulkanBackend;

use nalgebra::*;


pub fn main() {
    let backend = VulkanBackend::new();
    let scene = Scene::new();
    mursten::Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(CameraUpdater::new())
        .add_updater(KeyboardUpdater::new())
        .add_updater(MouseUpdater::new())
        .add_updater(LightUpdater::new())
        .add_renderer(MeshRenderer::new())
        .run(scene);
}

struct Scene {
    clock: Clock,
    player: Player,
    floor: Platform,
    cube: Cube,
    gizmo: AxisGizmo,
}

struct Player {
    camera: Camera,
    height: f32,
    position: Point3<f32>,
    direction: Vector3<f32>,

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
            position,
            direction: Vector3::z(),
        }
    }
}

struct Platform {
    position: Point3<f32>,
    scale: f32,
}

impl Platform {
    pub fn new(position: Point3<f32>) -> Self {
        Platform {
            position,
            scale: 10.0,
        }
    }
}

impl IntoMesh for Platform {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * Matrix4::new_scaling(self.scale)
    }
    fn mesh(&self) -> Mesh {
        let v1 = Vertex::at(Point3::new(-0.5, 0.0, -0.5));
        let v2 = Vertex::at(Point3::new(-0.5, 0.0,  0.5));
        let v3 = Vertex::at(Point3::new( 0.5, 0.0,  0.5));
        let v4 = Vertex::at(Point3::new( 0.5, 0.0, -0.5));

        Mesh {
            triangles: vec![
                Triangle::new(v1, v3, v2),
                Triangle::new(v1, v4, v3),
            ],
        }.color(Vector4::new(0.9, 0.9, 0.9, 1.0))
    }
}

struct Cube {
    position: Point3<f32>,
    scale: f32,
    color: Vector4<f32>,
}

impl Cube {
    pub fn new(position: Point3<f32>, scale: f32) -> Self {
        Self {
            position,
            scale,
            color: Vector4::new(0.8, 0.3, 0.3, 1.0),
        }
    }
}

impl IntoMesh for Cube {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * Matrix4::new_scaling(self.scale)
    }
    fn mesh(&self) -> Mesh {
        let v1 = Vertex::at(Point3::new(-0.5,  0.5, -0.5));
        let v2 = Vertex::at(Point3::new(-0.5,  0.5,  0.5));
        let v3 = Vertex::at(Point3::new( 0.5,  0.5,  0.5));
        let v4 = Vertex::at(Point3::new( 0.5,  0.5, -0.5));
        let v5 = Vertex::at(Point3::new(-0.5, -0.5, -0.5));
        let v6 = Vertex::at(Point3::new(-0.5, -0.5,  0.5));
        let v7 = Vertex::at(Point3::new( 0.5, -0.5,  0.5));
        let v8 = Vertex::at(Point3::new( 0.5, -0.5, -0.5));

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

struct AxisGizmo {
    position: Point3<f32>,
}

impl AxisGizmo {
    pub fn new(position: Point3<f32>) -> Self {
        Self { position }
    }
}

impl IntoMesh for AxisGizmo {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords)
    }
    fn mesh(&self) -> Mesh {
        use std::f32::consts::PI;

        let v1 = Vertex::at(Point3::new(-0.1,  0.1, -0.1));
        let v2 = Vertex::at(Point3::new(-0.1,  0.1,  0.1));
        let v3 = Vertex::at(Point3::new( 0.1,  0.1,  0.1));
        let v4 = Vertex::at(Point3::new( 0.1,  0.1, -0.1));
        let v5 = Vertex::at(Point3::new(-0.1, -0.1, -0.1));
        let v6 = Vertex::at(Point3::new(-0.1, -0.1,  0.1));
        let v7 = Vertex::at(Point3::new( 0.1, -0.1,  0.1));
        let v8 = Vertex::at(Point3::new( 0.1, -0.1, -0.1));
        let mut triangles = vec![
            Triangle::new(v1, v3, v2),
            Triangle::new(v1, v4, v3),
            Triangle::new(v5, v4, v1),
            Triangle::new(v5, v8, v4),
            Triangle::new(v8, v3, v4),
            Triangle::new(v8, v7, v3),
            Triangle::new(v7, v2, v3),
            Triangle::new(v7, v6, v2),
            Triangle::new(v6, v1, v2),
            Triangle::new(v6, v5, v1),
            Triangle::new(v5, v7, v8),
            Triangle::new(v5, v6, v7),
        ];

        let x = Point3::new(1.0, 0.0, 0.0);
        let y = Point3::new(0.0, 1.0, 0.0);
        let z = Point3::new(0.0, 0.0, 1.0);

        let arrow_len: f32 = 0.2;
        let arrow_radius: f32 = 0.1;
        let divisions = 14;
        for i in 0..divisions {
            let slice_angle = 2.0 * PI / divisions as f32;
            let angle = i as f32 * slice_angle;
            triangles.push(Triangle::new(
                (   z
                ).into(),
                (   z
                    - arrow_len * Vector3::z()
                    + Rotation3::from_axis_angle(&Vector3::z_axis(), angle + slice_angle) * Vector3::y() * arrow_radius
                ).into(),
                (   z
                    - arrow_len * Vector3::z()
                    + Rotation3::from_axis_angle(&Vector3::z_axis(), angle) * Vector3::y() * arrow_radius
                ).into(),
            ).color(Vector4::new(0.0, 0.0, 1.0, 1.0)));
            triangles.push(Triangle::new(
                (   x
                ).into(),
                (   x
                    - arrow_len * Vector3::x()
                    + Rotation3::from_axis_angle(&Vector3::x_axis(), angle + slice_angle) * Vector3::z() * arrow_radius
                ).into(),
                (   x
                    - arrow_len * Vector3::x()
                    + Rotation3::from_axis_angle(&Vector3::x_axis(), angle) * Vector3::z() * arrow_radius
                ).into(),
            ).color(Vector4::new(1.0, 0.0, 0.0, 1.0)));
            triangles.push(Triangle::new(
                (   y
                ).into(),
                (   y
                    - arrow_len * Vector3::y()
                    + Rotation3::from_axis_angle(&Vector3::y_axis(), angle + slice_angle) * Vector3::x() * arrow_radius
                ).into(),
                (   y
                    - arrow_len * Vector3::y()
                    + Rotation3::from_axis_angle(&Vector3::y_axis(), angle) * Vector3::x() * arrow_radius
                ).into(),
            ).color(Vector4::new(0.0, 1.0, 0.0, 1.0)));
        }
        
        Mesh {
            triangles,
        }
    }
}


struct Skybox {}

impl Skybox {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            clock: Clock::new(),
            player: Player::new(Point3::new(1.0, 0.0, -2.0)),
            floor: Platform::new(Point3::origin()),
            cube: Cube::new(Point3::new(0.0, 0.0, 0.0), 0.3),
            gizmo: AxisGizmo::new(Point3::new(0.0, 0.2, 0.0)),
        }
    }
}

impl mursten::Data for Scene {}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        self.clock += tick;
        std::thread::sleep_ms(20);

        self.cube.position = Point3::origin() + Rotation3::from_axis_angle(&Vector3::y_axis(), self.clock.time_in_sec()) * Vector3::new(2.0, 3.0, 0.0);

        const player_speed: f32 = 2.0;
        let translation = self.player.moving_towards * self.clock.delta_as_sec() * player_speed ;
        self.player.position += Rotation3::rotation_between(&Vector3::z(), &self.player.direction).unwrap() * translation;
        let rotation_angle = self.player.rotating_towards * self.clock.delta_as_sec() * player_speed * 0.3;
        self.player.direction = Rotation3::from_axis_angle(&Vector3::y_axis(), rotation_angle) * self.player.direction;
    }
}

impl GetMeshes for Scene {
    fn mesh_iter(&self) -> std::vec::IntoIter<&IntoMesh> {
        let mut v: Vec<&IntoMesh> = Vec::new();
        v.push(&self.floor);
        v.push(&self.cube);
        v.push(&self.gizmo);
        v.into_iter()
    }
}

impl GetCamera for Scene {
    fn get_camera(&self) -> (Matrix4<f32>, &Camera) {
        let camera_v_offset = Vector3::y() * self.player.height;
        let eye = self.player.position + camera_v_offset;
        let target = eye + self.player.direction;
        let view = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                0.0, 1.0, 0.0, 0.0,
                                0.0, 0.0,-1.0, 0.0,
                                0.0, 0.0, 0.0, 1.0) * Matrix4::look_at_lh(&eye, &target, &Vector3::y());
        (view, &self.player.camera)
    }
}

impl GetLights for Scene {
    fn get_light(&self) -> Light {
        let p = Point3::origin() + Rotation3::from_axis_angle(&Vector3::y_axis(), self.clock.time_in_sec()) * Vector3::new(2.0, 3.0, 0.0);
        Light::new(p, Vector3::new(1.0, 1.0, 1.0), 0.0)
    }
}

impl OnKeyboard for Scene {
    fn handle(&mut self, event: KeyboardEvent) {
        let mt = &mut self.player.moving_towards;
        let rt = &mut self.player.rotating_towards;
        match event {
            KeyboardEvent::Pressed(key, _) => {
                match key {
                    Key::A => mt.x = -1.0,
                    Key::S => mt.z = -1.0,
                    Key::D => mt.x = 1.0,
                    Key::W => mt.z = 1.0,
                    Key::Q => *rt = -1.0,
                    Key::E => *rt = 1.0,
                    _ => (),
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

impl OnMouse for Scene {
    fn handle(&mut self, event: MouseEvent) {
        match event {
            MouseEvent::Wheel(displacement) => {
                let r = Rotation3::rotation_between(&Vector3::z(), &Vector3::new(displacement.x, -displacement.y, 100.0)).unwrap();
                self.player.direction = r * self.player.direction;
            },
            _ => (),
        }
    }
}
