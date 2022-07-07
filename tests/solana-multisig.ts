import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaMultisig } from "../target/types/solana_multisig";

describe("solana-multisig", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaMultisig as Program<SolanaMultisig>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
