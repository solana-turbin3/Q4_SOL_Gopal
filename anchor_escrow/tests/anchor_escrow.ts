import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { getKeypairFromFile } from "@solana-developers/helpers";

describe("anchor_escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

  it("Is initialized!", async () => {
    const mintA = anchor.web3.Keypair.generate();
    const mintB = anchor.web3.Keypair.generate();

    const wallet = await getKeypairFromFile("~/.config/solana/id.json");

    const makerAtaA = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        mintA.publicKey,
        wallet.publicKey
      )
    ).address;

    const [escrow, escrow_bump] =
      anchor.web3.PublicKey.createProgramAddressSync(
        [
          Buffer.from("escrow"),
          wallet.publicKey.toBuffer(),
          Buffer.from(
            new Uint8Array(new anchor.BN(2).toArrayLike(Buffer, "le", 8))
          ),
        ],
        program.programId
      );

    const vault = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        mintA.publicKey,
        escrow
      )
    ).address;

    const tx = await program.methods
      .initialize(new anchor.BN(2), new anchor.BN(10), new anchor.BN(5))
      .accountsPartial({
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        makerAtaA,
        systemProgram: anchor.web3.SystemProgram.programId,
        vault,
        mintA: mintA.publicKey,
        mintB: mintB.publicKey,
        escrow,
        maker: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("Deposit and Withdraw", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.depositAndWithdraw().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
