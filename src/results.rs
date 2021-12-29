// use super::fightclub::FIGHT_QUANTITY;
use super::fightclub::FightClub;
use super::soldiers::SoldierBase;
use yew::prelude::*;

/////////////////////
// Results to view //
/////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Datum {
   Nil,
   // Num(i32),
   Stat((f32, f32)),
   Percent(f32),
}

impl Default for Datum {
   fn default() -> Datum {
      Datum::Nil
   }
}

impl std::fmt::Display for Datum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         Datum::Nil => write!(f, "-"),
         // Datum::Num(x) => write!(f, "{}", x),
         Datum::Stat((m, s)) => write!(f, "{:.0}±{:.0}", m, s),
         Datum::Percent(x) => write!(f, "{:.1}%", x),
      }
   }
}

// #[derive(Copy, Clone, Debug, PartialEq, Default)]
// pub type Data = [[Datum; FIGHT_QUANTITY]; FIGHT_QUANTITY];

// Data are passed as &[Datum], indexed as if [[Datum]], for element Data[row][column]

pub fn results_table(data: &[Datum], club: &FightClub) -> Html {
   let headrow = club.iter().map(|s| html! { s.name_two_lines() } );
   let rows = club
      .iter()
      .zip(data.chunks(club.len()))
      .map(|(s, dd)| rrow(s, dd));

   html! {
        <>
         <tr>
            <th></th>
            { headrow.collect::<Html>() }
         </tr>
         { rows.collect::<Html>() }
        </>
   }
}

fn rrow(c: &SoldierBase, dd: &[Datum]) -> Html {
   let row = dd.iter().map(|d| html! { <td> { d } </td> });
   html! {
      <tr>
         { c.name_two_lines() }
         { row.collect::<Html>() }
      </tr>
   }
}

////////////////////////////
// Parallel fight results //
////////////////////////////

pub struct ParallelFight {
   // s1 v s2
   pub s1_aggressor_win_percent: f32,
   pub s1_aggressor_health_percent: f32,
   pub s1_aggressor_health_average: (f32, f32),
   pub s2_defender_health_percent: f32,
   pub s2_defender_health_average: (f32, f32),

   // s2 v s1
   pub s2_aggressor_win_percent: f32,
   pub s2_aggressor_health_percent: f32,
   pub s2_aggressor_health_average: (f32, f32),
   pub s1_defender_health_percent: f32,
   pub s1_defender_health_average: (f32, f32),

   // total
   pub s1_total_win_percent: f32,
   pub s1_total_health_percent: f32,
   pub s1_total_health_average: (f32, f32),
   pub s2_total_health_percent: f32,
   pub s2_total_health_average: (f32, f32),
}

impl ParallelFight {
   pub fn new() -> ParallelFight {
      ParallelFight {
         s1_aggressor_win_percent: 0.0,
         s1_aggressor_health_percent: 0.0,
         s1_aggressor_health_average: (0.0, 0.0),
         s2_defender_health_percent: 0.0,
         s2_defender_health_average: (0.0, 0.0),

         s2_aggressor_win_percent: 0.0,
         s2_aggressor_health_percent: 0.0,
         s2_aggressor_health_average: (0.0, 0.0),
         s1_defender_health_percent: 0.0,
         s1_defender_health_average: (0.0, 0.0),

         s1_total_win_percent: 0.0,
         s1_total_health_percent: 0.0,
         s1_total_health_average: (0.0, 0.0),
         s2_total_health_percent: 0.0,
         s2_total_health_average: (0.0, 0.0),
      }
   }

   // Fight s1 v s2
   pub fn s1a_wins(&mut self, winpc: f32) -> () {
      self.s1_aggressor_win_percent = winpc;
   }
   pub fn s1a_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s1_aggressor_health_percent = total_percent;
      self.s1_aggressor_health_average = (mean, stdev);
   }
   pub fn s2d_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s2_defender_health_percent = total_percent;
      self.s2_defender_health_average = (mean, stdev);
   }

   // Fight s2 v s1
   pub fn s2a_wins(&mut self, winpc: f32) -> () {
      self.s2_aggressor_win_percent = winpc;
   }
   pub fn s2a_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s2_aggressor_health_percent = total_percent;
      self.s2_aggressor_health_average = (mean, stdev);
   }
   pub fn s1d_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s1_defender_health_percent = total_percent;
      self.s1_defender_health_average = (mean, stdev);
   }

   // Fight s1 vs s2 on equal-footing
   pub fn s1_wins(&mut self, winpc: f32) -> () {
      self.s1_total_win_percent = winpc;
   }
   pub fn s1_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s1_total_health_percent = total_percent;
      self.s1_total_health_average = (mean, stdev);
   }
   pub fn s2_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s2_total_health_percent = total_percent;
      self.s2_total_health_average = (mean, stdev);
   }
}
