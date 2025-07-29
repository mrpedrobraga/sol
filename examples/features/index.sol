name = "Maple's Room"
description = "The room where Maple sleeps."

def Object {
    name = "TV"
    check = scene
        * It's just some TV.
    end
    position = [9, 10]
    sprite = from "television.png" with Texture2DImporter
}

def Object {
    name = "Table"
    position = [8, 10]
}

if (let item = GameProgress.ActOne.ItemOnTable.0) then
    def Object {
        name = item.name
        check = item.check_interaction
        position = [10, 10]
    }
end