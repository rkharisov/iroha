use iroha_introspect::prelude::*;
use iroha_data_model::prelude::*;

fn main() {
    let _ = iroha_data_model::transaction::VersionedTransaction::introspect();
}
