mod fight;
mod fightclub;
mod soldiers;
use soldiers::*;

use yew::prelude::*;
mod results;
use results::{Data, Datum};

enum Msg {
   Compute,
}

struct Model {
   link: ComponentLink<Self>,
   n: i32,
   results1: Data,
   fight_club: fightclub::FightClub,
}

impl Component for Model {
   type Message = Msg;
   type Properties = ();

   fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      Self {
         link,
         n: 100000,
         results1: results::Data::default(),
         fight_club: fightclub::FightClub::new(),
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Compute => {
            // let bvse = fightclub::fightbmaxemax();
            // self.results1[0][1] = Datum::Percent(bvse.s1_total_win_percent);
            // self.results1[1][0] = Datum::Percent(100.0 - bvse.s1_total_win_percent);
            self
               .fight_club
               .fight_all_parallel(&mut self.results1, self.n);
            true
         }
      }
   }

   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
   }

   fn view(&self) -> Html {
      let onclick = self.link.callback(|_| Msg::Compute);
      let title1 = String::from("% win rate of row tribe vs column tribe");
      html! {
         <>
         <div>
            <button onclick={onclick}>{ "Fight!" }</button>
            <p>{ "Fighting " } { self.n } {" times" }</p>
         </div>
         { results::results_table(title1, self.results1) }
         </>
      }
   }
}

fn main() {
   yew::start_app::<Model>();
}
