use vector3d::Vector3d;
pub struct Element{
    pub symbol: String,
    pub mass: f64,
    pub radius: f64,
}

pub struct Atom<T>{
    pub pos: Vector3d<T>,
    pub vel: Vector3d<T>,
    pub element: Element,
}

pub struct Simulation{
    pub atoms: Vec<Atom<f64>>,
    pub t: f64,
    pub dt: f64,
}

impl Simulation {
    fn default() -> Self {
        Self {
            atoms: Vec::new(),
            t: 0.0,
            dt: 1.0,
        }
    }

    pub fn main(&mut self) {
        for atom in &mut self.atoms {
            atom.vel = atom.vel + Vector3d::new(0.0, 0.0, -9.8) * self.dt;
            atom.pos = atom.pos + atom.vel * self.dt;
        }
    }

    pub fn clock(&mut self){
        self.t += self.dt;
    }
}