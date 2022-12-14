use crate::objects::PlayerTurn;

// this is an enum to store the state of a space on the gameboard
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpaceState { Empty, Player1, Player2 }

impl SpaceState {
    // the board space should start at empty
    pub fn new() -> SpaceState { Self::Empty }

    // this changes the state of the enum depending on who's turn it is and who claimed the space
    pub fn claim(&mut self, current_player_turn : &PlayerTurn) {
        *self = match current_player_turn {
            PlayerTurn::Player1 => { Self::Player1 }
            PlayerTurn::Player2 => { Self::Player2 }
        }
    }
}