# Sol

A programming language for custom content.

> [!WARNING]
>
> Sol is in early development and should not be used for real projects yet.
> Feel free to become part of the conversation, though, and help me determine
> its future :-)

Sol brings modern programming language design to reinvent how you create and maintain game assets. Its syntax is *beautiful* **AND** productive **AND** type-safe. It takes inspiration from LUA, Rust and GDScript to create an amazing development experience aesthetically and organizationally.

Sol is also free, open-source, engine-agnostic and not owned by a company. After you author an asset, you really *own* it and can easily port it to other engines.

Each `.sol` file is an "asset" which can be, for example, an item...
```sol
-- weapons/iron_sword.sol

using game.models.*
using game.sprites.icons

impl ItemKind
  label = "Iron Sword"
  durability = 4
  icon = icons.iron_sword

  action on_use(target: Character)
    target.try_equip_weapon(weapon: self)
  end
end
```
...or a scene...
```sol
using game.chars.Jude

--todo Add portraits to those dialogues;
def "Main Scene"
  [Echo]
  - Jude, could you please move a little?

  [Jude]
  - nah

  [Echo]
  - I'll hit you in the head with a pan.

  [Jude]
  - *moves*

  move(who: Jude, by: vec2(1, 0))
end
```
...or even a level.
```sol
...impl Room

def player_spawn : Marker
  position = vec2(-10, -3)
  facing = DOWN
end

def chest1 : Chest
  position = vec2(0, 0)
end
```
Anything that is your games' "content" and not the core mechanic you have to code against the engine, you can put in .sol files :-)

The scripting capability of sol looks simple (courtesy of the LUA-esque syntax) but is far from minimal. Sol *is* a programming language, and it is fast as its VM is built with Rust.

## Features

- [x] Just text: Can be checked into version control easily;
- [x] Engine agnostic: Your assets will not be 'Godot' assets or 'Unity' assets, but *your* assets, wherever you take them;
- [x] Incredibly fast iteration cycles;
- [x] Easy but powerful dialogues, in a syntax so simple, you'll probably use it for sketching the roughs even;
```sol
[Jude smiling]
- Hello, there!
- I found one item here and I want to give it to you, {player.name}!
```
- [x] Thorough type safety, following the slogan "Make Invalid States Unrepresentable" to eliminate entire classes of bugs common in game dev;
```sol
model Item
  field name : text
  field unbreakable : truth
  field icon : Image
  if not unbreakable then
    field durability : nat
  end
end
```
- [ ] Dependency management with `sol add <dependency_uri>`, to quickly install asset packs or frameworks into your project;
- [ ] Interoperability with external resources (.png, .gltf, .aseprite) through the usage of `extensions`;
- [x] Possibility of integration with engines;
- [ ] Refactoring tools like the LSP and `sol migrate`;

## Installing

You can compile from source as a rust project, or install from crates.io via cargo.

```bash
cargo install sol-lang
```

Futurely, I want to provide integration plugins for a few engines. In those cases, the plugin will take care of building `sol`.

Sol LSP is being worked on, as well as a [Zed](https://zed.dev) extension.

## Contributing

Not yet available.

------

<sup>\[1\]</sup> - "Make Invalid States Unrepresentable."
