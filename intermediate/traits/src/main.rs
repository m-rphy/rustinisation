trait Park {
    // delcaring an interface that
    // needs to be defined
    fn park(&self);
}

// This how you use inheritance
// (not really inheritance...)
struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}

impl Park for Car {
    fn park(&self) {
        println!("parking car");
    }
}

// NOTE - this can be implemented for many different structs!!!
trait Paint {
    // default implementation (don't need to define when implemented)
    fn paint(&self, color: String) {
        println!("Painting object: {}", color)
    }
}

// The default implementaiton is the only implementation
impl Paint for Truck {}

// You can over write the default implementation is desired
struct House {
    color: String
}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting a house!: {}", color);
    }
}

struct Truck {
    info: VehicleInfo
}


#[allow(dead_code)]
impl Truck {
    
    fn unload(&self) {
        println!("unloading Truck");
    }
    
    fn park(&self) {
        println!("parking car");
    }
}

fn main () {

    println!("Hello World!");
}
