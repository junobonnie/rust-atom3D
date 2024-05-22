mod atom;
use atom::*;
use vector3d::Vector3d;

fn main() {
    let a = Vector3d::new(1.0, 2.0, 3.0);
    let b = Vector3d::new(4.0, 5.0, 6.0);
    let c = a + b;
    let d = a - b;
    let e = a.dot(b);
    let f = a.cross(b);
    let g = a.norm2();
    println!("c = {:?}", c);
    println!("d = {:?}", d);
    println!("e = {:?}", e);
    println!("f = {:?}", f);
    println!("g = {:?}", g);

    let element = Element{
        symbol: "H".to_string(),
        mass: 1.0,
        radius: 1.0,
    };

    let atom = Atom{
        pos: Vector3d::new(0.0, 0.0, 0.0),
        vel: Vector3d::new(1.0, 2.0, 0.0),
        element: element,
    };

    let mut simulation = Simulation{
        atoms: vec![atom],
        t: 0.0,
        dt: 0.001,
    };

    while simulation.t < 10.0{
        simulation.main();
        simulation.clock();
        println!("t = {}", simulation.t);
        println!("pos = {:?}", simulation.atoms[0].pos);
    }
    

    println!("Hello, world!");
}
