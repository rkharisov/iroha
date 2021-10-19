#![allow(clippy::restriction)]

use std::thread;

use iroha_client::client::transaction;
use iroha_core::config::Configuration;
use iroha_crypto::{Hash, HashOf};
use iroha_data_model::prelude::*;
use test_network::{*, Peer as TestPeer};

#[test]
fn client_can_query_transaction_by_hash() {
    let (_rt, peer, mut iroha_client) = <TestPeer>::start_test_with_runtime();
    let pipeline_time = Configuration::pipeline_time();

    // Given
    thread::sleep(pipeline_time);

    // let account_id = AccountId::new("alice", "wonderland");
    let asset_definition_id = AssetDefinitionId::new("xor", "wonderland");
    let create_asset = RegisterBox::new(IdentifiableBox::AssetDefinition(
        AssetDefinition::new_quantity(asset_definition_id.clone()).into(),
    ));

    let transaction = iroha_client
        .build_transaction(vec![create_asset.into()], UnlimitedMetadata::new())
        .expect("Failed to create transaction");
    iroha_client
        .submit_transaction(transaction.clone())
        .expect("Failed to prepare state.");

    let tx_hash: Hash = transaction.hash().into();

    thread::sleep(pipeline_time * 2);

    let tx_in_wsv = peer.iroha
        .as_ref()
        .expect("Could not get peer")
        .wsv
        .has_transaction(&HashOf::from_hash(tx_hash));
    assert!(tx_in_wsv);

    let request = iroha_client::client::transaction::by_hash(tx_hash);
    let response = iroha_client.request(request);
    if let Ok(TransactionValue::Transaction(_)) = response {
        //check if transaction is same as was sent
    } else {
      assert!(false, "Expected result is transaction, but was : {:?}", response)
    }
}

