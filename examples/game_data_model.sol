model GameData(Episode)
    claire_inventory: Inventory
    claire_hp: Int
    claire_dp: Int
    story_progress: Episode
end

either EpisodeOne
    ColdOpen: either
        MeetingBruno
        MeetingAndy
        DimensionalRift
        FindingHearthaven
    end

    ActOne: either
        Hearthaven
        LeavingHearthaven
        Aurora
        TheGarden: either
            Intro
            TeaParty
            Miniboss
            AfterMiniboss
        end
        CastleOfDawn
        ShrineOfSpirit
        BeforeBoss
        AfterBoss
    end

    ActTwo: either
        ...
    end

    ActThree: either
        ...
    end
    
    PostCredits
end