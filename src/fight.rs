use super::soldiers;
// use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Simulate many parallel fights and return mean,stdev of remaining health for each team.
/// half the fights start with s1, half with s2, and wins and survivors' statistics are printed.
pub fn fight_parallel(
   s1: soldiers::Soldier,
   s1_name: String,
   s2: soldiers::Soldier,
   s2_name: String,
   n: i32,
) -> () {
   // ) -> (Vec<u16>, Vec<u16>, Vec<u16>, Vec<u16>) {
   // results only handles maximum health <= 65536. Fine for now so just check and panic
   if (s1.health > 65536) || (s2.health > 65536) {
      panic!();
   }

   let mut rng = SmallRng::from_entropy();

   // Fight A: s1 always initiates against s2

   // results:
   let mut survivors1a: Vec<u16> = Vec::new();
   let mut survivors2a: Vec<u16> = Vec::new();

   // fight_loop
   for _ in 0..n / 2 {
      let (s1_wins, remaining_health) = fight(s1, s2, &mut rng);
      if s1_wins {
         survivors1a.push(remaining_health as u16);
      } else {
         survivors2a.push(remaining_health as u16);
      }
   }

   // Fight B: s2 initiates
   // results:
   let mut survivors1b: Vec<u16> = Vec::new();
   let mut survivors2b: Vec<u16> = Vec::new();

   // fight_loop
   for _ in 0..n / 2 {
      let (s2_wins, remaining_health) = fight(s2, s1, &mut rng);
      if s2_wins {
         survivors2b.push(remaining_health as u16);
      } else {
         survivors1b.push(remaining_health as u16);
      }
   }

   // Print the results

   let stats = |survivors: &Vec<u16>, base_health: i32, name: &String| {
      let health_total: u64 = survivors.iter().map(|h| (*h as u64)).sum();
      let health_total_percent = health_total as f32 * 200.0 / n as f32 / base_health as f32;
      let health_mean: i32 =
         (survivors.iter().map(|h| (*h as u64)).sum::<u64>() / (survivors.len() as u64)) as i32;
      let health_stdev = ((survivors
         .iter()
         .map(|h| (*h as i64 - health_mean as i64).pow(2))
         .sum::<i64>() as f32)
         / (survivors.len() as f32 - 1.0))
         .sqrt();

      println!(
         "{} remaining health: army total {:.1}%, individual average {} +- {:.0}",
         name, health_total_percent, health_mean, health_stdev
      );
   };

   // because I've got lazy now. Should rework stats and stats_t together not duplicate
   let stats_t =
      |survivors_a: &Vec<u16>, survivors_b: &Vec<u16>, base_health: i32, name: &String| {
         let survivors = survivors_a.iter().chain(survivors_b.iter());
         let survivors_len = survivors_a.len() + survivors_b.len();
         let health_total: u64 = survivors.map(|h| (*h as u64)).sum();
         let health_total_percent = health_total as f32 * 100.0 / n as f32 / base_health as f32;
         let health_mean: i32 = (health_total / (survivors_len as u64)) as i32;
         let survivors = survivors_a.iter().chain(survivors_b.iter());
         let health_stdev = ((survivors
            .map(|h| (*h as i64 - health_mean as i64).pow(2))
            .sum::<i64>() as f32)
            / (survivors_len as f32 - 1.0))
            .sqrt();

         println!(
            "{} remaining health: army total {:.1}%, individual average {} +- {:.0}",
            name, health_total_percent, health_mean, health_stdev
         );
      };

   println!("\n------\n");

   let wins_a = survivors1a.len() as f32 * 200.0 / n as f32;
   println!(
      "{0} attacking {1}: {0} win {3} out of {4} = {2:.1}%",
      s1_name,
      s2_name,
      wins_a,
      survivors1a.len(),
      n / 2
   );

   stats(&survivors1a, s1.health, &s1_name);
   stats(&survivors2a, s2.health, &s2_name);

   println!("\n------\n");

   let wins_b = survivors2b.len() as f32 * 200.0 / n as f32;
   println!(
      "{0} attacking {1}: {0} win {3} out of {4} = {2:.1}%",
      s2_name,
      s1_name,
      wins_b,
      survivors1b.len(),
      n / 2
   );

   stats(&survivors1b, s1.health, &s1_name);
   stats(&survivors2b, s2.health, &s2_name);

   println!("\n------\n");

   let wins_total = (survivors1a.len() + survivors1b.len()) as f32 * 100.0 / n as f32;
   println!(
      "{0} and {1} fight: {0} win {3} out of {4} = {2:.1}%",
      s1_name,
      s2_name,
      wins_total,
      survivors1a.len() + survivors1b.len(),
      n
   );

   stats_t(&survivors1a, &survivors1b, s1.health, &s1_name);
   stats_t(&survivors2a, &survivors2b, s2.health, &s2_name);
}

// fight once, to the death
// returns true if s1 (aggressor) wins
#[inline]
fn fight(mut s1: soldiers::Soldier, mut s2: soldiers::Soldier, rng: &mut SmallRng) -> (bool, i32) {
   loop {
      // s1 swings
      if rng.gen_range(1..=100) > s2.evade {
         // it hits!
         let attack = rng.gen_range(s1.attack_min..=s1.attack_max);
         s2.health = s2.health - (attack * (100 - s2.defence) / 100);
         if s2.health <= 0 {
            return (true, s1.health);
         }
      }

      // s2 swings
      if rng.gen_range(1..=100) > s1.evade {
         // it hits!
         let attack = rng.gen_range(s2.attack_min..=s2.attack_max);
         s1.health = s1.health - (attack * (100 - s1.defence) / 100);
         if s1.health <= 0 {
            return (false, s2.health);
         }
      }
   }
}
