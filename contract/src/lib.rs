use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen};
use serde::{Deserialize, Serialize};
use chess::{Game, Board, Square, ChessMove, Color};
use std::str::FromStr;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChessGame {
    pub player1: String,
    pub player2: String,
    pub board_fen: String,
}

type NearGames = near_sdk::collections::UnorderedMap<u64, ChessGame>;

impl Default for NearChess {
    fn default() -> Self {
        Self {
            games: near_sdk::collections::UnorderedMap::new(b"g".to_vec()),
            max_game_id: 0
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearChess {
    games: NearGames,
    max_game_id: u64,
}

#[near_bindgen]
impl NearChess {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            games: near_sdk::collections::UnorderedMap::new(b"g".to_vec()),
            max_game_id: 0,
        }
    }

    #[payable]
    pub fn start(&mut self, player1: String, player2: String) {
        self.max_game_id += 1;
        let game_id: u64 = self.max_game_id;

        let board = Board::default();
        let board_fen = format!("{}", board);
        self.games.insert(&game_id,
                          &ChessGame { player1, player2, board_fen },
        );
    }

    #[payable]
    pub fn make_move(&mut self, game_id: u64, square1: String, square2: String)  {
        let mut chess_game = self.games.get(&game_id).expect("Game doesn't exist");
        let board = Board::from_str(&chess_game.board_fen).unwrap();
        let chess_move = ChessMove::new(Square::from_string(square1).unwrap(), Square::from_string(square2).unwrap(), None);
        board.make_move_new(chess_move);
        chess_game.board_fen = format!("{}", board);
        self.games.insert(&game_id, &chess_game);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = NearChess::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_greeting("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = NearChess::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.get_greeting("francis.near".to_string())
        );
    }
}
