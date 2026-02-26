use anchor_lang::prelude::*;

use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_instruction::{AccountMeta, Instruction};
use solana_transaction::Transaction;
use solana_program::hash;

use litesvm::LiteSVM;

#[test]
fn test_initialize_poll() {
    let mut svm = LiteSVM::new();

    let program_id = crate::ID;
    let program_bytes = include_bytes!("../../../target/deploy/anchor_voting_dapp.so");
    svm.add_program(program_id, program_bytes).expect("add_program");

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let poll_id: u64 = 1;
    let description = "Test Poll";
    let poll_start: u64 = 0;
    let poll_end: u64 = u64::MAX;

    // Derive the poll PDA — seeds = [poll_id.to_le_bytes()]
    let (poll_pda, _bump) = Pubkey::find_program_address(
        &[&poll_id.to_le_bytes()],
        &program_id,
    );

    // Anchor instruction data: 8-byte discriminator + borsh-encoded args
    let discriminator = &hash::hash(b"global:initialize_poll").to_bytes()[..8];
    let mut data = discriminator.to_vec();
    data.extend_from_slice(&poll_id.to_le_bytes());
    // String borsh encoding: 4-byte LE length prefix + utf8 bytes
    data.extend_from_slice(&(description.len() as u32).to_le_bytes());
    data.extend_from_slice(description.as_bytes());
    data.extend_from_slice(&poll_start.to_le_bytes());
    data.extend_from_slice(&poll_end.to_le_bytes());

    let system_program_id = anchor_lang::solana_program::system_program::ID;

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),           // signer
            AccountMeta::new(poll_pda, false),                // poll PDA
            AccountMeta::new_readonly(system_program_id, false),
        ],
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx).unwrap();
    println!("Transaction logs: {:?}", result.logs);

    // Fetch and deserialize the poll account
    let account = svm.get_account(&poll_pda).expect("poll account not found");
    let poll = crate::Poll::try_deserialize(&mut account.data.as_slice()).unwrap();

    assert_eq!(poll.poll_id, poll_id);
    assert_eq!(poll.description, description);
    assert_eq!(poll.poll_start, poll_start);
    assert_eq!(poll.poll_end, poll_end);
    assert_eq!(poll.poll_index, 0);
    assert_eq!(poll.candidate_amount, 0);

    // Close the poll after it was initialized

    let close_discriminator = &hash::hash(b"global:close_poll").to_bytes()[..8];
    let mut close_data = close_discriminator.to_vec();
    close_data.extend_from_slice(&poll_id.to_le_bytes());

    let close_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),  // signer (receives lamports)
            AccountMeta::new(poll_pda, false),       // poll PDA to close
        ],
        data: close_data,
    };

    let close_tx = Transaction::new_signed_with_payer(
        &[close_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(close_tx).unwrap();
    println!("Close transaction logs: {:?}", result.logs);

    assert!(svm.get_account(&poll_pda).is_none(), "poll account should be closed");
}
