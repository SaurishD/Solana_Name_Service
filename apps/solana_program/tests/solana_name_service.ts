import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNameService } from "../target/types/solana_name_service";
import { expect } from "chai";

describe("solana_name_service", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaNameService as Program<SolanaNameService>;
  const domainName = "solana.sol"

  const user = anchor.web3.Keypair.generate()


  it("Is Adds domain address!", async () => {
    // Add your test here.
    
    const [domainPDA] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(domainName)], program.programId)

    await provider.connection.confirmTransaction( 
      {
        signature: await provider.connection.requestAirdrop(user.publicKey, 10_000_000_000), 
        blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
        lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,}
    );

    const plan = { day30: {} };

    const tx = await program.methods.addDomainAddress(domainName,user.publicKey, plan).accountsStrict({
      user: user.publicKey,
      addressStore: domainPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([user]).rpc();

    const addressStore = await program.account.addressStore.fetch(domainPDA);

    expect(addressStore.address.toString()).eq(user.publicKey.toString())
  });

  it("Updates domain address", async () => {
    const [domainPDA] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(domainName)], program.programId)

    const anotherUser = anchor.web3.Keypair.generate()

    const tx = await program.methods.updateDomainAddress(domainName,anotherUser.publicKey).accountsStrict({
      user: user.publicKey,
      addressStore: domainPDA
    }).signers([user]).rpc();

    const addressStore = await program.account.addressStore.fetch(domainPDA);

    expect(addressStore.address.toString()).eq(anotherUser.publicKey.toString())
    expect(addressStore.owner.toString()).eq(user.publicKey.toString())
  })

  it("Fetches Domain Address", async () => {
    const [domainPDA] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(domainName)], program.programId)


    const address: anchor.web3.PublicKey = await program.methods.getAddress(domainName).accountsStrict({
      user: user.publicKey,
      addressStore: domainPDA
    }).view();

    const addressStore = await program.account.addressStore.fetch(domainPDA);

    expect(addressStore.address.toString()).eq(address.toString())
  })

  it("Extends Domain Validity", async () => {
    const [domainPDA] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(domainName)], program.programId);
    const plan = {day90:{}}
    let addressStorPrev = await program.account.addressStore.fetch(domainPDA)
    const expiration_prev = addressStorPrev.expirationTime.toNumber();
    const tx = await program.methods.extendDomainValidity(domainName,plan).accountsStrict({
      user: user.publicKey,
      addressStore: domainPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([user]).rpc();

    let addressStorLater = await program.account.addressStore.fetch(domainPDA)
    const expiration_later = addressStorLater.expirationTime.toNumber();
    
    expect(expiration_later-expiration_prev).eq(90*24*60*60)
  })

  it("Deletes domain address", async () => {
    const [domainPDA] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(domainName)], program.programId)

    const userLamportBefore = await provider.connection.getBalance(user.publicKey)
    const tx = await program.methods.closeDomainMapping(domainName).accountsStrict({
      user: user.publicKey,
      addressStore: domainPDA
    }).signers([user]).rpc();

    const userLamportAfter = await provider.connection.getBalance(user.publicKey)
    const pdaLamport = await provider.connection.getBalance(domainPDA)

    expect(userLamportAfter).to.greaterThan(userLamportBefore)
    expect(pdaLamport).to.eq(0)

  })
});
