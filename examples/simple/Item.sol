-- An item is something that can be stored in your
-- inventory and used.
model Item
    name: Text
    description: Text
    attack_damage: Nat
    durability: dynamic Nat
    -- Called whenever the item is used.
    on_use: Action(Character, Character)

    destroyed: Signal
end