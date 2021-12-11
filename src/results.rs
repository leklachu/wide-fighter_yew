// use super::fightclub::FIGHT_QUANTITY;
use super::soldiers;
use super::soldiers::FIGHT_QUANTITY;
use yew::prelude::*;

/////////////////////
// Results to view //
/////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Datum {
   Nil,
   Num(i32),
   Stat(f32, f32),
   Percent(f32),
}

impl Default for Datum {
   fn default() -> Datum {
      // Datum::Num(0)
      Datum::Nil
   }
}

impl std::fmt::Display for Datum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         Datum::Nil => write!(f, "-"),
         Datum::Num(x) => write!(f, "{}", x),
         Datum::Stat(m, s) => write!(f, "{}±{}", m, s),
         Datum::Percent(x) => write!(f, "{:.1}%", x),
      }
   }
}

// #[derive(Copy, Clone, Debug, PartialEq, Default)]
pub type Data = [[Datum; FIGHT_QUANTITY]; FIGHT_QUANTITY];

pub fn results_table(title: String, data: Data) -> Html {
   html! {
      <div> <h2> {title} </h2>
      <table>
         <tr>
            <td></td>
            <th>{"Barbarians"}</th>
            <th>{"Empire"}</th>
            <th>{"Atlanteans"}</th>
            <th>{"Frisians"}</th>
            <th>{"Amazons"}</th>
            <th>{"custom"}</th>
         </tr><tr>
            <th> {"Barbarians"} </th>
            <td> {data[0][0]} </td>
            <td> {data[0][1]} </td>
            <td> {data[0][2]} </td>
            <td> {data[0][3]} </td>
            <td> {data[0][4]} </td>
            <td> {data[0][5]} </td>
         </tr><tr>
            <th> {"Empire"} </th>
            <td> {data[1][0]} </td>
            <td> {data[1][1]} </td>
            <td> {data[1][2]} </td>
            <td> {data[1][3]} </td>
            <td> {data[1][4]} </td>
            <td> {data[1][5]} </td>
         </tr><tr>
            <th> {"Atlanteans"} </th>
            <td> {data[2][0]} </td>
            <td> {data[2][1]} </td>
            <td> {data[2][2]} </td>
            <td> {data[2][3]} </td>
            <td> {data[2][4]} </td>
            <td> {data[2][5]} </td>
         </tr><tr>
            <th> {"Frisians"} </th>
            <td> {data[3][0]} </td>
            <td> {data[3][1]} </td>
            <td> {data[3][2]} </td>
            <td> {data[3][3]} </td>
            <td> {data[3][4]} </td>
            <td> {data[3][5]} </td>
         </tr><tr>
            <th> {"Amazons"} </th>
            <td> {data[4][0]} </td>
            <td> {data[4][1]} </td>
            <td> {data[4][2]} </td>
            <td> {data[4][3]} </td>
            <td> {data[4][4]} </td>
            <td> {data[4][5]} </td>
         </tr><tr>
            <th> {"custom"} </th>
            <td> {data[5][0]} </td>
            <td> {data[5][1]} </td>
            <td> {data[5][2]} </td>
            <td> {data[5][3]} </td>
            <td> {data[5][4]} </td>
            <td> {data[5][5]} </td>
         </tr>
      </table>
      </div>
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
