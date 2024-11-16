use crate::error::TicTacToeError;
use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;



#[account]
pub struct Game {
    players:[Pubkey;2],     // (32 * 2)
    turn: u8,                // 1 
    state:GameState,         // 32 + 1 
    board:[[Option<Sign>;3];3] // 9 * (1+1) = 18 This is a 2D Array
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
    X,
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
    self.state == GameState::Active;
  }

  fn current_player_index(&self) -> usize {
    ((self.turn - 1)%2) as usize
  }
}