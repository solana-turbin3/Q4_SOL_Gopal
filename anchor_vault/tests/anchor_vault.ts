import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { getKeypairFromFile } from "@solana-developers/helpers";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

  it("Is initialized!", async () => {
    const wallet = await getKeypairFromFile("~/.config/solana/id.json");
    const [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), wallet.publicKey.toBuffer()],
      program.programId
    );
    const [state, stateBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), wallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initialize()
      .accountsPartial({
        systemProgram: anchor.web3.SystemProgram.programId,
        vault,
        state,
        user: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
