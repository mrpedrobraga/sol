# Sol

A programming language for custom apps/game content.

> [!WARNING]
>
> Sol is in early development and should not be used for real projects yet.
> Feel free to become part of the conversation, though, and help me determine
> its future :-)

Sol is an implementation-agnostic format for creating arbitrary game data. Think items, enemies, rooms, etc.

By defining your assets in SOL, you can use them in Rust, C, Godot, Unity, etc. SOL files are plain text files, so they can be version-controlled with git, and manually authored if necessary.

## Features

> A check means it is already implemented.

- [x] Just text: Can be checked into version control easily;
- [x] Engine agnostic: Your assets will not be 'Godot' assets or 'Unity' assets, but *your* assets, wherever you take them;
- [x] Incredibly fast iteration cycles;
- [x] Easy but powerful dialogues, in a syntax so simple, you'll probably use it for sketching the roughs even;

```js
[Jude smiling]
- Hello, {PLAYER_NAME}!
- It is dangerous to go alone... take this!

Inventory.acquire_item(kind: IronSword, amount: 1)
```
- [x] Type safe — create your own models and enforce asset correctness;

```js
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
- [ ] Easy refactoring and versioning using `sol migrate`;


## Syntax (for nerds)

Each `.sol` file is a "module." This module is populated with "fields":
```lua
-- weapons/iron_sword.sol
@model Item

-- Dependencies on other modules!
using Icons

-- Static (Constant) fields
display_name = "Iron Sword"
durability = 4
icon = Icons.iron_sword

-- Fields can be procedures!
action on_use(target: Character)
    target.try_equip_weapon(weapon: self)
end
```

Sol files can be used to specify not just "resources" like items, but also scenes.

```lua
using Chars.(Echo, Jude)

--todo Add portraits to those dialogues;
scene main_scene
  [Echo]
  - Jude, could you please move a little?

  [Jude]
  - nah

  [Echo]
  - Imma hit you in the head with a pan.

  [Jude]
  - *moves*

  Jude.move(by: [1, 0])
end
```

...or levels layouts.

```lua
@model Room

def player_spawn as Marker
  position = [-10, -3]
  facing = DOWN
end

def chest1 as Chest
  position = [0, 0]
end
```

## Installing

You can compile from source as a rust project, or install from crates.io via cargo.

```bash
cargo install sol-lang
```

Integration with engines, such as Godot, are not yet available. Sol LSP is being worked on, as well as a [Zed](https://zed.dev) extension.

## Contributing

Not yet available.