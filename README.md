# **Tic-Tac-Toe on Solana**

This project implements the classic Tic-Tac-Toe game as a Solana smart contract using the Anchor framework. Players can initialize a game, take turns playing moves, and check for game outcomes like a win, tie, or if the game is still active.

---

## **Features**

1. **Game Setup:** 
   - Initialize a new game account with two players. 
   - The game starts in an active state.

2. **Play Game:** 
   - Players take turns selecting tiles on a 3x3 board.
   - The game automatically checks for a winner or a tie after each move.

3. **Winner/Tie Determination:**
   - Detects winning rows, columns, or diagonals.
   - Identifies a tie if the board is full without a winner.

---

## **Installation**

### **Prerequisites**
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- Install Solana CLI from [Solana Docs](https://docs.solana.com/cli/install-solana-cli-tools)

### **Steps**
1. Clone the repository:
   ```bash
   git clone https://github.com/<your-username>/tic-tac-toe.git
   cd tic-tac-toe
2.Build the program:
   ```bash
   anchor build
