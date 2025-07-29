@model Item

name = "Wooden Sword"
description = "A humble maple wood sword you can bonk enemies with."
attack_damage = 20
durability = 20

action on_use(user, target)
    if target.is_enemy() then
        target.damage(attack_damage)
        self.durability -= 1
        if self.durability <= 0 then
            destroyed()
        end
    else
        * Don't hit your friends!
    end
end