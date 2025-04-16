// Private trait (or interface in other languages)
trait MyTrait {
    fn get_value(&self) -> i32;
    fn times_two(&self) -> i32;
}

mod private {
    use super::*;

    pub(super) struct PrivateImpl {
        value: i32,
    }

    impl PrivateImpl {
        pub(super) fn new(value: i32) -> Self {
            Self { value }
        }
    }

    impl MyTrait for PrivateImpl {
        fn get_value(&self) -> i32 {
            self.value
        }

        fn times_two(&self) -> i32 {
            self.value * 2
        }
    }
}

use private::PrivateImpl;

// Public 'interface' (forward declaration)
pub struct MyClassWithTrait {
    private_impl: Box<dyn MyTrait>, // Trait object or hidden struct
}

// Implementation of the trait for MyClass
impl MyClassWithTrait {
    pub fn new(value: i32) -> Self {
        Self {
            private_impl: Box::new(PrivateImpl::new(value)),
        }
    }

    pub fn get_value(&self) -> i32 {
        self.private_impl.get_value()
    }

    pub fn times_two(&self) -> i32 {
        self.private_impl.times_two()
    }
}
