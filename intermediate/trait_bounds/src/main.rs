
struct VehicleInfo {
    model: String,
    make: String,
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

impl Paint for Car {
    fn paint(&self, color: String) {
        println!("Painting a Car");
    }
}

struct Truck {
    info: VehicleInfo
}

impl Truck {

    fn unload(&self) {
        println!("unloading Truck");
    }

    fn park(&self) {
        println!("parking car");
    }
}

trait Park {
    // delcaring an interface that
    // needs to be defined
    fn park(&self);
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

fn main() {

    let car = Car {
        info: VehicleInfo {
            make: "Toyota".to_owned(),
            model: "Tacoma".to_owned(),
            year: 2024
        }
    };

    let house = House {
        color: "White".to_owned(),
    };

    let object = create_paintable_obj();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);
}

// Syntax 1
// T is restricted to any type that implement the trait Paint
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

// Syntax 2
// Syntax sugar for the same code above
fn paint_red2(object: & impl Paint) {
    object.paint("red".to_owned());
}

// Syntax 3
// If you have multiple trait bounds, this is a useful syntax
// This function is only accessible for any function 
// that implements the Paint trait AND Park trait
fn paint_vehicle_red3<T>(object: &T) where T: Paint + Park {
    object.paint("red".to_owned());
}

// You can also have a return type with that is paintable!
fn create_paintable_obj() -> impl Paint {
    House {
        color: String::from("White"),
    }
}
