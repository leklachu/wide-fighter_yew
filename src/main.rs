mod fight;
mod fightclub;
mod soldiers;

// use web_sys::HtmlInputElement;
use yew::prelude::*;
// use yew::{events::Event, InputEvent, TargetCast};
mod results;
use soldiers::{Param, Tribe};

enum Msg {
   Compute,
   NPlus,
   NMinus,
   RemoveSoldier(usize),
   AddSoldier(Tribe),
   LvlUp(usize, Param),
   LvlDown(usize, Param),
}

struct Model {
   n: i32,
   // remember the computed fight club for results display, while the display one can be changed
   // for a new compute
   fight_club_computed: fightclub::FightClub,
   fight_club_ui: fightclub::FightClub,

   results_total_wins: Vec<results::Datum>,
   results_total_health_remaining: Vec<results::Datum>,
   results_total_health_stats: Vec<results::Datum>,

   results_asym_wins: Vec<results::Datum>,
   results_health_remaining_a: Vec<results::Datum>,
   results_health_stats_a: Vec<results::Datum>,
   results_health_remaining_d: Vec<results::Datum>,
   results_health_stats_d: Vec<results::Datum>,
}

impl Component for Model {
   type Message = Msg;
   type Properties = ();

   // fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
   fn create(_ctx: &Context<Self>) -> Self {
      let fc = fightclub::FightClub::new();
      let len = fc.len() * fc.len();
      Self {
         n: 100000,
         fight_club_computed: fc.clone(),
         fight_club_ui: fc,

         results_total_wins: vec![results::Datum::Nil; len],
         results_total_health_remaining: vec![results::Datum::Nil; len],
         results_total_health_stats: vec![results::Datum::Nil; len],

         results_asym_wins: vec![results::Datum::Nil; len],
         results_health_remaining_a: vec![results::Datum::Nil; len],
         results_health_stats_a: vec![results::Datum::Nil; len],
         results_health_remaining_d: vec![results::Datum::Nil; len],
         results_health_stats_d: vec![results::Datum::Nil; len],
      }
   }

   fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Msg::Compute => {
            if self.fight_club_ui.len() == 0 {
               return false;
            }

            // rebuild the results tables if size has changed
            if self.fight_club_computed.len() != self.fight_club_ui.len() {
               for table in [
                  &mut self.results_total_wins,
                  &mut self.results_total_health_remaining,
                  &mut self.results_total_health_stats,
                  &mut self.results_asym_wins,
                  &mut self.results_health_remaining_a,
                  &mut self.results_health_stats_a,
                  &mut self.results_health_remaining_d,
                  &mut self.results_health_stats_d,
               ] {
                  table.resize(
                     self.fight_club_ui.len() * self.fight_club_ui.len(),
                     results::Datum::Nil,
                  );
               }
            }

            // clone the displayed fight club into the one to be computed
            self.fight_club_computed = self.fight_club_ui.clone();

            self.fight_club_computed.fight_all_parallel(
               &mut self.results_total_wins.as_mut_slice(),
               &mut self.results_total_health_remaining.as_mut_slice(),
               &mut self.results_total_health_stats.as_mut_slice(),
               &mut self.results_asym_wins.as_mut_slice(),
               &mut self.results_health_remaining_a.as_mut_slice(),
               &mut self.results_health_stats_a.as_mut_slice(),
               &mut self.results_health_remaining_d.as_mut_slice(),
               &mut self.results_health_stats_d.as_mut_slice(),
               self.n,
            );
            true
         }
         Msg::NPlus => {
            self.n = self.n * 2;
            true
         }
         Msg::NMinus => {
            self.n = self.n / 2;
            true
         }
         Msg::RemoveSoldier(i) => {
            self.fight_club_ui.remove(i);
            true
         }
         Msg::AddSoldier(t) => {
            self.fight_club_ui.add_soldier(t);
            true
         }
         Msg::LvlUp(i, param) => {
            self.fight_club_ui.lvl_up(i, param);
            true
         }
         Msg::LvlDown(i, param) => {
            self.fight_club_ui.lvl_down(i, param);
            true
         }
      }
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      let link = ctx.link();
      let compute_go = link.callback(|_| Msg::Compute);
      let n_plus = link.callback(|_| Msg::NPlus);
      let n_minus = link.callback(|_| Msg::NMinus);

      let add_barbarian = link.callback(|_| Msg::AddSoldier(Tribe::Barbarian));
      let add_empire = link.callback(|_| Msg::AddSoldier(Tribe::Empire));
      let add_atlantean = link.callback(|_| Msg::AddSoldier(Tribe::Atlantean));
      let add_frisian = link.callback(|_| Msg::AddSoldier(Tribe::Frisian));
      let add_amazon = link.callback(|_| Msg::AddSoldier(Tribe::Amazon));

      let fc = &self.fight_club_computed;

      let fc_display = self
         .fight_club_ui
         .iter()
         .zip(0..)
         .map(|(s, i): (&soldiers::SoldierBase, usize)| soldier_item(s, i, &link.clone()));

      html! {
        <>
         <header>
         <h1>{ "Fight simulator for Widelands soldiers, v0.3 or so." }</h1>
         <p>{ "Soldiers' stats are for max level, from Widelands v1.0" }</p>
         </header>

         <article class="fight_club">
         <h2>{ "The soldiers to fight" }</h2>
         // The customisable fight club
         <table class="fight_club"><tr>
            { fc_display.collect::<Html>() }
            <td><table>
               <tr><td><button onclick={add_barbarian}>{ "+ Barbarian" }</button></td></tr>
               <tr><td><button onclick={add_empire}>{ "+ Empire" }</button></td></tr>
               <tr><td><button onclick={add_atlantean}>{ "+ Atlantean" }</button></td></tr>
               <tr><td><button onclick={add_frisian}>{ "+ Frisian" }</button></td></tr>
               <tr><td><button onclick={add_amazon}>{ "+ Amazon" }</button></td></tr>
            </table></td>
         </tr></table>

         // Fighting how many times
         <div>
            <p>{ "Each pair will fight " }
               <button onclick={n_minus}>{ "-" }</button> { self.n }
               <button onclick={n_plus}>{ "+" }</button> {" times" }
            </p>
            <button onclick={compute_go}>{ "Fight!" }</button>
         </div>
         </article>

         // Results
         <article class="result-equal">
         <h1>{ "Equal fights" }</h1>
         <section>

         <table>
            <caption>{ "% win rate of row tribe vs column tribe" }</caption>
            { results::results_table(self.results_total_wins.as_slice(), fc) }
         </table>

         <table>
             <caption>{ "% health remaining of row's team" }</caption>
             { results::results_table(self.results_total_health_remaining.as_slice(), fc) }
         </table>

         <table>
            <caption>
                { "average health remaining of row's surviving soldiers" }<br/>
                <span class="small">{ "(for reference, max level Barbarians start at 22000)" }</span>
            </caption>
         { results::results_table(self.results_total_health_stats.as_slice(), fc) }
         </table>
         </section>
         </article>

         <article class="result-asym">
         <h1>{ "Asymmetric" }</h1>
         <p>{ "(row tribe always hits first)" }</p>

         <section>

         <table>
            <caption>
                { "% win rate of row tribe vs column tribe" }
            </caption>
            { results::results_table(self.results_asym_wins.as_slice(), fc) }
         </table>

         <table>
            <caption>
                { "% remaining health of row (aggressor)'s team" }
            </caption>
            { results::results_table(self.results_health_remaining_a.as_slice(), fc) }
         </table>

         <table>
            <caption>
                { "average remaining health of row (aggressor)'s survivors" }
            </caption>
            { results::results_table(self.results_health_stats_a.as_slice(), fc) }
         </table>

         <table>
            <caption>
                { "% remaining health of column (defender)'s team" }
            </caption>
            { results::results_table(self.results_health_remaining_d.as_slice(), fc) }
         </table>

         <table>
            <caption>
                { "average remaining health of column (defender)'s survivors" }
            </caption>
            { results::results_table(self.results_health_stats_d.as_slice(), fc) }
         </table>
         </section>
         </article>
         </>
      }
   }
}

fn soldier_item(s: &soldiers::SoldierBase, i: usize, link: &yew::html::Scope<Model>) -> Html {
   let remove = link.callback(move |_| Msg::RemoveSoldier(i));

   let attack_up = link.callback(move |_| Msg::LvlUp(i, Param::Attack));
   let attack_down = link.callback(move |_| Msg::LvlDown(i, Param::Attack));

   let defence_up = link.callback(move |_| Msg::LvlUp(i, Param::Defence));
   let defence_down = link.callback(move |_| Msg::LvlDown(i, Param::Defence));

   let health_up = link.callback(move |_| Msg::LvlUp(i, Param::Health));
   let health_down = link.callback(move |_| Msg::LvlDown(i, Param::Health));

   let evade_up = link.callback(move |_| Msg::LvlUp(i, Param::Evade));
   let evade_down = link.callback(move |_| Msg::LvlDown(i, Param::Evade));

   html! {
      <td><table>
         <tr><td>{ s.name_long() }</td></tr>
         <tr><td>{ "Base attack: " }{s.params.attack_base}{"-"}{s.params.attack_maxm}</td></tr>
         // <tr><td>{ "+ per level: " }{s.params.attack_incr}</td></tr>
         <tr><td>
            { "+" }{s.params.attack_incr}{" per lvl, "}
            <button onclick={attack_down}>{ "-" }</button>
            {s.levels.attack}{"/"}{s.params.attack_lvls}
            <button onclick={attack_up}>{ "+" }</button>
         </td></tr>

         <tr><td>{ "Base defence: " }{s.params.defence_base}</td></tr>
         // <tr><td>{ "+ per level: " }{s.params.defence_incr}</td></tr>
         <tr><td>
            { "+" }{s.params.defence_incr}{" per lvl, "}
            <button onclick={defence_down}>{ "-" }</button>
            {s.levels.defence}{"/"}{s.params.defence_lvls}
            <button onclick={defence_up}>{ "+" }</button>
         </td></tr>

         <tr><td>{ "Base health: " }{s.params.health_base}</td></tr>
         // <tr><td>{ "+ per level: " }{s.params.health_incr}</td></tr>
         <tr><td>
            { "+" }{s.params.health_incr}{" per lvl, "}
            <button onclick={health_down}>{ "-" }</button>
            {s.levels.health}{"/"}{s.params.health_lvls}
            <button onclick={health_up}>{ "+" }</button>
         </td></tr>

         <tr><td>{ "Base evade: " }{s.params.evade_base}</td></tr>
         // <tr><td>{ "+ per level: " }{s.params.evade_incr}</td></tr>
         <tr><td>
            { "+" }{s.params.evade_incr}{" per lvl, "}
            <button onclick={evade_down}>{ "-" }</button>
            {s.levels.evade}{"/"}{s.params.evade_lvls}
            <button onclick={evade_up}>{ "+" }</button>
         </td></tr>


         <tr><td><button onclick={remove}>{ "remove" }</button></td></tr>
      </table></td>
   }
}

fn main() {
   yew::start_app::<Model>();
}
