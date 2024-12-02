use crate::jakobi_method::jakobi_method;
use crate::simple_iteration::simple_iteration;
use crate::square_roots_method::square_root_method;

mod simple_iteration;
mod square_roots_method;
mod jakobi_method;

fn main() {
    // simple_iteration();
    // square_root_method();
    jakobi_method();
}
