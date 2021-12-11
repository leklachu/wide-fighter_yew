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
   value: i64,
   results1: Data,
}

impl Component for Model {
   type Message = Msg;
   type Properties = ();

   fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      Self {
         link,
         value: 0,
         results1: Data::test(),
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Compute => {
            self.value += 1;
            let bvse = fightclub::fightbmaxemax();
            self.results1.b.e = Datum::Percent(bvse.s1_total_win_percent);
            self.results1.e.b = Datum::Percent(100.0 - bvse.s1_total_win_percent);
            true
         }
      }
   }

   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
   }

   fn view(&self) -> Html {
      let onclick = self.link.callback(|_| Msg::Compute);
      let title1 = String::from("Fight 1");
      html! {
         <>
         <div>
            <button onclick={onclick}>{ "Fight!" }</button>
            <p>{ "Fights are, " } { self.value }</p>
         </div>
         { results::results_table(title1, self.results1) }
         </>
      }
   }
}

fn main() {
   yew::start_app::<Model>();
}
