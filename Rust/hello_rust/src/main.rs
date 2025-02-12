use hello_rust::string_utils::reverse::reverse;
use hello_rust::math::{add, subtract, multiply, divide};

macro_rules! create_struct_with_methods {
    ($name:ident, $($field:ident: $type:ty),*) => {
        struct $name {
            $(
                $field: $type,
            )*
        }

        impl $name {
            fn new($($field: $type),*) -> Self {
                Self { $($field),* }
            }

            fn display(&self) {
                $(
                    println!("{}: {:?}", stringify!($field), self.$field);
                )*
            }
        }
    };
}


fn main() {
    const MAX_VALUE: u32 = std::u32::MAX;
    const MIN_VALUE: u32 = std::u32::MIN;

    create_struct_with_methods!(Car, brand: String, speed: u32);
    
    let sum = add(3, 5);
    println!("Sum: {}", sum);

    let product = multiply(3, 5);
    println!("Product: {}", product);

    let reversed = reverse("hello");
    println!("Reversed: {}", reversed);

    let mycar = Car::new("Toyota".to_string(), 120);
    mycar.display();
}
