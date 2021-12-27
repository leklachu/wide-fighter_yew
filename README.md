Wide-fighter is a soldier fight simulator for widelands, similar to wl_soldiers
(https://github.com/einstein13/wl_soldiers).

Simulate many fights between soldiers of the different Widelands tribes to see
which wins more, under various conditions.

Packaged as a webapp and hosted at https://wide-fighter.netlify.app/

Currently wide-fighter will fight 100,000 soldiers in parallel for every tribe combination. It records the survivors and calculates:

- Win percentage of each tribe
- Total % health remaining (sum of survivors' health / team total starting health)
- Average remaining health of each survivor (ignoring dead soldiers.

Stats are calculated for each side always starting fights, and for equal
starting / not starting.

# Building / Contributing

To run this just go to https://wide-fighter.netlify.app/ - it runs decently fast in-browser.

To build, you need

- rust `https://www.rust-lang.org/learn/get-started`
- web assembly target for rust
  - `rustup target add wasm32-unknown-unknown`
- trunk `https://trunkrs.dev/`
  - `cargo install --locked trunk`
- This source code
  - `git clone https://github.com/leklachu/wide-fighter.git`

then just build with `trunk build`, test with `trunk serve` and point your browser to `localhost:8080`, and build a fast version with `trunk build --release`

The web frontend is built with `https://yew.rs/` v0.19. (Trunk will
automatically fetch this when building.) It's pretty easy to make a basic app,
but apparently tricker to read any user input beyond buttons.

Contributions are welcome, with pull requests, patches / whatever, and I'll try not to be too slow incorporating them. The code is released to the public domain; if that's a problem let me know and I can change the licence.


# Tribes:

- Barbarians
- Empire
- Atlanteans
- Frisians
- Amazons
- (internal 'custom' tribe, currently initialised to Amazons, currently unused)

# [TODO]

- Enable customising a tribe's stats
- Make the web app pretty
- Further simulation options (see below)

# Methods:

- Many individual fights from full health, recording wins/losses and remaining health.
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

# Other notes

- using 16-bit unsigned integers to store survivor health statistics for
  massively-parallel fights. Will panic if a soldier's starting health is above
  65536... lazy programming, I know. Current max appears to be 22,000
- stored numbers are generally inconsistent. e.g. number of fights is currently
  i32, and can be made -ve on the app by increasing too many times!

