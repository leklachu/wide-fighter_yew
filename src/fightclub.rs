// A group of soldiers, and ways to make them fight. Actual fight simulations are in fight.rs

use super::fight;
use super::results;
use super::results::{Data, Datum};
use super::soldiers::*;

////////////////////////
// A Club of Fighters //
////////////////////////

#[derive(Debug, Copy, Clone)]
pub struct FightClub([SoldierBase; FIGHT_QUANTITY]);

impl FightClub {
   // make an iterator of all the fight combinations
   fn iter_fights(&self) -> FightIter {
      FightIter {
         i: 1,
         j: 0,
         // club: &self,
      }
   }

   // new club, default stuff
   pub fn new() -> Self {
      FightClub([
         SoldierBase::new(Tribe::Barbarian),
         SoldierBase::new(Tribe::Empire),
         SoldierBase::new(Tribe::Atlantean),
         SoldierBase::new(Tribe::Frisian),
         SoldierBase::new(Tribe::Amazon),
         SoldierBase::new(Tribe::Custom),
      ])
   }

   // parallel-fight them against each other, and enter results into table
   pub fn fight_all_parallel(&self, data: &mut Data, n: i32) {
      for (i, j) in self.iter_fights() {
         // fight i vs j
         let outcome = fight::fight_parallel(self.0[i].soldier(), self.0[j].soldier(), n);
         // put one way round into table
         data[i][j] = Datum::Percent(outcome.s1_total_win_percent);
         // if appropriate put other way round into table
         if i != j {
            data[j][i] = Datum::Percent(100.0 - outcome.s1_total_win_percent);
         }
      }
   }
}

// struct FightIter<'a> {
struct FightIter {
   i: usize,
   j: usize,
   // club: &'a FightClub,
}

// impl Iterator for FightIter<'_> {
impl Iterator for FightIter {
   type Item = (usize, usize);
   // type Item = (SoldierBase, SoldierBase);

   fn next(&mut self) -> Option<Self::Item> {
      if self.j < FIGHT_QUANTITY {
         self.j += 1;
         // Some((self.club.0[self.i - 1], self.club.0[self.j - 1]))
         Some((self.i - 1, self.j - 1))
      } else if self.i < FIGHT_QUANTITY {
         self.i += 1;
         self.j = self.i;
         // Some((self.club.0[self.i - 1], self.club.0[self.j - 1]))
         Some((self.i - 1, self.j - 1))
      } else {
         None
      }
   }
}

////////////////////////////////////////
// Fight each team against the other, //
// and put results in the given table //
////////////////////////////////////////

///////////////////
// old

pub fn fightbmaxemax() -> results::ParallelFight {
   // let fight_result =
   fight::fight_parallel(
      Soldier::new_max(BARBARIAN),
      Soldier::new_max(EMPIRE),
      100000,
   )
}
