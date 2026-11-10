# Portal Interact

Open your inventory while you stand in a nether portal — and keep it open.
A Horizon native mod (mixin bundle), independent original work, MIT.

## What it does

Standing in a nether portal, the game refuses the inventory key (and a
ridden mount's or chest boat's) and closes any inventory you manage to open,
every tick, until the teleport takes you — because the dimension change
would destroy the screen anyway. This mod stands those two behaviors down
while its toggle is on:

- **E opens the inventory** from inside the portal, as it would anywhere
  else — including a horse's, llama's or chest boat's own screen.
- **The inventory stays open**: the portal's forced close is stood down
  while a container you opened is up. The purple transition overlay freezes
  for exactly as long as the screen is open, and resumes when you close it.

What the mod does **not** do: the teleport is the server's own, and it
happens on schedule regardless — stand in the portal with your inventory
open and you still travel; the dimension change then closes the screen the
game's own way. Block breaking and placing from inside the portal worked
already; this mod changes nothing about them.

The toggle is in **Options > Mods > Portal Interact** (`Enabled`, default
ON — installing it is asking for it), persisted in the mod's settings
storage, read fresh from disk every time a gate runs.

## How it fits together

Two HEAD hooks answer the game's own questions: `Game::in_portal_transition`
returns "no" while the mod asks (so the key gates pass), and
`Game::client_portal_effect` stands down for a tick when a container you
opened is up (so the forced close skips it). Both run on window Games only —
the integrated server's thread keeps its own truthful answers, and the
teleport is its business throughout. The policy lives in a pure-std runtime
module (3 tests); the helpers are four thin splices.

Verification recipe:

```sh
python3 -I tools/test.py           # fmt, clippy, the runtime's tests
python3 -I tools/horizon.py check  # staged cargo check on the pinned revision
python3 -I tools/horizon.py test   # staged runtime tests
python3 -I tools/package.py        # target/portal_interact-0.1.0.hnmod
```

## License

MIT — see [LICENSE](LICENSE).
