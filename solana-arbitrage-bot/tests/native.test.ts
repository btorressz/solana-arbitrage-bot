// No imports needed: web3, borsh, pg, and more are globally available

import assert from "assert";

/**
 * Represents a liquidity pool account state
 */
class LiquidityPool {
  totalTokens = 0;
  constructor(fields: { totalTokens: number } | undefined = undefined) {
    if (fields) {
      this.totalTokens = fields.totalTokens;
    }
  }
}

/**
 * Borsh schema definition for liquidity pool accounts
 */
const LiquidityPoolSchema = new Map([
  [LiquidityPool, { kind: "struct", fields: [["totalTokens", "u64"]] }],
]);

/**
 * The expected size of each liquidity pool account.
 */
const LIQUIDITY_POOL_SIZE = borsh.serialize(
  LiquidityPoolSchema,
  new LiquidityPool()
).length;

describe("Arbitrage Bot On-Chain Test", () => {
  it("should initialize liquidity pools and perform an arbitrage", async () => {
    // Step 1: Create two liquidity pool accounts
    const pool1Kp = new web3.Keypair();
    const pool2Kp = new web3.Keypair();

    // Get the minimum balance for rent exemption
    const lamports = await pg.connection.getMinimumBalanceForRentExemption(
      LIQUIDITY_POOL_SIZE
    );

    // Create instructions to initialize both liquidity pool accounts
    const createPool1Ix = web3.SystemProgram.createAccount({
      fromPubkey: pg.wallet.publicKey,
      lamports,
      newAccountPubkey: pool1Kp.publicKey,
      programId: pg.PROGRAM_ID,
      space: LIQUIDITY_POOL_SIZE,
    });

    const createPool2Ix = web3.SystemProgram.createAccount({
      fromPubkey: pg.wallet.publicKey,
      lamports,
      newAccountPubkey: pool2Kp.publicKey,
      programId: pg.PROGRAM_ID,
      space: LIQUIDITY_POOL_SIZE,
    });

    // Step 2: Create instructions to initialize the pools
    const initPool1Ix = new web3.TransactionInstruction({
      keys: [
        { pubkey: pool1Kp.publicKey, isSigner: false, isWritable: true },
        { pubkey: pg.wallet.publicKey, isSigner: true, isWritable: false },
      ],
      programId: pg.PROGRAM_ID,
      data: Buffer.from(Uint8Array.of(0, ...new BN(1000).toArray("le", 8))), // Initialize pool with 1000 tokens
    });

    const initPool2Ix = new web3.TransactionInstruction({
      keys: [
        { pubkey: pool2Kp.publicKey, isSigner: false, isWritable: true },
        { pubkey: pg.wallet.publicKey, isSigner: true, isWritable: false },
      ],
      programId: pg.PROGRAM_ID,
      data: Buffer.from(Uint8Array.of(0, ...new BN(2000).toArray("le", 8))), // Initialize pool with 2000 tokens
    });

    // Step 3: Execute an arbitrage instruction
    const arbitrageIx = new web3.TransactionInstruction({
      keys: [
        { pubkey: pool1Kp.publicKey, isSigner: false, isWritable: true },
        { pubkey: pool2Kp.publicKey, isSigner: false, isWritable: true },
        { pubkey: pg.wallet.publicKey, isSigner: true, isWritable: false },
      ],
      programId: pg.PROGRAM_ID,
      data: Buffer.from(Uint8Array.of(1, ...new BN(500).toArray("le", 8))), // Arbitrage 500 tokens
    });

    // Step 4: Create a transaction and add all the instructions
    const tx = new web3.Transaction();
    tx.add(createPool1Ix, createPool2Ix, initPool1Ix, initPool2Ix, arbitrageIx);

    // Step 5: Send and confirm the transaction
    const txHash = await web3.sendAndConfirmTransaction(pg.connection, tx, [
      pg.wallet.keypair,
      pool1Kp,
      pool2Kp,
    ]);
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Step 6: Fetch and verify the state of the liquidity pools
    const pool1Account = await pg.connection.getAccountInfo(pool1Kp.publicKey);
    const pool2Account = await pg.connection.getAccountInfo(pool2Kp.publicKey);

    if (!pool1Account || !pool2Account) {
      throw new Error("Failed to fetch pool accounts");
    }

    // Deserialize the account data
    const pool1Data = borsh.deserialize(
      LiquidityPoolSchema,
      LiquidityPool,
      pool1Account.data
    );
    const pool2Data = borsh.deserialize(
      LiquidityPoolSchema,
      LiquidityPool,
      pool2Account.data
    );

    // Assertions to verify the arbitrage operation was successful
    assert.strictEqual(pool1Account.lamports, lamports);
    assert.strictEqual(pool2Account.lamports, lamports);
    assert(pool1Account.owner.equals(pg.PROGRAM_ID));
    assert(pool2Account.owner.equals(pg.PROGRAM_ID));
    assert.strictEqual(pool1Data.totalTokens, 500); // Should be reduced by 500
    assert.strictEqual(pool2Data.totalTokens, 2500); // Should be increased by 500
  });
});
