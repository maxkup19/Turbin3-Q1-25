import { transfer } from '@solana/spl-token';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL, sendAndConfirmRawTransaction } from "@solana/web3.js";
import { assert } from "chai";
import BN from "bn.js";


describe("anchor-vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Vault as Program<AnchorVault>;
  const connection = provider.connection;

  const signer = anchor.web3.Keypair.generate();

  const vault_state = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), signer.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = PublicKey.findProgramAddressSync(
    [vault_state.toBuffer()],
    program.programId
  )[0];

  before(async () => {

    await airdrop(connection, signer.publicKey, 5);
    await airdrop(connection, vault, 1);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accountsStrict({
        user: signer.publicKey,
        state: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .catch(err => {
        console.log(err);
      });

    console.log("✅ Your Initialization transaction signature", tx);
  });

  it("Is Deposited!", async () => {
    const tx = await program.methods
      .deposit(new BN(2_000_000_000))
      .accountsStrict({
        user: signer.publicKey,
        state: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();
    console.log("✅ Your Deposit transaction signature", tx);

    let vaultBalance = await getBalance(connection, vault);
    assert.equal(vaultBalance, 2 * 1_000_000_000);
  });

  it("Is Withdraw!", async () => {
    const tx = program.methods
      .withdraw(new BN(1))
      .accountsStrict({
        user: signer.publicKey,
        state: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    let vaultBalance = await getBalance(connection, vault);
    assert.equal(vaultBalance, 2 * 1_000_000_000);

    let signerBalance = await getBalance(connection, signer.publicKey);
    assert.isTrue(signerBalance >= 4.9);
  });

  it("Is Closed!", async () => {
    const tx = await program.methods
      .close()
      .accountsStrict({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();
  });
});

async function airdrop(connection: anchor.web3.Connection, address: PublicKey, amount: number) {
  let airdropSignature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdropSignature);

  await connection.confirmTransaction(airdropSignature);

  console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", airdropSignature);

  return airdropSignature;
}

async function calculateRentExemption(
  connection: anchor.web3.Connection,
  address: PublicKey
) {
  let accountInfo = await connection.getAccountInfo(address);
  let accountSize;

  if (accountInfo === null) {
    accountSize = 1000;
  } else {
    accountSize = accountInfo.data.length;
  }

  const rentExemptionAmount = await connection.getMinimumBalanceForRentExemption(
    accountSize
  );

  return rentExemptionAmount;
}

async function getBalance(
  connection: anchor.web3.Connection,
  address: PublicKey
) {
  let accountInfo = await connection.getAccountInfo(address);
  return accountInfo.lamports;
}
