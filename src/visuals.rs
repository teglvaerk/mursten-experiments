extern crate graphics;
extern crate image;
extern crate markov;
extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;
extern crate piston_window;
extern crate rand;
extern crate reqwest;

use mursten::{Application, Backend, Data, Renderer, Updater};
use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_vulkan_backend::{Uniforms, VulkanBackend};
use nalgebra::*;

pub fn main() {
    let backend = VulkanBackend::new();
    let mut variables = Variables::default();
    Application::new(backend)
        .add_updater(ColorRotator)
        .add_renderer(Visual::new())
        .run(variables);
}

struct Variables {
    center: Point2<f32>,
    separation: Vector2<f32>,
    matrix_size: Vector2<f32>,
    ray_proportion: f32,
    glow_amount: f32,     // < 0
    cross_intensity: f32, // < 0
    current_color: Vector3<f32>,
}

impl Variables {
    pub fn new(center: Point2<f32>) -> Self {
        Variables {
            center,
            ..Variables::default()
        }
    }
}

impl Default for Variables {
    fn default() -> Self {
        Variables {
            center: Point2::new(0.0, 0.0),
            separation: Vector2::repeat(0.06),
            matrix_size: Vector2::new(15.0, 15.0),
            ray_proportion: 4.0,
            glow_amount: 5.0,
            cross_intensity: 6.0,
            current_color: Vector3::new(0.1, 0.6, 0.9),
        }
    }
}

impl Data for Variables {}

struct ColorRotator;

impl<B> Updater<B, Variables> for ColorRotator {
    fn update(&mut self, _: &mut B, var: &mut Variables) {
        var.current_color =
            Matrix3::new(0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0) * var.current_color;
    }
}

struct Visual {}

impl Visual {
    pub fn new() -> Self {
        Visual {}
    }
}

fn ray(pos: Point2<f32>, rot: Rotation2<f32>, len: f32) -> Mesh {
    // Transformaciones esteticas
    let scale = 0.04;
    let rot = {
        let rpos = rot * pos;
        Rotation3::rotation_between(
            &Vector3::new(pos.x, pos.y, 0.0),
            &Vector3::new(rpos.x, rpos.y, 0.0),
        ).unwrap()
    };
    let pos = Point3::new(pos.x, pos.y, -600.0 + len.abs() * 3.0);
    let len = len.sqrt();

    let r = Vertex::from(pos).color(1.0, 0.0, 0.0, 0.8);
    let g = Vertex::from(pos + rot * Vector3::new(2.0 * len, 0.0, 0.0) * scale)
        .color(0.0, 1.0, 0.0, 0.8);
    let b = Vertex::from(pos + rot * Vector3::new(4.0 * len, 0.0, 0.0) * scale)
        .color(0.0, 0.0, 1.0, 0.8);
    let v1 = Vertex::from(pos + rot * Vector3::new(-1.0 * len, 0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);
    let v2 = Vertex::from(pos + rot * Vector3::new(1.0 * len, 0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);
    let v3 = Vertex::from(pos + rot * Vector3::new(3.0 * len, 0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);
    let v4 = Vertex::from(pos + rot * Vector3::new(5.0 * len, 0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);
    let v5 = Vertex::from(pos + rot * Vector3::new(-1.0 * len, -0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);
    let v6 = Vertex::from(pos + rot * Vector3::new(1.0 * len, -0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);
    let v7 = Vertex::from(pos + rot * Vector3::new(3.0 * len, -0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);
    let v8 = Vertex::from(pos + rot * Vector3::new(5.0 * len, -0.4, 0.0) * scale)
        .color(0.0, 0.0, 0.0, 0.0);

    Mesh {
        triangles: vec![
            Triangle::new(r, v1, v2),
            Triangle::new(r, v5, v1),
            Triangle::new(r, v6, v5),
            Triangle::new(v2, g, r),
            Triangle::new(v6, r, g),
            Triangle::new(g, v2, v3),
            Triangle::new(g, v7, v6),
            Triangle::new(v3, b, g),
            Triangle::new(v7, g, b),
            Triangle::new(b, v3, v4),
            Triangle::new(b, v4, v8),
            Triangle::new(b, v8, v7),
        ],
        transform: Transform3::identity(),
    }
}

impl Renderer<VulkanBackend, Variables> for Visual {
    fn render(&mut self, backend: &mut VulkanBackend, var: &Variables) {
        let (w, h) = (20, 20);
        //let (w, h) = backend.screen_size();
        backend.set_uniforms(Uniforms {
            projection: Orthographic3::new(-1.0, 1.0, -1.0, 1.0, 10.0, 900.0).to_homogeneous(),
            ..Uniforms::default()
        });

        backend.queue_render({
            let lu = Vertex::at(Point3::new(-1.0, -1.0, 0.0)).color(0.05, 0.05, 0.05, 1.0);
            let ru = Vertex::at(Point3::new(1.0, -1.0, 0.0)).color(0.05, 0.05, 0.05, 1.0);
            let ld = Vertex::at(Point3::new(-1.0, 1.0, 0.0)).color(0.05, 0.05, 0.05, 1.0);
            let rd = Vertex::at(Point3::new(1.0, 1.0, 0.0)).color(0.05, 0.05, 0.05, 1.0);
            Mesh {
                triangles: vec![Triangle::new(lu, ru, ld), Triangle::new(ld, ru, rd)],
                transform: Transform3::identity() * Translation3::new(0.0, 0.0, -800.0),
            }
        });

        use rand::distributions::normal::Normal;
        use rand::distributions::IndependentSample;
        let normal = Normal::new(1.0, 0.1);
        let mut rng = rand::thread_rng();

        let mut Q: Vec<(Point2<f32>, Rotation2<f32>)> = Vec::new();

        for j in 0..(var.matrix_size.y as u32 * 2 + 1) {
            for i in 0..(var.matrix_size.x as u32 * 2 + 1) {
                if (i, j) == (var.matrix_size.x as u32, var.matrix_size.y as u32) {
                    continue;
                }
                let p = var.matrix_size - Vector2::new(i as f32, j as f32);
                let q = var.center + p.component_mul(&var.separation);
                let r = Rotation2::rotation_between(&Vector2::x(), &p);
                Q.push((q, r));
            }
        }

        Q.sort_by(|a, b| {
            (b.0.coords.norm() * -1.0)
                .partial_cmp(&a.0.coords.norm())
                .unwrap()
        });

        for (q, rot) in Q {
            let (x, y) = (q.x, q.y);
            let len = normal.ind_sample(&mut rng) as f32 / (q.coords.norm() * 10.0);
            backend.queue_render(ray(q, rot, len));
        }

        //
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(20));
    }
}

mod equations {
    use nalgebra::*;
    use std::f32::consts::{E, PI};
    use std::f32::EPSILON;

    pub fn transform(
        point: &Point2<f32>,
        center: &Point2<f32>,
        pivot: &Point2<f32>,
        rot: &Rotation2<f32>,
        proportion: f32,
    ) -> Point2<f32> {
        let scale = Matrix3::new_nonuniform_scaling(&Vector2::new(1.0 / proportion, 1.0));
        Point2::from_homogeneous(scale * (rot * (point - pivot.coords)).to_homogeneous()).unwrap()
    }
    pub fn ray_intensity(point: &Point2<f32>) -> f32 {
        let (x, y) = (point.x, point.y);
        E.powf(-4.0 * x.powi(4) + 8.0 * x.powi(3) - 4.0 * x.powi(2) - 100.0 * y.powi(6))
    }
    pub fn cross_intensity(point: &Point2<f32>, intensity: f32) -> f32 {
        E.powf(-(0.001 / intensity.powi(4)) * (point.x * point.y).powi(2))
    }
    pub fn red_intensity(scalar: f32) -> f32 {
        (PI * scalar.min(0.5)).cos().powi(2)
    }
    pub fn green_intensity(scalar: f32) -> f32 {
        (PI * scalar).sin().powi(2)
    }
    pub fn blue_intensity(scalar: f32) -> f32 {
        (PI * scalar.max(0.5)).cos().powi(2)
    }
    pub fn glow_amount(scalar: f32, intensity: f32) -> f32 {
        E.powf(-scalar / intensity)
    }

    #[test]
    fn test_cross_intensity() {
        // Center glows at full intensity
        let v = Point2::new(0.0, 0.0);
        let i = 0.123123;
        assert_eq!(cross_intensity(&v, i), 1.0);

        // Also the axis
        let v = Point2::new(0.0, 4621.0);
        let i = 1.0123;
        assert_eq!(cross_intensity(&v, i), 1.0);

        // With intensity 1 at a distance ~7 the rays intensity ~halves
        let v = Point2::new(5.0, 5.0);
        let i = 1.0;
        assert!(cross_intensity(&v, i) - 0.53526145 < EPSILON);

        // With intensity 1 at a distance ~14 the rays intensity already unnoticeable
        let v = Point2::new(10.0, 10.0);
        let i = 1.0;
        assert!(cross_intensity(&v, i) < 0.0001);

        // You need to duplicate the intensity to compensate
        let v = Point2::new(10.0, 10.0);
        let i = 2.0;
        assert!(cross_intensity(&v, i) - 0.53526145 < EPSILON);
    }
}
