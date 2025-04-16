mod private {
    pub(super) struct PrivateImpl {
        value: i32,
    }

    impl PrivateImpl {
        pub(super) fn new(value: i32) -> Self {
            Self { value }
        }

        pub(super) fn get_value(&self) -> i32 {
            self.value
        }

        pub(super) fn times_two(&self) -> i32 {
            self.value * 2
        }
    }
}

use private::PrivateImpl;

pub struct MyClassStruct {
    private_impl: Box<PrivateImpl>,
}

impl MyClassStruct {
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
