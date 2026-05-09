describe("solana_strive", () => {
  
  const goalAccount = anchor.web3.Keypair.generate();

  it("Goal Create Kar rha hy!", async () => {
    // 0.01 SOL = 10,000,000 Lamports
    const description = "Learn Solana Development";
    const amount = new anchor.BN(10000000);

    const tx = await pg.program.methods
      .createGoal(description, amount)
      .accounts({
        goalAccount: goalAccount.publicKey,
        user: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([goalAccount]) 
      .rpc();

    console.log("Goal created successfully! Signature:", tx);
  });

  it("Goal Complete aur SOL wapis!", async () => {
    const tx = await pg.program.methods
      .completeGoal()
      .accounts({
        goalAccount: goalAccount.publicKey,
        user: pg.wallet.publicKey,
      })
      .rpc();

    console.log("Goal completed successfully! Signature:", tx);
  });
});
