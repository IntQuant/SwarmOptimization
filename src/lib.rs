use std::iter;

use na::SVector;
use nalgebra as na;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

pub type Vector<const D: usize> = SVector<f32, D>;

pub fn himmelblau(input: Vector<2>) -> f32 {
    let x = input[0];
    let y = input[1];
    (x*x + y - 11.0).powf(2.0) + (x + y*y - 7.0).powf(2.0)
}

pub struct Particle<const D: usize> {
    pub position: Vector<D>,
    speed: Vector<D>,
    best_position: Vector<D>,
    best_score: f32,
}

impl<const D: usize> Particle<D> {
    fn new(rng: &mut impl Rng, distribution: f32) -> Self {
        Self {
            position: Vector::from_fn(|_, _| {rng.gen_range(-distribution..=distribution)}),
            speed: Vector::zeros(),
            best_position: Vector::zeros(),
            best_score: f32::INFINITY,
        }
    }
}

pub struct StepSettings {
    pub my_position_factor: f32,
    pub swarm_position_factor: f32,
    pub inertia_factor: f32,
}

impl Default for StepSettings {
    fn default() -> Self {
        Self { 
            my_position_factor: 1.5, 
            swarm_position_factor: 1.5,
            inertia_factor: 0.7
        }
    }
}

pub struct ParticleWorld<const D: usize> {
    pub particles: Vec<Particle<D>>,
    global_best_position: Vector<D>,
    global_best_score: f32,
}

impl<const D: usize> ParticleWorld<D> {
    pub fn new(particle_amount: usize, distribution: f32) -> Self {
        let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);
        Self {
            particles: iter::repeat_with(|| {Particle::new(&mut rng, distribution)}).take(particle_amount).collect(),
            global_best_position: Vector::zeros(),
            global_best_score: f32::INFINITY,
        }
    }

    pub fn step(&mut self, scorer: impl Fn(Vector<D>) -> f32, settings: &StepSettings) {
        for particle in self.particles.iter_mut() {
            particle.position += particle.speed;
            let score = scorer(particle.position);
            if score < particle.best_score {
                particle.best_position = particle.position;
                particle.best_score = score;
                if score < self.global_best_score {
                    self.global_best_position = particle.position;
                    self.global_best_score = score;
                }
            }
            particle.speed = particle.speed * settings.inertia_factor 
                            + (particle.best_position - particle.position)*settings.my_position_factor 
                            + (self.global_best_position - particle.position)*settings.swarm_position_factor;
        }
    }

    pub fn best_solution(&self) -> (f32, Vector<D>) {
        (self.global_best_score, self.global_best_position)
    }
}


