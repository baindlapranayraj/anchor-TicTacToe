import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TicTacToe } from "../target/types/tic_tac_toe";
import { expect } from "chai";

async function play(
  program: Program<TicTacToe>,
  game,
  player,
  tile,
  expectedTurn,
  expectedGameState,
  expectedBoard
) {
  await program.methods
    .playGame(tile)
    .accounts({
      player: player.publicKey,
      game,
    })
    .signers(player instanceof (anchor.Wallet as any) ? [] : [player])
    .rpc()


  const gameState = await program.account.game.fetch(game)
  expect(gameState.turn).to.equal(expectedTurn)
  expect(gameState.state).to.eql(expectedGameState)
  expect(gameState.board).to.eql(expectedBoard)
}

describe("tic-tac-toe", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>;




  it('setup game!',async () => {

  let player_1 = (program.provider as anchor.AnchorProvider).wallet; // Not a Keypair
  let player_2 = anchor.web3.Keypair.generate();
  let gameKeypair  = anchor.web3.Keypair.generate();

    let trxHash = await program.methods
    .setupGame(player_2.publicKey)
    .accounts({
      game:gameKeypair.publicKey, // The gameKeypair public key
      player1:player_1.publicKey,
    })
    .signers([gameKeypair])
    .rpc()

    console.log(`The completed transaction sign is: ${trxHash}`);

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);

    expect(gameState.turn).to.equal(1);
    expect(gameState.players).to.eql([player_1.publicKey,player_2.publicKey])
    expect(gameState.state).to.eql({active: {}})
    expect(gameState.board).to.eql([
      [null,null,null],
      [null,null,null],
      [null,null,null],
    ])
  })

  it("player one winns!",async ()=>{
    const player_1 = (program.provider as anchor.AnchorProvider).wallet; // Not a Keypair
    const gameKeypair  = anchor.web3.Keypair.generate();
    const player_2 = anchor.web3.Keypair.generate();

    let trxHash = await program.methods
    .setupGame(player_2.publicKey)
    .accounts({
      game:gameKeypair.publicKey, // The gameKeypair public key
      player1:player_1.publicKey,
    })
    .signers([gameKeypair])
    .rpc()

    console.log(`The completed transaction sign is: ${trxHash}`);

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);

    expect(gameState.turn).to.equal(1);
    expect(gameState.players).to.eql([player_1.publicKey,player_2.publicKey])
    expect(gameState.state).to.eql({active: {}})
    expect(gameState.board).to.eql([
      [null,null,null],
      [null,null,null],
      [null,null,null],
    ])

    try {
          
 await play(
   program,
   gameKeypair.publicKey,
   player_1,
   {row: 0,column:0},
   2,
   { active: {} },
   [
     [{ x: {} }, null, null],
     [null, null, null],
     [null, null, null],
   ]
  )
    } catch (error) {
      console.log("The error is:",error);
    }

  })


});
