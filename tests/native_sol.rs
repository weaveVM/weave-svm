mod setup;

use {
    setup::{system_account, TestValidatorContext},
    solana_sdk::{signature::Keypair, signer::Signer},
    wvm_svm::{transaction::PayTubeTransaction, PayTubeChannel},
};

#[test]
fn test_native_sol() {
    let alice = Keypair::new();
    let bob = Keypair::new();
    let will = Keypair::new();

    let alice_pubkey = alice.pubkey();
    let bob_pubkey = bob.pubkey();
    let will_pubkey = will.pubkey();

    let accounts = vec![
        (alice_pubkey, system_account(10_000_000)),
        (bob_pubkey, system_account(10_000_000)),
        (will_pubkey, system_account(10_000_000)),
    ];

    // Start validator before creating runtime
    let context = TestValidatorContext::start_with_accounts(accounts);
    let test_validator = &context.test_validator;
    let payer = context.payer.insecure_clone();
    let rpc_client = test_validator.get_rpc_client();

    println!("{:?}", &rpc_client.url());

    let paytube_channel = PayTubeChannel::new(vec![payer, alice, bob, will], rpc_client);

    // Create runtime after validator is started
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            paytube_channel
                .process_paytube_transfers(&[
                    PayTubeTransaction {
                        from: alice_pubkey,
                        to: bob_pubkey,
                        amount: 2_000_000,
                        mint: None,
                    },
                    PayTubeTransaction {
                        from: bob_pubkey,
                        to: will_pubkey,
                        amount: 5_000_000,
                        mint: None,
                    },
                    PayTubeTransaction {
                        from: alice_pubkey,
                        to: bob_pubkey,
                        amount: 2_000_000,
                        mint: None,
                    },
                    PayTubeTransaction {
                        from: will_pubkey,
                        to: alice_pubkey,
                        amount: 1_000_000,
                        mint: None,
                    },
                ])
                .await;
        });
    })
    .join()
    .unwrap();

    // Verify balances after all operations complete
    let rpc_client = test_validator.get_rpc_client();
    assert_eq!(rpc_client.get_balance(&alice_pubkey).unwrap(), 7_000_000);
    assert_eq!(rpc_client.get_balance(&bob_pubkey).unwrap(), 9_000_000);
    assert_eq!(rpc_client.get_balance(&will_pubkey).unwrap(), 14_000_000);
}
