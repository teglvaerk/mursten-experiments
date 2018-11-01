extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;

use mursten::{Application, Backend, Data};

use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_blocks::camera::{Camera, CameraUpdater, GetCamera};
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_blocks::input::{Key, KeyboardEvent, OnKeyboard, KeyboardUpdater, MouseEvent, OnMouse, MouseUpdater, MouseButton};
use mursten_blocks::mesh_renderer::{GetMeshes, IntoMesh, MeshRenderer};
use mursten_blocks::light::{Light, GetLights, LightUpdater};

use mursten_vulkan_backend::VulkanBackend;

use nalgebra::*;
use std::f32::consts::PI;
use std::process::{Command, Stdio};


pub fn main() {
    let backend = VulkanBackend::new();
    let scene = Scene::new();
    Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(CameraUpdater::new())
        .add_updater(KeyboardUpdater::new())
        .add_updater(MouseUpdater::new())
        .add_renderer(MeshRenderer::new())
        .run(scene);
}

struct Scene { 
    clock: Clock,
    player: Player,
    floor: Platform,
    walls: Vec<Platform>,
    roof: Platform,
    desk: Desk,
    lamp: Lamp,
    painting: Painting,
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
    rotation: Rotation3<f32>,
    scale: Vector3<f32>,
    color: Vector3<f32>,
}

impl Platform {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            color: Vector3::new(1.0, 1.0, 1.0),
        }
    }
    pub fn rotated(self, rotation: Rotation3<f32>) -> Self {
        Self { rotation, ..self }
    }
    pub fn scaled(self, scale: Vector3<f32>) -> Self {
        Self { scale, ..self }
    }
    pub fn colored(self, color: Vector3<f32>) -> Self {
        Self { color, ..self }
    }
}

impl IntoMesh for Platform {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * self.rotation.to_homogeneous() * Matrix4::new_nonuniform_scaling(&self.scale)
    }
    fn mesh(&self) -> Mesh {
        let v1 = Vertex::at(Point3::new(-0.50, 0.0, -0.50));
        let v2 = Vertex::at(Point3::new(-0.50, 0.0,  0.50));
        let v3 = Vertex::at(Point3::new( 0.50, 0.0,  0.50));
        let v4 = Vertex::at(Point3::new( 0.50, 0.0, -0.50));

        Mesh {
            triangles: vec![
                Triangle::new(v1, v3, v2),
                Triangle::new(v1, v4, v3),
            ],
        }.color(Vector4::new(self.color.x, self.color.y, self.color.z, 1.0))
    }
}

struct Desk {
    position: Point3<f32>,
    rotation: Rotation3<f32>,
}

impl Desk {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0),
        }
    }
    pub fn rotated(self, rotation: Rotation3<f32>) -> Self {
        Self { rotation, ..self }
    }
}

impl IntoMesh for Desk {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * self.rotation.to_homogeneous()
    }
    fn mesh(&self) -> Mesh {
        let mut triangles = Vec::new();

        let desk_leg = |position, rotation| {
            let v1 = Vertex::at(position + rotation * Vector3::new(-0.04,  0.99, -0.04));
            let v2 = Vertex::at(position + rotation * Vector3::new(-0.04,  0.99,  0.02));
            let v3 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.99,  0.02));
            let v4 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.99, -0.04));
            let v5 = Vertex::at(position + rotation * Vector3::new(-0.02,  0.0,  -0.02));
            let v6 = Vertex::at(position + rotation * Vector3::new(-0.02,  0.0,   0.02));
            let v7 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.0,   0.02));
            let v8 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.0,  -0.02));

            vec![
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
            ]
        };

        let desk_table = || {
            let v1 = Vertex::at(Point3::new(-0.27,  1.0,  -0.52));
            let v2 = Vertex::at(Point3::new(-0.27,  1.0,   0.52));
            let v3 = Vertex::at(Point3::new( 0.27,  1.0,   0.52));
            let v4 = Vertex::at(Point3::new( 0.27,  1.0,  -0.52));
            let v5 = Vertex::at(Point3::new(-0.27,  0.96, -0.52));
            let v6 = Vertex::at(Point3::new(-0.27,  0.96,  0.52));
            let v7 = Vertex::at(Point3::new( 0.27,  0.96,  0.52));
            let v8 = Vertex::at(Point3::new( 0.27,  0.96, -0.52));

            vec![
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
            ]
        };

        triangles.append(&mut desk_leg(Point3::new( 0.23, 0.0,  0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0)));
        triangles.append(&mut desk_leg(Point3::new(-0.23, 0.0,  0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), -PI/2.0)));
        triangles.append(&mut desk_leg(Point3::new(-0.23, 0.0, -0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), -PI)));
        triangles.append(&mut desk_leg(Point3::new( 0.23, 0.0, -0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), 3.0 * -PI/2.0)));
        triangles.append(&mut desk_table());

        Mesh { triangles, }.color(Palette::ZinnwalditeBrown.into())
    }
}

struct Lamp {
    position: Point3<f32>,
    offset: Vector3<f32>,
    rotation: Rotation3<f32>,
    is_on: bool,
    is_target: bool,
    glow: Vector4<f32>,
}

impl Lamp {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            offset: Vector3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0),
            is_on: true,
            is_target: false,
            glow: Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn rotated(self, rotation: Rotation3<f32>) -> Self {
        Self { rotation, ..self }
    }
}

impl IntoMesh for Lamp {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&(self.position.coords + self.offset)) * self.rotation.to_homogeneous()
    }
    fn mesh(&self) -> Mesh {
        let mut triangles = Vec::new();

        let cylindre = |p1: Point3<f32>, r1: f32, p2: Point3<f32>, r2: f32, color: Vector4<f32>| {
            let mut ts = Vec::new();
            let divisions = 18;
            let d = p2 - p1;
            let n = Vector3::z().cross(&d).normalize();
            for i in 0..divisions {
                let rot = Rotation3::from_axis_angle(&Unit::new_normalize(d), 2.0 * PI / divisions as f32);
                let rot1 = rot.powf(i as f32);
                let rot2 = rot.powf(i as f32 + 1.0);
                ts.push(Triangle {
                    v1: (p1 + rot1 * n * r1).into(),
                    v2: (p2 + rot1 * n * r2).into(),
                    v3: (p1 + rot2 * n * r1).into(),
                }.color(color));
                ts.push(Triangle {
                    v1: (p2 + rot2 * n * r2).into(),
                    v2: (p1 + rot2 * n * r1).into(),
                    v3: (p2 + rot1 * n * r2).into(),
                }.color(color));
            }
            ts
        };
        let color_a: Vector4<f32> = Palette::LapisLazuli.into();
        let color_b: Vector4<f32> = Palette::PewterBlue.into();
        let color_a = color_a + self.glow;
        let color_b = color_b + self.glow;

        triangles.append(&mut cylindre(Point3::new(0.0, 0.25, 0.0), 0.06, Point3::new(0.0, 0.27, 0.0), 0.06, color_a));
        triangles.append(&mut cylindre(Point3::new(0.0, 0.27, 0.0), 0.06, Point3::new(0.0, 0.32, 0.0), 0.03, color_a));
        triangles.append(&mut cylindre(Point3::new(0.0, 0.32, 0.0), 0.03, Point3::new(0.0, 0.35, 0.0), 0.03, color_a));
        triangles.append(&mut cylindre(Point3::new(0.0, 0.35, 0.0), 0.03, Point3::new(0.0, 0.351, 0.0), 0.0, color_a));

        triangles.append(&mut cylindre(Point3::new( 0.02, 0.34, 0.01), 0.005, Point3::new( 0.02, 0.15, 0.25), 0.005, color_b));
        triangles.append(&mut cylindre(Point3::new(-0.02, 0.34, 0.01), 0.005, Point3::new(-0.02, 0.15, 0.25), 0.005, color_b));

        triangles.append(&mut cylindre(Point3::new( 0.02, 0.15, 0.25), 0.005, Point3::new( 0.02, 0.01, 0.20), 0.005, color_b));
        triangles.append(&mut cylindre(Point3::new(-0.02, 0.15, 0.25), 0.005, Point3::new(-0.02, 0.01, 0.20), 0.005, color_b));

        triangles.append(&mut cylindre(Point3::new(0.0, 0.0, 0.20), 0.08, Point3::new(0.0,  0.02,  0.20), 0.08, color_a));
        triangles.append(&mut cylindre(Point3::new(0.0, 0.02, 0.20), 0.08, Point3::new(0.0, 0.04,  0.20), 0.03, color_a));
        triangles.append(&mut cylindre(Point3::new(0.0, 0.04, 0.20), 0.03, Point3::new(0.0, 0.041, 0.20), 0.00, color_a));

        if self.is_on {
            triangles.append(&mut cylindre(Point3::new(0.0, 0.32, 0.0), 0.03, Point3::new(0.0, -self.offset.y, 0.0), 0.1, Vector4::new(1.0, 1.0, 1.0, 0.02)));
        }

        Mesh { triangles, }
    }
}



struct Painting {
    position: Point3<f32>,
    rotation: Rotation3<f32>,
    is_target: bool,
    glow: Vector4<f32>,
}

impl Painting {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0),
            is_target: false,
            glow: Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn rotated(self, rotation: Rotation3<f32>) -> Self {
        Self { rotation, ..self }
    }
}

impl IntoMesh for Painting {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * self.rotation.to_homogeneous()
    }
    fn mesh(&self) -> Mesh {
        let mut triangles = Vec::new();

        let color: Vector4<f32> = Palette::ZinnwalditeBrown.into();
        let color = color + self.glow;

        let frame_segment = |pos: Point3<f32>, len: f32, rot: Rotation3<f32>| {

            let v1 = Vertex::at(pos + rot * Vector3::new(-len/2.0 + 0.02,  0.02, -0.013)).color(color);
            let v2 = Vertex::at(pos + rot * Vector3::new(-len/2.0 + 0.02,  0.02,  0.013)).color(color);
            let v3 = Vertex::at(pos + rot * Vector3::new( len/2.0 - 0.02,  0.02,  0.013)).color(color);
            let v4 = Vertex::at(pos + rot * Vector3::new( len/2.0 - 0.02,  0.02, -0.013)).color(color);
            let v5 = Vertex::at(pos + rot * Vector3::new(-len/2.0 - 0.02, -0.02, -0.02)).color(color);
            let v6 = Vertex::at(pos + rot * Vector3::new(-len/2.0 - 0.02, -0.02,  0.02)).color(color);
            let v7 = Vertex::at(pos + rot * Vector3::new( len/2.0 + 0.02, -0.02,  0.02)).color(color);
            let v8 = Vertex::at(pos + rot * Vector3::new( len/2.0 + 0.02, -0.02, -0.02)).color(color);

            vec![
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
            ]
        };

        triangles.append(&mut frame_segment(Point3::new( 0.0,  1.5, 0.0), 1.0, Rotation3::from_axis_angle(&Vector3::z_axis(), 0.0)));
        triangles.append(&mut frame_segment(Point3::new( 0.0,  2.0, 0.0), 1.0, Rotation3::from_axis_angle(&Vector3::z_axis(), PI)));
        triangles.append(&mut frame_segment(Point3::new( 0.5, 1.75, 0.0), 0.5, Rotation3::from_axis_angle(&Vector3::z_axis(), PI/2.0)));
        triangles.append(&mut frame_segment(Point3::new(-0.5, 1.75, 0.0), 0.5, Rotation3::from_axis_angle(&Vector3::z_axis(), 3.0 * PI/2.0)));

        triangles.push(Triangle {
            v1: Vertex::at(Point3::new( 0.5,  1.5, 0.0)),
            v2: Vertex::at(Point3::new(-0.5,  1.5, 0.0)),
            v3: Vertex::at(Point3::new(-0.5,  2.0, 0.0)),
        });
        triangles.push(Triangle {
            v1: Vertex::at(Point3::new(-0.5,  2.0, 0.0)),
            v2: Vertex::at(Point3::new( 0.5,  2.0, 0.0)),
            v3: Vertex::at(Point3::new( 0.5,  1.5, 0.0)),
        });

        Mesh { triangles, }
    }
}

impl Scene {
    pub fn new() -> Self {
        let floor_color = Palette::PewterBlue.into();
        let wall_color = Palette::AntiqueRuby.into();
        let roof_color = Palette::RaisinBlack.into();
        Scene {
            clock: Clock::new(),
            player: Player::new(Point3::new(1.0, 0.0, -2.0)),
            floor: Platform::new(Point3::origin())
                .scaled(Vector3::new(6.0, 1.0, 6.0))
                .colored(floor_color),
            walls: vec![
                Platform::new(Point3::new(-3.0, 1.5, 0.0))
                    .scaled(Vector3::new(3.0, 1.0, 6.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::z_axis(), -PI/2.0)),
                Platform::new(Point3::new(0.0, 1.5, 3.0))
                    .scaled(Vector3::new(6.0, 1.0, 3.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::x_axis(), PI/2.0)),
                Platform::new(Point3::new(3.0, 1.5, 0.0))
                    .scaled(Vector3::new(3.0, 1.0, 6.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::z_axis(), -PI/2.0)),
                Platform::new(Point3::new(0.0, 1.5, -3.0))
                    .scaled(Vector3::new(6.0, 1.0, 3.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::x_axis(), -PI/2.0)),
            ],
            roof: Platform::new(Point3::origin() + Vector3::y() * 3.0)
                .scaled(Vector3::new(6.0, 1.0, 6.0))
                .colored(roof_color),
            desk: Desk::new(Point3::new(-2.0, 0.0, 0.0)),
            lamp: Lamp::new(Point3::new(-2.0, 1.0, 0.0)),
            painting: Painting::new(Point3::new(0.0, 0.0, 2.98)),
        }
    }
}

impl mursten::Data for Scene {}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        self.clock += tick;
        std::thread::sleep_ms(20);

        const player_speed: f32 = 2.0;
        let translation = self.player.moving_towards * self.clock.delta_as_sec() * player_speed;
        let mut floor_direction = self.player.direction;
        floor_direction.y = 0.0;

        let mut new_position = self.player.position + Rotation3::rotation_between(&Vector3::z(), &floor_direction).unwrap() * translation;

        let in_room = |position: Point3<f32>| {
            position.x > -2.88 &&
            position.x <  2.88 &&
            position.z > -2.88 &&
            position.z <  2.88
        };
        let in_desk = |position: Point3<f32>| {
            position.x > -2.45 &&
            position.x < -1.55 &&
            position.z > -0.5 &&
            position.z <  0.5
        };

        if in_room(new_position) && !in_desk(new_position) {
            self.player.position = new_position;
        }

        //self.player.position.y = 1.75 + (self.clock.time_in_sec() * 0.3).sin();
        //eprintln!("{}", self.player.position.y);

        let rotation_angle = self.player.rotating_towards * self.clock.delta_as_sec() * player_speed * 0.3;
        self.player.direction = Rotation3::from_axis_angle(&Vector3::y_axis(), rotation_angle) * self.player.direction;

        let head_pos = self.player.position + Vector3::y() * 1.7;

        if ((head_pos + self.player.direction.normalize() * 1.0) - self.lamp.position).norm() < 0.4
            || ((head_pos + self.player.direction.normalize() * 0.7) - self.lamp.position).norm() < 0.4
            || ((head_pos + self.player.direction.normalize() * 0.4) - self.lamp.position).norm() < 0.3 {
            self.lamp.glow = Vector4::new(0.2, 0.2, 0.2, 0.0) + (self.clock.time_in_sec()*5.0).sin() * Vector4::new(0.2, 0.2, 0.2, 0.0);
            self.lamp.is_target = true;
        } else {
            self.lamp.glow = Vector4::new(0.0, 0.0, 0.0, 0.0);
            self.lamp.is_target = false;
        }

        let painting_pos = self.painting.position + Vector3::y() * 1.4;

        if ((head_pos + self.player.direction.normalize() * 1.0) - painting_pos).norm() < 0.4
            || ((head_pos + self.player.direction.normalize() * 0.7) - painting_pos).norm() < 0.4
            || ((head_pos + self.player.direction.normalize() * 0.4) - painting_pos).norm() < 0.3 {
            self.painting.glow = Vector4::new(0.2, 0.2, 0.2, 0.0) + (self.clock.time_in_sec()*5.0).sin() * Vector4::new(0.2, 0.2, 0.2, 0.0);
            self.painting.is_target = true;
        } else {
            self.painting.glow = Vector4::new(0.0, 0.0, 0.0, 0.0);
            self.painting.is_target = false;
        }

        if !self.lamp.is_on {
            self.lamp.offset = self.lamp.offset * 0.8 + Rotation3::from_axis_angle(&Vector3::y_axis(), self.clock.time_in_sec()) * Vector3::new(0.05, 0.1 + (self.clock.time_in_sec() * 3.7).sin() * 0.05, 0.0) * 0.2;
        } else {
            self.lamp.offset = self.lamp.offset * 0.5;
        }
    }
}

impl GetMeshes for Scene {
    fn mesh_iter(&self) -> std::vec::IntoIter<&IntoMesh> {
        let mut v: Vec<&IntoMesh> = Vec::new();
        v.push(&self.floor);
        v.push(&self.roof);
        for wall in self.walls.iter() {
            v.push(wall);
        }
        v.push(&self.desk);
        v.push(&self.lamp);
        v.push(&self.painting);
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
        Light::new(p, Vector3::new(0.0, 2.8, 0.0), 0.3)
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
                    Key::Q | Key::J => *rt = -1.0,
                    Key::E | Key::K => *rt = 1.0,
                    Key::F => self.lamp.is_on = !self.lamp.is_on,
                };
            }
            KeyboardEvent::Released(key, _) => {
                match key {
                    Key::A | Key::D => mt.x = 0.0,
                    Key::S | Key::W => mt.z = 0.0,
                    Key::Q | Key::E | Key::J | Key::K => *rt = 0.0,
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
            MouseEvent::Movement(delta) => {
                let x =  delta.x.signum() * delta.x.abs().sqrt() / 100.0;
                let y = -delta.y.signum() * delta.y.abs().sqrt() / 100.0;
                let left_right = Rotation3::from_axis_angle(&Vector3::y_axis(), x);
                let axis = Unit::new_normalize(self.player.direction.cross(&Vector3::y()));
                let up_down = Rotation3::from_axis_angle(&axis, y);
                let new_direction = left_right * up_down * self.player.direction;
                let y_ang = new_direction.angle(&Vector3::y_axis());
                let deadangle = 0.5;
                if (y_ang > deadangle && y_ang < PI - deadangle) || (y_ang > PI + deadangle && y_ang < 2.0*PI - deadangle) {
                    self.player.direction = new_direction;
                }
            },
            MouseEvent::Pressed(MouseButton::Left, _) if self.lamp.is_target => {
                self.lamp.is_on = !self.lamp.is_on;
            },
            MouseEvent::Pressed(MouseButton::Left, _) if self.painting.is_target => {

                if cfg!(windows) {
                    eprintln!("Sorpresa!");
                    Command::new("cmd")
                            .arg("/c")
                            .arg("start")
                            .arg("https://www.youtube.com/watch?v=oFlG7KN6OqY")
                            .stderr(Stdio::null())
                            .spawn()
                            .expect("Surprise failed to start");
                } else if cfg!(target_os = "macos") {
                    eprintln!("Sorpresa!");
                    Command::new("open")
                            .arg("https://www.youtube.com/watch?v=oFlG7KN6OqY")
                            .stderr(Stdio::null())
                            .spawn()
                            .expect("Surprise failed to start");
                } else {
                    eprintln!("Surprises are not allowed for your os.");
                } 
            },
            _ => (),
        }
    }
}

enum Palette {
    RaisinBlack,
    AntiqueRuby,
    ZinnwalditeBrown,
    PewterBlue,
    LapisLazuli,
}

impl Into<Vector3<f32>> for Palette {
    fn into(self) -> Vector3<f32> {
        match self {
            Palette::RaisinBlack      => Vector3::new(0x26 as f32, 0x26 as f32, 0x26 as f32) / 256.0,
            Palette::AntiqueRuby      => Vector3::new(0x88 as f32, 0x29 as f32, 0x2F as f32) / 256.0,
            Palette::ZinnwalditeBrown => Vector3::new(0x2E as f32, 0x1E as f32, 0x0F as f32) / 256.0,
            Palette::PewterBlue       => Vector3::new(0x90 as f32, 0xA9 as f32, 0xB7 as f32) / 256.0,
            Palette::LapisLazuli      => Vector3::new(0x25 as f32, 0x5C as f32, 0x99 as f32) / 256.0,
        }
    }
}

impl Into<Vector4<f32>> for Palette {
    fn into(self) -> Vector4<f32> {
        let c: Vector3<f32> = self.into();
        Vector4::new(c.x, c.y, c.z, 1.0)
    }
}
