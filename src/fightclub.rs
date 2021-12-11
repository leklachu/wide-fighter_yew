// Assorted fight routines

use super::fight;
use super::results;
use super::soldiers::*;

pub fn fightbmaxemax() -> results::ParallelFight {
   // let fight_result =
   fight::fight_parallel(
      Soldier::new_max(BARBARIAN),
      Soldier::new_max(EMPIRE),
      100000,
   )
}

// fn fightb0e0() -> () {
//    fight::fight_parallel(
//       Soldier::new(BARBARIAN),
//       String::from("Barbie"),
//       Soldier::new(EMPIRE),
//       String::from("Kenpire"),
//       100000,
//    );
// }
//
// fn fightbmaxemax() -> () {
//    fight::fight_parallel(
//       Soldier::new_max(BARBARIAN),
//       String::from("Big Barbarian"),
//       Soldier::new_max(EMPIRE),
//       String::from("Big Empire"),
//       100000,
//    );
// }
//
// fn fightbmaxamax() -> () {
//    fight::fight_parallel(
//       Soldier::new_max(BARBARIAN),
//       String::from("Big Barbarian"),
//       Soldier::new_max(ATLANTEAN),
//       String::from("Big Atlantean"),
//       100000,
//    );
// }
//
// fn fightbmaxfmax() -> () {
//    fight::fight_parallel(
//       Soldier::new_max(BARBARIAN),
//       String::from("Big Barbarian"),
//       Soldier::new_max(FRISIAN),
//       String::from("Big Frisian"),
//       100000,
//    );
// }
//
// fn fightbmaxzmax() -> () {
//    fight::fight_parallel(
//       Soldier::new_max(BARBARIAN),
//       String::from("Big Barbarian"),
//       Soldier::new_max(AMAZON),
//       String::from("Big Amazon"),
//       100000,
//    );
// }
//
// fn fightemaxamax() -> () {
//    fight::fight_parallel(
//       Soldier::new_max(EMPIRE),
//       String::from("Empire"),
//       Soldier::new_max(ATLANTEAN),
//       String::from("Atlantean"),
//       100000,
//    );
// }
