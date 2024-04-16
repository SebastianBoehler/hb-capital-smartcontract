import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HbCapitalSmartcontract } from "../target/types/hb_capital_smartcontract";
import { expect } from "chai";

describe("hb-capital-smartcontract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const signer = anchor.Wallet.local().payer;
  const program = anchor.workspace
    .HbCapitalSmartcontract as Program<HbCapitalSmartcontract>;
  const ticker = "TEST";

  const [posPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("pos"),
      anchor.utils.bytes.utf8.encode(ticker),
      signer.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("Is initialized!", async () => {
    const bump = 6;
    const tx = await program.methods
      .initialize(ticker, bump)
      .accounts({
        signer: signer.publicKey,
        //@ts-ignore
        position: posPDA,
      })
      .rpc();

    console.log("Your transaction hash", tx);

    const pda = await program.account.position.fetch(posPDA);
    console.log("PDA", pda);

    expect(pda.ticker).to.eq(ticker);
  });

  it("Add order to position", async () => {
    const order_type = 1;
    const side = 1;
    const price = new anchor.BN(60000);
    const time = new anchor.BN(Date.now() / 1000);

    const tx = await program.methods
      .addOrder(ticker, order_type, side, price, time)
      .accounts({
        signer: signer.publicKey,
        //@ts-ignore
        position: posPDA,
      })
      .rpc();

    console.log("Your transaction hash", tx);

    const pda = await program.account.position.fetch(posPDA);
    console.log("PDA", pda);

    expect(pda.orders.length).to.eq(1);
  });
});
