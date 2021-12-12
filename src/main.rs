mod fight;
mod fightclub;
mod soldiers;

use yew::prelude::*;
mod results;
use results::Data;

enum Msg {
   Compute,
   // ChangeN(i32),
}

struct Model {
   link: ComponentLink<Self>,
   n: i32,
   fight_club: fightclub::FightClub,

   results_total_wins: Data,
   results_total_health_remaining: Data,
   results_total_health_stats: Data,

   results_asym_wins: Data,
   results_health_remaining_a: Data,
   results_health_stats_a: Data,
   results_health_remaining_d: Data,
   results_health_stats_d: Data,
}

impl Component for Model {
   type Message = Msg;
   type Properties = ();

   fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      Self {
         link,
         n: 100000,
         fight_club: fightclub::FightClub::new(),

         results_total_wins: results::Data::default(),
         results_total_health_remaining: results::Data::default(),
         results_total_health_stats: results::Data::default(),

         results_asym_wins: results::Data::default(),
         results_health_remaining_a: results::Data::default(),
         results_health_stats_a: results::Data::default(),
         results_health_remaining_d: results::Data::default(),
         results_health_stats_d: results::Data::default(),
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Compute => {
            // let bvse = fightclub::fightbmaxemax();
            // self.results1[0][1] = Datum::Percent(bvse.s1_total_win_percent);
            // self.results1[1][0] = Datum::Percent(100.0 - bvse.s1_total_win_percent);
            self.fight_club.fight_all_parallel(
               &mut self.results_total_wins,
               &mut self.results_total_health_remaining,
               &mut self.results_total_health_stats,
               &mut self.results_asym_wins,
               &mut self.results_health_remaining_a,
               &mut self.results_health_stats_a,
               &mut self.results_health_remaining_d,
               &mut self.results_health_stats_d,
               self.n,
            );
            true
         }
      }
   }

   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
   }

   fn view(&self) -> Html {
      let compute_go = self.link.callback(|_| Msg::Compute);
      // let change_n = self.link.callback(|e| {
      //    if
      //    ));

      html! {
         <>
         <p>{ "Fight simulator for Widelands soldiers, v0.2 or so." }</p>
         <p>{ "Soldiers' stats are for max level, from Widelands v1.0" }</p>
         <div>
            // <input type="number" pattern="[0-9]*" value={format!("{}",self.n)}/>
            <input type="number" value={format!("{}",self.n)}/>
            <p>{ "Each pair will fight " } { self.n } {" times" }</p>
            <button onclick={compute_go}>{ "Fight!" }</button>
         </div>
         <h1>{ "Equal fights" }</h1>
         <h2>{ "% win rate of row tribe vs column tribe" }</h2>
         { results::results_table(self.results_total_wins) }
         <h2>{ "% health remaining of row's team" }</h2>
         { results::results_table(self.results_total_health_remaining) }
         <h2>{ "average health remaining of row's surviving soldiers" }</h2>
         <p>{ "(for reference, max level Barbarians start at 22000)" }</p>
         { results::results_table(self.results_total_health_stats) }

         <h1>{ "Asymmetric" }</h1>
         <p>{ "(row tribe always hits first)" }</p>

         <h2>{ "% win rate of row tribe vs column tribe" }</h2>
         { results::results_table(self.results_asym_wins) }

         <h2>{ "% remaining health of row (aggressor)'s team" }</h2>
         { results::results_table(self.results_health_remaining_a) }
         <h2>{ "average remaining health of row (aggressor)'s survivors" }</h2>
         { results::results_table(self.results_health_stats_a) }

         <h2>{ "% remaining health of column (defender)'s team" }</h2>
         { results::results_table(self.results_health_remaining_d) }
         <h2>{ "average remaining health of column (defender)'s survivors" }</h2>
         { results::results_table(self.results_health_stats_d) }
         </>
      }
   }
}

fn main() {
   yew::start_app::<Model>();
}
