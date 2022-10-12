import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import { Barca } from "../target/types/barca";
import { Token } from "@solana/spl-token";
import { nanoid } from "nanoid";

describe("Barcelona escrow demo", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Barca as Program<Barca>;

  let offeredTokenMint: Token = null;
  let wantedTokenMint: Token = null;

  let initializerOfferedTokenAccount: PublicKey = null;
  let initializerWantedTokenAccount: PublicKey = null;
  let takerOfferedTokenAccount: PublicKey = null;
  let takerWantedTokenAccount: PublicKey = null;

  const takerAmount = 1000;
  const initializerAmount = 500;

  let payer: Keypair = null;
  let mintAuthority: Keypair = null;
  let initializerMainAccount: Keypair = null;
  let takerMainAccount: Keypair = null;

  beforeEach(async () => {
    payer = anchor.web3.Keypair.generate();
    mintAuthority = anchor.web3.Keypair.generate();
    initializerMainAccount = anchor.web3.Keypair.generate();
    takerMainAccount = anchor.web3.Keypair.generate();

    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        payer.publicKey,
        LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        initializerMainAccount.publicKey,
        LAMPORTS_PER_SOL * 2
      ),
      "confirmed"
    );

    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        takerMainAccount.publicKey,
        LAMPORTS_PER_SOL * 2
      ),
      "confirmed"
    );

    // Creating offered Mint
    offeredTokenMint = await Token.createMint(
      program.provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    // Creating offered Mint
    wantedTokenMint = await Token.createMint(
      program.provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    // Offered Token Accounts for both initializer and taker
    initializerOfferedTokenAccount = await offeredTokenMint.createAccount(
      initializerMainAccount.publicKey
    );
    takerOfferedTokenAccount = await offeredTokenMint.createAccount(
      takerMainAccount.publicKey
    );

    // Wanted Token Accounts for both initializer and taker
    initializerWantedTokenAccount = await wantedTokenMint.createAccount(
      initializerMainAccount.publicKey
    );
    takerWantedTokenAccount = await wantedTokenMint.createAccount(
      takerMainAccount.publicKey
    );

    // Minting offered Tokens to initializer's offered tokens token account
    await offeredTokenMint.mintTo(
      initializerOfferedTokenAccount,
      mintAuthority.publicKey,
      [mintAuthority],
      initializerAmount
    );

    // Minting wanted Tokens to taker's wanted tokens token account
    await wantedTokenMint.mintTo(
      takerWantedTokenAccount,
      mintAuthority.publicKey,
      [mintAuthority],
      takerAmount
    );

    const gotInitializerOfferedTokenAccount =
      await offeredTokenMint.getAccountInfo(initializerOfferedTokenAccount);
    const gotTakerWantedTokenAccount = await wantedTokenMint.getAccountInfo(
      takerWantedTokenAccount
    );

    assert.ok(
      gotInitializerOfferedTokenAccount.amount.toNumber() == initializerAmount
    );
    assert.ok(gotTakerWantedTokenAccount.amount.toNumber() == takerAmount);
  });

  it("#001 - Init escrow", async () => {
    const [offer] = await PublicKey.findProgramAddress(
      [initializerMainAccount.publicKey.toBuffer()],
      program.programId
    );

    const [escrowedOfferedTokensAccountPda] =
      await PublicKey.findProgramAddress([offer.toBuffer()], program.programId);

    const offeredAmount = new anchor.BN(100);
    const wantedAmount = new anchor.BN(200);

    await program.rpc.initializeEscrow(offeredAmount, wantedAmount, {
      accounts: {
        offer: offer,
        initializer: initializerMainAccount.publicKey,
        initializerOfferedTokensAccount: initializerOfferedTokenAccount,
        escrowedOfferedTokensAccountPda: escrowedOfferedTokensAccountPda,
        offeredTokensMint: offeredTokenMint.publicKey,
        wantedTokensMint: wantedTokenMint.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [initializerMainAccount],
    });

    const gotOffer = await program.account.offer.fetch(offer);
    const gotEscrowedOfferedTokensAccountPda =
      await offeredTokenMint.getAccountInfo(escrowedOfferedTokensAccountPda);
    assert.equal(
      gotEscrowedOfferedTokensAccountPda.amount.toNumber(),
      offeredAmount.toNumber()
    );
  });

  it("#002 Cancel escrow", async () => {
    const [offer] = await PublicKey.findProgramAddress(
      [initializerMainAccount.publicKey.toBuffer()],
      program.programId
    );

    const [escrowedOfferedTokensAccountPda] =
      await PublicKey.findProgramAddress([offer.toBuffer()], program.programId);

    const offeredAmount = new anchor.BN(100);
    const wantedAmount = new anchor.BN(200);
    await program.rpc.initializeEscrow(offeredAmount, wantedAmount, {
      accounts: {
        offer: offer,
        initializer: initializerMainAccount.publicKey,
        initializerOfferedTokensAccount: initializerOfferedTokenAccount,
        escrowedOfferedTokensAccountPda: escrowedOfferedTokensAccountPda,
        offeredTokensMint: offeredTokenMint.publicKey,
        wantedTokensMint: wantedTokenMint.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [initializerMainAccount],
    });

    await program.rpc.cancelEscrow({
      accounts: {
        offer: offer,
        initializer: initializerMainAccount.publicKey,
        initializersOfferedTokensAccount: initializerOfferedTokenAccount,
        escrowedOfferedTokensAccountPda: escrowedOfferedTokensAccountPda,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [initializerMainAccount],
    });
  });

  it("#003 Accept escrow", async () => {
    const [offer] = await PublicKey.findProgramAddress(
      [initializerMainAccount.publicKey.toBuffer()],
      program.programId
    );

    const [escrowedOfferedTokensAccountPda] =
      await PublicKey.findProgramAddress([offer.toBuffer()], program.programId);

    const offeredAmount = new anchor.BN(100);
    const wantedAmount = new anchor.BN(200);
    await program.rpc.initializeEscrow(offeredAmount, wantedAmount, {
      accounts: {
        offer: offer,
        initializer: initializerMainAccount.publicKey,
        initializerOfferedTokensAccount: initializerOfferedTokenAccount,
        escrowedOfferedTokensAccountPda: escrowedOfferedTokensAccountPda,
        offeredTokensMint: offeredTokenMint.publicKey,
        wantedTokensMint: wantedTokenMint.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [initializerMainAccount],
    });

    await program.rpc.acceptEscrow({
      accounts: {
        offer: offer,
        escrowedOfferedTokensAccountPda: escrowedOfferedTokensAccountPda,
        initializer: initializerMainAccount.publicKey,
        initializersWantedTokenAccount: initializerWantedTokenAccount,
        taker: takerMainAccount.publicKey,
        takerWantedTokenAccount: takerWantedTokenAccount,
        takersOfferedTokensAccount: takerOfferedTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        wantedTokenMint: wantedTokenMint.publicKey,
      },
      signers: [takerMainAccount],
    });
  });
});
