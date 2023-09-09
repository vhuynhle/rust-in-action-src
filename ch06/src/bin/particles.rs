use std::{
    alloc::{GlobalAlloc, System},
    time::Instant,
};

use graphics::{
    clear,
    math::{add, mul_scalar},
    rectangle,
    types::Vec2d,
};
use piston_window::{PistonWindow, WindowSettings};
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        eprintln!("{},{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        System.dealloc(ptr, layout);
    }
}

struct Particle {
    width: f64,
    height: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y: f64 = rng.gen_range(0.0..world.height);
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            width: 4.0,
            height: 4.0,
            position: [x, y],
            velocity: [x_velocity, y_velocity],
            acceleration: [x_acceleration, y_acceleration],
            color: [1.0, 1.0, 1.0, 0.99], // almost opaque white
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995;
    }
}

struct World {
    width: f64,
    height: f64,
    particles: Vec<Box<Particle>>,
    rng: ThreadRng,
    current_turn: u64,
}

impl World {
    fn new(width: f64, height: f64) -> Self {
        World {
            width,
            height,
            particles: vec![],
            rng: thread_rng(),
            current_turn: 0,
        }
    }

    fn add_shapes(&mut self, nshapes: u32) {
        for _ in 0..nshapes {
            let particle = Particle::new(&self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        }
    }

    fn remove_shapes(&mut self, nshapes: u32) {
        if self.particles.is_empty() {
            return;
        }

        for _ in 0..nshapes {
            let mut deleted = false;
            for (i, particle) in self.particles.iter().enumerate() {
                if particle.color[3] < 0.02 {
                    self.particles.remove(i);
                    deleted = true;
                    break;
                }
            }

            if !deleted {
                self.particles.remove(0);
            }
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);
        if n > 0 {
            self.add_shapes(n as u32);
        } else {
            self.remove_shapes((-n) as u32);
        }

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }

        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .transparent(true)
        .fullscreen(false)
        .srgb(true)
        .build()
        .expect("Could not create a window");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.19], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
