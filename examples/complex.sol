-- An item which can be held in the inventory and consumed
model Item
    name: Text
    description: Text
    is_key_item: Truth = no
    use: Action(user: Character, target: Character)
end

model Character
    name: Text
    hp: Int
    dp: Int
end

model StatusAlteration
    -- Stuff inside!
end

either
    -- AAah.
end

def apple as Item
    name = "Apple"
    description = "Gorgeous Crimson Apple"

    action use(user: Character, target: Character)
        StatusAlteration.new()
            .heal_hp(10)
            .heal_dp(5)
            .apply(target)
    end
end