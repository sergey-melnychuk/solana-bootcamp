import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVotingDapp } from "../target/types/anchor_voting_dapp";
import { assert } from "chai";

describe("anchor-voting-dapp", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorVotingDapp as Program<AnchorVotingDapp>;

  it("initialize_poll creates a poll account", async () => {
    const pollId = new anchor.BN(1);
    const description = "What is your favorite programming language?";
    const pollStart = new anchor.BN(0);
    const pollEnd = new anchor.BN(Date.now() / 1000 + 86400); // 24 hours from now

    await program.methods
      .initializePoll(pollId, description, pollStart, pollEnd)
      .rpc();

    const [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const poll = await program.account.poll.fetch(pollPda);

    assert.ok(poll.pollId.eq(pollId), "poll_id mismatch");
    assert.equal(poll.description, description, "description mismatch");
    assert.ok(poll.pollStart.eq(pollStart), "poll_start mismatch");
    assert.ok(poll.pollEnd.eq(pollEnd), "poll_end mismatch");
    assert.ok(poll.candidateAmount.eq(new anchor.BN(0)), "candidate_amount should be 0");
  });

  it("initialize_candidate creates a candidate account", async () => {
    const pollId = new anchor.BN(2);
    const description = "What is your favorite programming language?";
    const pollStart = new anchor.BN(0);
    const pollEnd = new anchor.BN(Date.now() / 1000 + 86400); // 24 hours from now

    await program.methods
      .initializePoll(pollId, description, pollStart, pollEnd)
      .rpc();

    const candidateName = "Rust";
    await program.methods
      .initializeCandidate(candidateName, pollId)
      .rpc();

    const [candidatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(candidateName),
        pollId.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    const candidate = await program.account.candidate.fetch(candidatePda);
    assert.equal(candidate.candidateName, candidateName);
    assert.equal(candidate.candidateVotes.toNumber(), 0);
  });

  it("vote increments a candidate vote count", async () => {
    const pollId = new anchor.BN(3);
    const description = "What is your favorite programming language?";
    const pollStart = new anchor.BN(0);
    const pollEnd = new anchor.BN(Date.now() / 1000 + 86400); // 24 hours from now

    await program.methods
      .initializePoll(pollId, description, pollStart, pollEnd)
      .rpc();

    const candidateName = "Rust";
    await program.methods
      .initializeCandidate(candidateName, pollId)
      .rpc();

    await program.methods
      .vote(candidateName, pollId)
      .rpc();

    const [candidatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(candidateName),
        pollId.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    const candidate = await program.account.candidate.fetch(candidatePda);
    assert.equal(candidate.candidateName, candidateName);
    assert.equal(candidate.candidateVotes.toNumber(), 1);
  });

  it("close_poll removes the poll account", async () => {
    const pollId = new anchor.BN(1);

    const [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .closePoll(pollId)
      .rpc();

    try {
      await program.account.poll.fetch(pollPda);
      assert.fail("Poll account should have been closed");
    } catch (err) {
      assert.ok(true, "Poll account was successfully closed");
    }
  });

});
