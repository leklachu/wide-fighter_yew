// A group of soldiers, and ways to make them fight. Actual fight simulations are in fight.rs

use super::fight;
use super::results::Datum;
use super::soldiers::*;

////////////////////////
// A Club of Fighters //
////////////////////////

// #[derive(Debug, Copy, Clone)]
// pub struct FightClub([SoldierBase; FIGHT_QUANTITY]);
#[derive(Debug, Clone)]
pub struct FightClub(Vec<SoldierBase>);

impl FightClub {
   // make an iterator of all the fight combinations
   fn iter_fights(&self) -> FightIter {
      FightIter {
         i: 1,
         j: 0,
         n: self.0.len(),
      }
   }

   // new club, default stuff
   pub fn new() -> Self {
      FightClub(vec![
         SoldierBase::new(Tribe::Barbarian),
         SoldierBase::new(Tribe::Empire),
         SoldierBase::new(Tribe::Atlantean),
         SoldierBase::new(Tribe::Frisian),
         SoldierBase::new(Tribe::Amazon),
         // SoldierBase::new(Tribe::Custom),
      ])
   }

   // parallel-fight them against each other, and enter results into table
   pub fn fight_all_parallel(
      &self,
      results_total_wins: &mut [Datum],
      results_total_health_remaining: &mut [Datum],
      results_total_health_stats: &mut [Datum],
      results_asym_wins: &mut [Datum],
      results_health_remaining_a: &mut [Datum],
      results_health_stats_a: &mut [Datum],
      results_health_remaining_d: &mut [Datum],
      results_health_stats_d: &mut [Datum],
      n: i32,
   ) {
      for (i, j) in self.iter_fights() {
         let ij = self.0.len() * i + j;
         let ji = self.0.len() * j + i;

         // fight i vs j
         let outcome = fight::fight_parallel(self.0[i].soldier(), self.0[j].soldier(), n);

         // Total Win Percent
         results_total_wins[ij] = Datum::Percent(outcome.s1_total_win_percent);
         // if i != j {
         results_total_wins[ji] = Datum::Percent(100.0 - outcome.s1_total_win_percent);
         // }

         // Total health remaining
         results_total_health_remaining[ij] = Datum::Percent(outcome.s1_total_health_percent);
         results_total_health_remaining[ji] = Datum::Percent(outcome.s2_total_health_percent);

         // Total health stats
         results_total_health_stats[ij] = Datum::Stat(outcome.s1_total_health_average);
         results_total_health_stats[ji] = Datum::Stat(outcome.s2_total_health_average);

         // Asymmetric win percent
         results_asym_wins[ij] = Datum::Percent(outcome.s1_aggressor_win_percent);
         results_asym_wins[ji] = Datum::Percent(outcome.s2_aggressor_win_percent);

         // Aggressors' Health remaining
         results_health_remaining_a[ij] = Datum::Percent(outcome.s1_aggressor_health_percent);
         results_health_remaining_a[ji] = Datum::Percent(outcome.s2_aggressor_health_percent);

         // Aggressors' stats A
         results_health_stats_a[ij] = Datum::Stat(outcome.s1_aggressor_health_average);
         results_health_stats_a[ji] = Datum::Stat(outcome.s2_aggressor_health_average);

         // Defenders' Health remaining B
         results_health_remaining_d[ij] = Datum::Percent(outcome.s1_defender_health_percent);
         results_health_remaining_d[ji] = Datum::Percent(outcome.s2_defender_health_percent);

         // Defenders' Health stats B
         results_health_stats_d[ij] = Datum::Stat(outcome.s1_defender_health_average);
         results_health_stats_d[ji] = Datum::Stat(outcome.s2_defender_health_average);
      }
   }

   pub fn iter(&self) -> std::slice::Iter<'_, SoldierBase> {
      self.0.iter()
   }
   pub fn len(&self) -> usize {
      self.0.len()
   }
   pub fn remove(&mut self, i: usize) {
      self.0.remove(i);
   }
   pub fn add_soldier(&mut self, t: Tribe) {
      self.0.push(SoldierBase::new(t));
   }
   pub fn lvl_up(&mut self, i: usize, param: Param) {
      match param {
         Param::Attack => {
            if self.0[i].levels.attack < self.0[i].params.attack_lvls as u8 {
               self.0[i].levels.attack += 1;
            }
         }
         Param::Defence => {
            if self.0[i].levels.defence < self.0[i].params.defence_lvls as u8 {
               self.0[i].levels.defence += 1;
            }
         }
         Param::Health => {
            if self.0[i].levels.health < self.0[i].params.health_lvls as u8 {
               self.0[i].levels.health += 1;
            }
         }
         Param::Evade => {
            if self.0[i].levels.evade < self.0[i].params.evade_lvls as u8 {
               self.0[i].levels.evade += 1;
            }
         }
      }
   }
   pub fn lvl_down(&mut self, i: usize, param: Param) {
      match param {
         Param::Attack => {
            if self.0[i].levels.attack > 0 {
               self.0[i].levels.attack -= 1;
            }
         }
         Param::Defence => {
            if self.0[i].levels.defence > 0 {
               self.0[i].levels.defence -= 1;
            }
         }
         Param::Health => {
            if self.0[i].levels.health > 0 {
               self.0[i].levels.health -= 1;
            }
         }
         Param::Evade => {
            if self.0[i].levels.evade > 0 {
               self.0[i].levels.evade -= 1;
            }
         }
      }
   }
}

struct FightIter {
   i: usize,
   j: usize,
   n: usize,
}

impl Iterator for FightIter {
   type Item = (usize, usize);

   fn next(&mut self) -> Option<Self::Item> {
      if self.j < self.n {
         self.j += 1;
         Some((self.i - 1, self.j - 1))
      } else if self.i < self.n {
         self.i += 1;
         self.j = self.i;
         Some((self.i - 1, self.j - 1))
      } else {
         None
      }
   }
}
