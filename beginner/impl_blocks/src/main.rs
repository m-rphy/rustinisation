
struct Product {
    name: String,
    price: f32,
    in_stock: bool 
}

// add functionality for a struct
// A method (or class!)
impl Product {
    // A constructor function (but an associated function)
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true
        }
    }

    // associated function
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn calc_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    } 

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought!");
        123 // a reciept number
    }
}

fn main() {

    let mut book = Product::new(
        String::from("Love in a time of Cholera"),
        28.85
        );

    let sales_tax = book.calc_sales_tax();
    println!("Sales tax is {}", sales_tax);
    book.set_price(1.0);
    book.buy();
    // Book instance is no longer valid!
}
