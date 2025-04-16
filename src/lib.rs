pub mod my_class_with_trait;
use my_class_with_trait::*;
pub mod my_class_struct;
use my_class_struct::*;

pub fn run() {
    let my_class_with_trait = MyClassWithTrait::new(5);
    println!("{}", my_class_with_trait.get_value()); // Output: 5
    println!("{}", my_class_with_trait.times_two()); // Output: 10

    let my_class_only_struct = MyClassOnlyStruct::new(12);
    println!("{}", my_class_only_struct.get_value()); // Output: 12
    println!("{}", my_class_only_struct.times_two()); // Output: 24
}
