
struct Product {
    name: String,
    price: f32,
    in_stock: bool 
}


fn main() {
    
    let mut book = Product {
        name: String::from("Love in a time of Cholera"),
        price: 28.85,
        in_stock: true,
    };

    // change/access values of a struct
    let price = book.price;
    book.in_stock = false;
    

    let sales_tax = calculate_tax(&book);

    println!("Sales tax is {}", sales_tax);
}

fn calculate_tax(product: &Product) -> f32 {
    product.price * 0.1
}
