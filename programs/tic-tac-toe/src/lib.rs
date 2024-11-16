use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;
use anchor_lang::error_code;


declare_id!("8L2uEvLGGb1TFDwngQFwYfJBwaJXJCBA75eqXprFULob");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<GameSetup>,player_2:Pubkey) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let player_1 = ctx.accounts.player_1.key();
        let result = game.start([player_1,player_2]);
        result
    }

    pub fn play_game(ctx: Context<Play>, tile: Tile) -> Result<()> {
      let game = &mut ctx.accounts.game;
  
      require_keys_eq!(
          game.current_player(),
          ctx.accounts.player.key(),
          TicTacToeError::NotPlayersTurn
      );
  
      // Attempt to play the game with the provided tile
      game.play(&tile)
  }
  

}


// Initializing the Game account
#[derive(Accounts)]
pub struct GameSetup<'info> {
  #[account(
    init, 
    payer = player_1,
    space = 8 + Game::MAXIMUM_SIZE
  )]
  pub game: Account<'info,Game>,

 #[account(mut)]
 pub player_1:Signer<'info>,

 pub system_program: Program<'info,System>
}


#[derive(Accounts)]
pub struct Play<'info> {
  #[account(mut)]
  pub game: Account<'info,Game>,
  pub player: Signer<'info> // player needs to sign or someone else could play for the player.
}

#[account]
pub struct Game {
    players:[Pubkey;2],     // (32 * 2)
    turn: u8,                // 1 
    state:GameState,         // 32 + 1 
    board:[[Option<Sign>;3];3] // 9(tiles) * (1(option) + 1(Sign discriminant)) = 18 This is a 2D Array
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey }
}  

#[derive(
    AnchorSerialize, 
    AnchorDeserialize, 
    FromPrimitive, 
    ToPrimitive,
    Clone, 
    PartialEq, 
    Eq
)]
pub enum Sign {
    X = 0,
    O
}

#[derive(AnchorSerialize,AnchorDeserialize)]
pub struct Tile {
  row: u8,
  column:u8,
}


impl Game {
  pub const MAXIMUM_SIZE:usize = (32*2) + 1 + 9*(1+1) + 32 + 1;

  pub fn start(&mut self,player:[Pubkey;2]) -> Result<()> {
    require_eq!(self.turn,0,TicTacToeError::GameAlreadyStarted);
    self.players = player;
    self.turn = 1;
     Ok(())
  }

  pub fn is_active(&self) -> bool {
    self.state == GameState::Active
  }

  fn current_player_index(&self) -> usize {
    ((self.turn - 1)%2) as usize
  }

 pub fn current_player(&self) -> Pubkey {
    let index = self.current_player_index();
    self.players[index]
  }

  // Playing your move based on your tile coordinates
  pub fn play(&mut self,tile:&Tile) -> Result<()> {
    require!(self.is_active(), TicTacToeError::GameAlreadyOver);

    match tile {
        Tile { row:0..=2,column:0..=2 } => {
          match self.board[tile.row as usize][tile.column as usize] {
              Some(_)=>return Err(TicTacToeError::TileAlreadySet.into()),
              None=>{
                self.board[tile.row as usize][tile.column as usize] = Some(Sign::from_usize(self.current_player_index()).unwrap());
              }
          }
        },
        _ => return Err(TicTacToeError::TileOutOfBounds.into()),
    }

    self.update_state();
    if self.is_active() {
      self.turn+=1;
    }


    Ok(())
  }

  // Checking winner
  fn is_winnig_trio(&self, trio:[(usize,usize);3]) -> bool {
    let [first,second,third] = trio;
    let board = &self.board;
    board[first.0][first.1].is_some()
     &&board[first.0][first.1] == board[second.0][second.1]
     &&board[second.0][second.1] == board[third.0][third.1]
  }

  pub fn update_state(&mut self){
    
    for i in 0..=2 {
      // Lets check row trio-winning
      if self.is_winnig_trio([(i,0),(i,1),(i,2)]){
        self.state = GameState::Won { winner: self.current_player() };
        return;
      }
    }
    // Lets check the column
    for i in 0..=2 {
      if self.is_winnig_trio([(0,i),(1,i),(2,i)]){
        self.state = GameState::Won { winner: self.current_player() };
        return;
      }
    }

    // Lets check diagonal   
      if self.is_winnig_trio([(0,0),(1,1),(2,2)])
      || self.is_winnig_trio([(0,2),(1,1),(2,0)]) {
        self.state = GameState::Won { winner: self.current_player() };
        return;
    }

    // If this part of code is running, that would mean game is either active or tied

    // Lets check is game is tied
    for i in 0..=2 {
        for j in 0..=2 {
           if self.board[i][j].is_none(){
            self.state = GameState::Active;
            return;
           }
        }
    }

    // If this below code is working that would mean no one has won as well as all the places in board is filled that would mean it a tie
    self.state  = GameState::Tie;

  }
}


#[error_code]
pub enum TicTacToeError {
  TileOutOfBounds,
  TileAlreadySet,
  GameAlreadyOver,
  NotPlayersTurn,
  GameAlreadyStarted
}




// ++++++++++++++++++++++++++++++++++++++++++ Key Learnings ++++++++++++++++++++++++++++++++++++++++++
//
// - num_traits crate is commonly used to convert an integer into a corresponding variant of an enum. We use from_size() for converting number/integers to enum.
// - by using from_usize it will return Option with Some(_) if integer is valid or None if integer is not valid.
// - Anchor used borsh to serialize or deserialize the state/data of the account

// Space allocation for enum would be 1 byte for the discriminant and plus the biggest variant init 
// for example in state space would be 1(enum discriminant) + 32(Pubkey)

// U might think we can just use mem::size_of<Game>() ?
// 


// Key concepts for testing
// -Program: This is the deployed Solana smart contract,which contains methods like setupGame.
// -Provider: It sets up the connection to the Solana blockchain(Local network in this case).
// -Accounts: These are on-chain accounts storing data (like the state of the game).
// -Signers: These are the entities (Keypairs) that authorize trx's.

