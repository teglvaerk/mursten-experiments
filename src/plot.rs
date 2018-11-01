extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;

use mursten::{Application, Backend, Data, Renderer, Updater};
use mursten_blocks::camera::{Camera, GetCamera, CameraUpdater};
use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_blocks::midi::{MidiMessage, MidiUpdater, OnMidiMessage};
use mursten_blocks::mesh_renderer::{MeshRenderer, IntoMesh, GetMeshes};
use mursten_blocks::properties::{GetProperties, Properties};
use mursten_blocks::repl::create_repl;
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_vulkan_backend::{Uniforms, VulkanBackend};

use nalgebra::*;
use std::thread;

pub fn main() {
    //let (repl_client, repl_server) = create_repl();

    //let main_thread = thread::current();
    //let repl_thread = thread::spawn(move || {
    //    repl_client.run();
    //});

    let backend = VulkanBackend::new();
    let scene = Scene::new();
    Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(MidiUpdater::prompt())
        .add_updater(CameraUpdater::new())
        //.add_updater(repl_server)
        .add_renderer(MeshRenderer::new())
        .run(scene);

    //repl_thread.join();
}

struct Scene {
    clock: Clock,
    paused: bool,
    active_track: Track,
    w: f32,
    x: f32,
    y: f32,
    z: f32,
    cube : Cube,
    floor : Floor,
    camera : Camera,
}

impl GetCamera for Scene {
    fn get_camera<'a>(&'a self) -> (Matrix4<f32>, &'a Camera) {
        let eye = Point3::new(
            self.x * 400.0 - 200.0,
            self.y * 400.0 - 200.0,
            self.z * 400.0 - 200.0,
        );
        let view = Matrix4::from_euler_angles(0.0, self.w * 6.0 - 3.0, 0.0)
                * Matrix4::new_translation(&eye.coords);

        (view, &self.camera)
    }
}

impl GetProperties for Scene {
    fn properties<'a>(&'a mut self) -> Properties {
        Properties::new()
            .add("w", &mut self.w)
            .add("x", &mut self.x)
            .add("y", &mut self.y)
            .add("z", &mut self.z)
            .add("paused", &mut self.paused)
    }
}

impl GetMeshes for Scene {
    fn mesh_iter<'a>(&'a self) -> std::vec::IntoIter<&IntoMesh> {
        let mut v: Vec<&IntoMesh> = Vec::new();
        v.push(&self.cube);
        v.push(&self.floor);
        v.into_iter()
    }
}

#[derive(Debug)]
enum Track {
    W,
    X,
    Y,
    Z,
}

impl Data for Scene {}

impl Scene {
    pub fn new() -> Self {
        Scene {
            clock: Clock::new(),
            paused: false,
            active_track: Track::W,
            w: 1.0,
            x: 1.0,
            y: 1.0,
            z: 1.0,
            cube: Cube,
            floor: Floor,
            camera : Camera::perspective()
        }
    }
}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        if !self.paused {
            self.clock += tick;
        }
        std::thread::sleep_ms(20);
    }
}

impl OnMidiMessage for Scene {
    fn on_midi_message(&mut self, msg: MidiMessage) {
        match msg {
            MidiMessage::NoteOn(28, _) => {
                self.active_track = Track::W;
            }
            MidiMessage::NoteOn(29, _) => {
                self.active_track = Track::X;
            }
            MidiMessage::NoteOn(30, _) => {
                self.active_track = Track::Y;
            }
            MidiMessage::NoteOn(31, _) => {
                self.active_track = Track::Z;
            }
            MidiMessage::PitchBendChange(amount) => {
                let value = amount as f32 / 16383.0;
                println!("{:?}: {}", self.active_track, value);
                match self.active_track {
                    Track::W => self.w = value,
                    Track::X => self.x = value,
                    Track::Y => self.y = value,
                    Track::Z => self.z = value,
                }
            }
            msg => {
                println!("{:?}", msg);
            }
        }
    }
}

fn tesselated_rectangle(divisions: u32) -> Mesh {
    let quads = divisions + 1;

    let upper_triangles = (0..quads).map(|i| {
        let z_0 = i as f32 / quads as f32;
        let z_1 = (i + 1) as f32 / quads as f32;
        Triangle {
            v1: Point3::new(-1.0, 0.0, -z_0).into(),
            v2: Point3::new(-1.0, 0.0, -z_1).into(),
            v3: Point3::new(1.0, 0.0, -z_0).into(),
        }
    });

    let lower_triangles = (0..quads).map(|i| {
        let z_0 = i as f32 / quads as f32;
        let z_1 = (i + 1) as f32 / quads as f32;
        Triangle {
            v1: Point3::new(-1.0, 0.0, -z_1).into(),
            v2: Point3::new(1.0, 0.0, -z_1).into(),
            v3: Point3::new(1.0, 0.0, -z_0).into(),
        }
    });

    Mesh {
        // triangles: upper_triangles.chain(lower_triangles).collect(),
        triangles: vec![Triangle {
            v1: Point3::new(-1.0, 0.0, 1.0).into(),
            v2: Point3::new(1.0, 0.0, -1.0).into(),
            v3: Point3::new(1.0, 0.0, 1.0).into(),
        }],
        //transform: Transform3::identity(),
    }
}

struct Cube;

impl IntoMesh for Cube {
    fn transform(&self) -> Matrix4<f32> {
        let size = 1.;
        (Transform3::identity() *
            Similarity3::from_scaling(size * 20.0 + 0.001)).to_homogeneous()
    }
    fn mesh(&self) -> Mesh {
        let red = Vector4::new(1.0, 0.0, 0.0, 1.0);
        let green = Vector4::new(0.0, 1.0, 0.0, 1.0);
        let blue = Vector4::new(0.0, 0.5, 1.0, 1.0);
        let yellow = Vector4::new(1.0, 1.0, 0.0, 1.0);
        let purple = Vector4::new(0.0, 1.0, 1.0, 1.0);
        // Reference Unit Cube
        Mesh {
            triangles: vec![
                // +Z
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0, 1.0)).color(blue),
                    Vertex::at(Point3::new(-1.0, 1.0, 1.0)).color(blue),
                    Vertex::at(Point3::new(1.0, -1.0, 1.0)).color(blue),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(1.0, -1.0, 1.0)).color(blue),
                    Vertex::at(Point3::new(-1.0, 1.0, 1.0)).color(blue),
                    Vertex::at(Point3::new(1.0, 1.0, 1.0)).color(blue),
                ),
                // -Z
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0, -1.0)).color(yellow),
                    Vertex::at(Point3::new(-1.0, 1.0, -1.0)).color(yellow),
                    Vertex::at(Point3::new(1.0, -1.0, -1.0)).color(yellow),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(1.0, -1.0, -1.0)).color(yellow),
                    Vertex::at(Point3::new(-1.0, 1.0, -1.0)).color(yellow),
                    Vertex::at(Point3::new(1.0, 1.0, -1.0)).color(yellow),
                ),
                // +Y
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, 1.0, -1.0)).color(green),
                    Vertex::at(Point3::new(-1.0, 1.0, 1.0)).color(green),
                    Vertex::at(Point3::new(1.0, 1.0, -1.0)).color(green),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(1.0, 1.0, -1.0)).color(green),
                    Vertex::at(Point3::new(-1.0, 1.0, 1.0)).color(green),
                    Vertex::at(Point3::new(1.0, 1.0, 1.0)).color(green),
                ),
                // -Y
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0, -1.0)).color(red),
                    Vertex::at(Point3::new(-1.0, -1.0, 1.0)).color(red),
                    Vertex::at(Point3::new(1.0, -1.0, -1.0)).color(red),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(1.0, -1.0, -1.0)).color(red),
                    Vertex::at(Point3::new(-1.0, -1.0, 1.0)).color(red),
                    Vertex::at(Point3::new(1.0, -1.0, 1.0)).color(red),
                ),
                // +X
                Triangle::new(
                    Vertex::at(Point3::new(1.0, -1.0, -1.0)).color(red),
                    Vertex::at(Point3::new(1.0, -1.0, 1.0)).color(red),
                    Vertex::at(Point3::new(1.0, 1.0, -1.0)).color(red),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(1.0, 1.0, -1.0)).color(red),
                    Vertex::at(Point3::new(1.0, -1.0, 1.0)).color(red),
                    Vertex::at(Point3::new(1.0, 1.0, 1.0)).color(red),
                ),
                // -X
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0, -1.0)).color(purple),
                    Vertex::at(Point3::new(-1.0, -1.0, 1.0)).color(purple),
                    Vertex::at(Point3::new(-1.0, 1.0, -1.0)).color(purple),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, 1.0, -1.0)).color(purple),
                    Vertex::at(Point3::new(-1.0, -1.0, 1.0)).color(purple),
                    Vertex::at(Point3::new(-1.0, 1.0, 1.0)).color(purple),
                ),
            ],
        }
    }
}

struct Floor;

impl IntoMesh for Floor {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::identity()
    }
    fn mesh(&self) -> Mesh {
        let green = Vector4::new(0.0, 1.0, 0.0, 1.0);
        Mesh {
            triangles: vec![
                Triangle::new(
                    Vertex::at(Point3::new(0.0, 0.0, 0.0)).color(green),
                    Vertex::at(Point3::new(120.0, 0.0, -80.0)),
                    Vertex::at(Point3::new(120.0, 0.0, 0.0)),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(0.0, 0.0, 0.0)).color(green),
                    Point3::new(0.0, 0.0, -80.0).into(),
                    Point3::new(120.0, 0.0, -80.0).into(),
                ),
            ],
        }
    }
}
