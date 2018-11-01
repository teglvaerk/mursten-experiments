extern crate midir;
extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;
extern crate rand;

use mursten::{Application, Data, Renderer};
use mursten_blocks::camera::{Camera, CameraUpdater, GetCamera};
use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_blocks::midi::{MidiMessage, MidiUpdater, OnMidiMessage};
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_vulkan_backend::VulkanBackend;

use nalgebra::*;

pub fn main() {
    let backend = VulkanBackend::new();
    let scene = Scene::default();
    Application::new(backend)
        .add_updater(CameraUpdater::new())
        .add_updater(ClockUpdater::new())
        .add_updater(MidiUpdater::prompt())
        .add_renderer(Visual::new())
        .run(scene);
}

struct Scene {
    clock: Clock,
    camera: Camera,
    paused: bool,
    keyboard: [u8; 128],
}

impl Scene {
    pub fn new() -> Self {
        Scene::default()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            clock: Clock::new(),
            paused: false,
            keyboard: [0; 128],
            camera: Camera::perspective(), 
        }
    }
}

impl Data for Scene {}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        if !self.paused {
            self.clock += tick;
        }
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(20));
    }
}

impl OnMidiMessage for Scene {
    fn on_midi_message(&mut self, msg: MidiMessage) {
        match msg {
            MidiMessage::NoteOn(key, vel) => {
                self.keyboard[key as usize] = vel;
            }
            MidiMessage::NoteOff(key, _) => {
                self.keyboard[key as usize] = 0;
            }
            _ => {}
        }
    }
}

impl GetCamera for Scene {
    fn get_camera(&self) -> (Matrix4<f32>, &Camera) {
        let eye = Point3::origin();
        let target = eye + Vector3::z();
        let view = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                0.0, 1.0, 0.0, 0.0,
                                0.0, 0.0,-1.0, 0.0,
                                0.0, 0.0, 0.0, 1.0) * Matrix4::look_at_lh(&eye, &target, &Vector3::y());
        (view, &self.camera)
    }
}

struct Visual {
    last_keyboard: Vec<u8>,
}

impl Visual {
    pub fn new() -> Self {
        Visual {
            last_keyboard: (0..36).collect(),
        }
    }
}

fn interpolate(new: Vec<u8>, old: Vec<u8>) -> Vec<u8> {
    new.iter()
        .cloned()
        .zip(old.iter().cloned())
        .map(|(new, old)| if new > old { new } else { (new + old) / 2 })
        .collect()
}

fn spiral_points(keyboard: Vec<u8>) -> Vec<Vertex> {
    let len = keyboard.len();
    keyboard
        .iter()
        .rev()
        .enumerate()
        .map(|(key, vel)| {
            let rotation = Rotation2::new(f32::two_pi() / 12.0).powf(key as f32);
            let len = (key + 1) as f32 / (1 + len) as f32;
            let strength = *vel as f32 / 127.0;
            let pressed = strength * 0.2;
            let pos = rotation * Point2::new(0.0, len + pressed);
            let v = Vertex::at(Point3::new(pos.x, pos.y, 1.0));
            v.color(0.0, 1.0 - 0.8 * strength, 0.3 + 0.7 * strength, 1.0)
        })
        .collect()
}

impl Renderer<VulkanBackend, Scene> for Visual {
    fn render(&mut self, backend: &mut VulkanBackend, scene: &Scene) {
        let keyboard = interpolate(
            scene.keyboard.iter().skip(24).take(36).cloned().collect(),
            self.last_keyboard.clone(),
        );
        self.last_keyboard = keyboard.clone();
        let points = spiral_points(keyboard);


        let blue = Vector4::new(0.0, 0.0, 1.0, 1.0);
        let red = Vector4::new(1.0, 0.0, 0.0, 1.0);
        // Reference triangle
        let mesh = Mesh {
            triangles: vec![
                Triangle::new(
                    Vertex::at(Point3::new(1.0, 1.0, -1.8)).color(red.clone()),
                    Vertex::at(Point3::new(1.0, 0.9, -2.2)).color(red.clone()),
                    Vertex::at(Point3::new(0.9, 1.0, -2.2)).color(red.clone()),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(1.0, 1.0, -2.0)).color(blue.clone()),
                    Vertex::at(Point3::new(1.0, 0.9, -2.0)).color(blue.clone()),
                    Vertex::at(Point3::new(0.9, 1.0, -2.0)).color(blue.clone()),
                ),
            ],
            transform: Transform3::identity(),
        };
        backend.queue_render(mesh);
        // Rose (?
        let mesh = Mesh {
            triangles: points
                .iter()
                .cloned()
                .skip(1)
                .zip(points.iter().cloned())
                .map(|(a, b)| {
                    let c = Vertex::at(Point3::new(0.0, 0.0, -1.0)).color(1.0, 0.0, 0.2, 1.0);
                    Triangle::new(a, b, c)
                })
                .collect(),
            transform: {
                let t = scene.clock.time_in_sec();
                let height = t.sin() * 1.0;
                let distance = (5.0 * t).sin() * 2.0 - 6.0;
                let rotation_y = t.sin() * 1.0;
                let rotation_z = (t * 3.0).sin();
                Transform3::identity()
                    * Translation3::new(0.0, height, distance)
                    * Rotation3::from_euler_angles(0.0, rotation_z, rotation_y)
            },
        };
        backend.queue_render(mesh);
    }
}
