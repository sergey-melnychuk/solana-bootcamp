Project: Favorites

IDE: [online](https://beta.solpg.io)

### src/lib.rs

```rust
use anchor_lang::prelude::*;

declare_id!("4Tb8zNcYfCdtFdnNerMqaUQuak1Y15b1j6GiJ7mpfcyF");

#[program]
mod favorites {
    use super::*;
    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("set_favorites: user={}", ctx.accounts.user.key());
        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });
        Ok(())
    }

    pub fn close_favorites(_ctx: Context<CloseFavorites>) -> Result<()> {
        msg!("Closing favorites account and reclaiming rent.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed, 
        payer = user, 
        space = 8 + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        // This 'close' attribute is the magic: it zeroes the data 
        // and sends the SOL to the specified account (user).
        close = user, 
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    number: u64,
    #[max_len(50)]
    color: String,
    #[max_len(5, 50)]
    hobbies: Vec<String>,
}
```

### tests/anchor.test.ts

```typescript
describe("Test", () => {
  it("Sets and fetches favorites", async () => {
    // 1. Derive the PDA for the favorites account
    const [favoritesPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    const favoritesData = {
      number: new anchor.BN(42), // Use BN for u64
      color: 'yellow',
      hobbies: ['gym', 'boxing', 'football']
    };

    // 2. Send transaction
    const txHash = await pg.program.methods
      .setFavorites(favoritesData.number, favoritesData.color, favoritesData.hobbies)
      .accounts({
        user: pg.wallet.publicKey,
        favorites: favoritesPda, // Pass the derived PDA here
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Transaction Hash: ${txHash}`);
    await pg.connection.confirmTransaction(txHash);

    // 3. Fetch the account using the PDA
    // Note: Use the name of the struct 'favorites'
    const account = await pg.program.account.favorites.fetch(favoritesPda);

    console.log("On-chain number:", account.number.toString());
    console.log("On-chain color:", account.color);

    // 4. Assertions
    assert.equal(account.number.toNumber(), favoritesData.number.toNumber());
    assert.equal(account.color, favoritesData.color);
    assert.deepEqual(account.hobbies, favoritesData.hobbies);
  });

  it("Closes the favorites account", async () => {
    // 1. Re-derive the PDA
    const [favoritesPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    // 2. Call the close instruction
    const txHash = await pg.program.methods
      .closeFavorites()
      .accounts({
        user: pg.wallet.publicKey,
        favorites: favoritesPda,
      })
      .rpc();

    console.log(`Close Transaction: ${txHash}`);
    await pg.connection.confirmTransaction(txHash);

    // 3. Verify it's gone
    const accountInfo = await pg.connection.getAccountInfo(favoritesPda);
    assert.strictEqual(accountInfo, null, "Account should be deleted");
    
    console.log("Account successfully closed and SOL reclaimed!");
  });

});
```

### client/client.ts

```typescript
// Client
console.log("My address:", pg.wallet.publicKey.toString());
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
```

### keypair

```
$ cat wallet-keypair.json
[119,49,195,20,41,151,129,200,223,81,25,212,70,60,40,58,136,191,18,17,198,255,223,119,6,224,89,161,13,175,97,172,127,210,175,170,252,142,17,137,195,204,76,180,108,52,137,182,170,191,59,232,108,25,159,94,14,20,77,32,33,210,179,218]
```

### on-chain

https://explorer.solana.com/address/9by7MZm4e3PGxXqrc6F69hWJtS648QZCcJg6kqq6YPAH?cluster=devnet

https://explorer.solana.com/tx/2d1MzdXFrf8QmhURG18KU5fykzyRGEhQzR91BcSuUdAJTRWZPAACFtKwx5MMnzKE5RThCA4UJeY2XzUK7syFUnPS?cluster=devnet

https://explorer.solana.com/tx/2zTqHxKTVT9BBdFGSNkDAhcJ4kL3Nzg5ymYg8uELcLhKbRNdchByMX69pudxJPHc8MwefUot9sRsDFjwL8j2YMfr?cluster=devnet

https://explorer.solana.com/tx/47MdUh6J6YeRYND6j38N1YaxdUNQSYPiR2ET1GDtf8uh6etiay5LBqgjNxAsTAvTqx58KWF9Gp5dTjWc5n8CBgCn?cluster=devnet

https://explorer.solana.com/tx/3jRN2XbaQyQ54ppi9QMe87pNNZN8ZKs4NZ2qR1e38B1ZaZYHzQTmLPQgemM8QY1gPKaiqkw3fT65bL8dFfBtfBtt?cluster=devnet

