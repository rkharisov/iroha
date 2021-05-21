#![recursion_limit = "1000000000000000"]

use iroha_introspect::prelude::*;
use iroha_data_model::prelude::*;

fn main() {
    let meta = iroha_data_model::expression::Expression::introspect();
    println!("{:#?}", meta)
}
