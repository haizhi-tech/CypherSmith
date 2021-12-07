mod ast;
mod driver;


use driver::driver;

pub struct Driver {

}

impl Driver {
    pub fn driver() {
        driver();
    }
}