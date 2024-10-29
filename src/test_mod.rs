fn private_fn() {
    println!("this is a private function called `test_mod::private_fn()`");
}

pub fn public_fn() {
    println!("this is a public function called `test_mod::public_fn()`");
}

pub mod nested_mod {
    fn private_fn() {
        println!("this is a private function called `test_mod::nested_mod::private_fn()`");
    }

    // Functions declared using `pub(in path)` syntax are only visible
    // within the given path. `path` must be a parent or ancestor module
    pub(in crate::test_mod) fn public_function_in_test_mod() {
        print!("called `test_mod::nested::public_function_in_test_mod()`, that\n> ");
        public_function_in_nested();
    }

    // Functions declared using `pub(self)` syntax are only visible within
    // the current module, which is the same as leaving them private
    pub(self) fn public_function_in_nested() {
        println!("called `test_mod::nested::public_function_in_nested()`");
    }

    // Functions declared using `pub(super)` syntax are only visible within
    // the parent module
    pub(super) fn public_function_in_super_mod() {
        println!("called `test_mod::nested::public_function_in_super_mod()`");
    }

    pub fn public_fn() {
        println!("this is a public function called `test_mod::public_fn()`");
    }
}
