Wide-fighter is a soldier fight simulator for widelands, similar to wl_soldiers
(https://github.com/einstein13/wl_soldiers).

[TODO] update this readme!

Simulate many fights between soldiers of the different Widelands tribes to see
which wins more, under various conditions.

Now packaging as a webapp.
- [TODO] publish the compiled site somewhere!

The webapp currently has one table, and will run every tribe combination
(1/2(n)(n+1)=21 fights) through 100,000 parallel
fight-from-full-health-to-the-death, for each tribe's max-level soldier. The
results are the % wins of the row_label tribe against the column_label tribe.
Soldiers initiate (take the first hit) both ways round equally (i.e. one stat =
100% - other stat).

# Tribes:

- Barbarians
- Empire
- Atlanteans
- Frisians
- Amazons
- custom (currently initialises =Amazons)

# Methods:

- Many individual fights from full health, recording wins/losses and remaining health.
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

