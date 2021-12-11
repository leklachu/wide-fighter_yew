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
         Datum::Percent(x) => write!(f, "{}%", x),
      }
   }
}

// type DataA = [[Datum; NUM_TRIBES]; NUM_TRIBES];

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Data {
   pub b: Row,
   pub e: Row,
   pub a: Row,
   pub f: Row,
   pub z: Row,
   pub c: Row,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Row {
   pub b: Datum,
   pub e: Datum,
   pub a: Datum,
   pub f: Datum,
   pub z: Datum,
   pub c: Datum,
}

impl Data {
   pub fn test() -> Self {
      Data {
         b: Row::test(),
         e: Row::test(),
         a: Row::test(),
         f: Row::test(),
         z: Row::test(),
         c: Row::test(),
      }
   }
}

impl Row {
   fn test() -> Self {
      Row {
         b: Datum::Num(412),
         e: Datum::Percent(49.0),
         a: Datum::Stat(11.1, 4.23),
         f: Datum::Num(66),
         z: Datum::Num(5),
         c: Datum::Num(9),
      }
   }
}

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
            <td> {data.b.b} </td>
            <td> {data.b.e} </td>
            <td> {data.b.a} </td>
            <td> {data.b.f} </td>
            <td> {data.b.z} </td>
            <td> {data.b.c} </td>
         </tr><tr>
            <th> {"Empire"} </th>
            <td> {data.e.b} </td>
            <td> {data.e.e} </td>
            <td> {data.e.a} </td>
            <td> {data.e.f} </td>
            <td> {data.e.z} </td>
            <td> {data.e.c} </td>
         </tr><tr>
            <th> {"Atlanteans"} </th>
            <td> {data.a.b} </td>
            <td> {data.a.e} </td>
            <td> {data.a.a} </td>
            <td> {data.a.f} </td>
            <td> {data.a.z} </td>
            <td> {data.a.c} </td>
         </tr><tr>
            <th> {"Frisians"} </th>
            <td> {data.f.b} </td>
            <td> {data.f.e} </td>
            <td> {data.f.a} </td>
            <td> {data.f.f} </td>
            <td> {data.f.z} </td>
            <td> {data.f.c} </td>
         </tr><tr>
            <th> {"Amazons"} </th>
            <td> {data.z.b} </td>
            <td> {data.z.e} </td>
            <td> {data.z.a} </td>
            <td> {data.z.f} </td>
            <td> {data.z.z} </td>
            <td> {data.z.c} </td>
         </tr><tr>
            <th> {"custom"} </th>
            <td> {data.c.b} </td>
            <td> {data.c.e} </td>
            <td> {data.c.a} </td>
            <td> {data.c.f} </td>
            <td> {data.c.z} </td>
            <td> {data.c.c} </td>
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
