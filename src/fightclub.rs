// A group of soldiers, and ways to make them fight. Actual fight simulations are in fight.rs

use super::fight;
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
   pub fn fight_all_parallel(
      &self,
      results_total_wins: &mut Data,
      results_total_health_remaining: &mut Data,
      results_total_health_stats: &mut Data,
      results_asym_wins: &mut Data,
      results_health_remaining_a: &mut Data,
      results_health_stats_a: &mut Data,
      results_health_remaining_d: &mut Data,
      results_health_stats_d: &mut Data,
      n: i32,
   ) {
      for (i, j) in self.iter_fights() {
         // fight i vs j
         let outcome = fight::fight_parallel(self.0[i].soldier(), self.0[j].soldier(), n);

         // Total Win Percent
         results_total_wins[i][j] = Datum::Percent(outcome.s1_total_win_percent);
         // if i != j {
         results_total_wins[j][i] = Datum::Percent(100.0 - outcome.s1_total_win_percent);
         // }

         // Total health remaining
         results_total_health_remaining[i][j] = Datum::Percent(outcome.s1_total_health_percent);
         results_total_health_remaining[j][i] = Datum::Percent(outcome.s2_total_health_percent);

         // Total health stats
         results_total_health_stats[i][j] = Datum::Stat(outcome.s1_total_health_average);
         results_total_health_stats[j][i] = Datum::Stat(outcome.s2_total_health_average);

         // Asymmetric win percent
         results_asym_wins[i][j] = Datum::Percent(outcome.s1_aggressor_win_percent);
         results_asym_wins[j][i] = Datum::Percent(outcome.s2_aggressor_win_percent);

         // Aggressors' Health remaining
         results_health_remaining_a[i][j] = Datum::Percent(outcome.s1_aggressor_health_percent);
         results_health_remaining_a[j][i] = Datum::Percent(outcome.s2_aggressor_health_percent);

         // Aggressors' stats A
         results_health_stats_a[i][j] = Datum::Stat(outcome.s1_aggressor_health_average);
         results_health_stats_a[j][i] = Datum::Stat(outcome.s2_aggressor_health_average);

         // Defenders' Health remaining B
         results_health_remaining_d[i][j] = Datum::Percent(outcome.s1_defender_health_percent);
         results_health_remaining_d[j][i] = Datum::Percent(outcome.s2_defender_health_percent);

         // Defenders' Health stats B
         results_health_stats_d[i][j] = Datum::Stat(outcome.s1_defender_health_average);
         results_health_stats_d[j][i] = Datum::Stat(outcome.s2_defender_health_average);
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
