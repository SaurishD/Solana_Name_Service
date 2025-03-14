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
        signature: await provider.connection.requestAirdrop(user.publicKey, 1_000_000_000), 
        blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
        lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight,}
    );



    const tx = await program.methods.addDomainAddress(domainName,user.publicKey).accountsStrict({
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
