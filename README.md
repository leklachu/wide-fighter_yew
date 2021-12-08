Wide-fighter is a soldier fight simulator for widelands, similar to wl_soldiers
(https://github.com/einstein13/wl_soldiers).

Simulate many fights between soldiers of the different Widelands tribes to see
which wins more, under various conditions.

# Tribes:

- Barbarians
- Empire
- Atlanteans
- Frisians
- Amazons

# Methods:

- [TODO] Many individual fights from full health, recording wins/losses and remaining health.
  + [TODO] With either side starting fights, or alternating.
- [TODO] A line of fighters from each side: the front two fight, the loser dies, the winner stays on to fight the next from the loser's line.
  + [TODO] Optionally the winner leaves if he's on low heath (as he does in widelands)
- [TODO] Simulation of two full fortresses (12 soldiers), where low-health winners return to heal then return to the fight.
  + [TODO] Battle equidistant from fortresses or nearer one side.

# Questions:

- cf. https://www.widelands.org/wiki/SoldierLevels/ , when a player attacks, do
 their soldiers always start as the aggressor in each individual soldier
 battle?
- Are the stats up to date?
- are the calculations in-game int (as they appear to be) or float?
  + some rounding questions

# Other notes

- using 16-bit unsigned integers to store survivor health statistics for
  massively-parallel fights. Will panic if a soldier's starting health is above
  65536... lazy programming, I know. Current max appears to be 22,000

