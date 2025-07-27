#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use ink::storage::{traits::SpreadLayout, traits::PackedLayout, traits::StorageLayout};
use scale::{Encode, Decode};

#[ink::contract]
mod lottery_core {
    use super::*;

    #[derive(scale::Encode, scale::Decode, Clone, Default, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Round {
        pub id: u32,
        pub participants: Vec<AccountId>,
        pub winner: Option<AccountId>,
    }

    #[ink(storage)]
    pub struct LotteryCore {
        rounds: Mapping<u32, Round>,
        current_round: u32,
    }

    impl LotteryCore {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                rounds: Mapping::default(),
                current_round: 0,
            }
        }

        #[ink(message)]
        pub fn join_lottery(&mut self) {
            let caller = self.env().caller();
            let mut round = self.rounds.get(self.current_round).unwrap_or_default();
            round.id = self.current_round;
            round.participants.push(caller);
            self.rounds.insert(self.current_round, &round);
        }

        #[ink(message)]
        pub fn draw_winner(&mut self) -> Option<AccountId> {
            let mut round = self.rounds.get(self.current_round).unwrap_or_default();
            if round.participants.is_empty() {
                return None;
            }
            let index = (self.env().block_timestamp() as usize) % round.participants.len();
            let winner = round.participants[index];
            round.winner = Some(winner);
            self.rounds.insert(self.current_round, &round);
            self.current_round += 1;
            Some(winner)
        }

        #[ink(message)]
        pub fn get_round(&self, round_id: u32) -> Option<Round> {
            self.rounds.get(round_id)
        }
    }
}