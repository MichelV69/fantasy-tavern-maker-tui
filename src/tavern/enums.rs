// ---- start of file ----
pub mod List {

    use strum::IntoEnumIterator;
    use strum_macros::Display;
    use strum_macros::EnumIter;
    use strum_macros::EnumString;

    use is_vowel;
    use rand::prelude;
    use rand_derive2::RandGen;
    use std::fmt;

    extern crate variant_count;
    use variant_count::VariantCount;
    // ---

    // ---
    #[derive(Display)]
    pub enum NameVerb {
        Autumn,
        Blue,
        Carousing,
        Checkered,
        Drifting,
        Fickle,
        Flirting,
        Green,
        Heaving,
        Lazy,
        Melting,
        Pouring,
        Red,
        Roaring,
        Saucy,
        Silver,
        Spring,
        Summer,
        Waltzing,
        Winter,
        Yellow,
    }

    #[derive(Display)]
    pub enum NameNoun {
        Bard,
        Chalice,
        Cockrel,
        Crystal,
        Curmudgeon,
        Draft,
        Dragon,
        Dryad,
        Fortune,
        Harvest,
        Hen,
        Ice,
        Meadow,
        Mongrel,
        Sky,
        Snows,
        Sun,
        Tempest,
        Tide,
        Waters,
        Waves,
        Wench,
        Werebear,
    }

    #[derive(Debug, Display, EnumString, Eq, PartialEq)]
    pub enum MoodData {
        Busy,
        Dour,
        EnthusiasticGamblers,
        Erudite,
        Flirty,
        Jovial,
        Loud,
        LowerClass,
        MerchantFriendly,
        MiddleClass,
        Relaxing,
        Rowdy,
        Seedy,
        Shady,
        Smoky,
        Subdued,
        UpperClass,
    }

    #[derive(Display)]
    pub enum LightingAdjectives {
        Brightly,
        Clearly,
        Dimly,
        Evenly,
        Shadowly,
    }

    #[derive(Display)]
    pub enum LightingVerb {
        Illuminated,
        Lit,
    }

    #[derive(Display)]
    pub enum LightingSources {
        AFireplace,
        Candles,
        MagicOrbsAndCrystals,
        OilLamps,
    }

    #[derive(Debug, Display, Clone, Copy)]
    pub enum EstablishmentQualityLevel {
        Aristocratic,
        Comfortable,
        Modest,
        Poor,
        Squalid,
        Wealthy,
    }
    // ---
    #[derive(Debug, Display, Clone)]
    pub enum SizeList {
        Large,
        Massive,
        Modest,
        Small,
        Tiny,
    }

    #[derive(Debug, Display, Clone)]
    pub enum BedTypeList {
        BunkBeds,
        Hammocks,
        SingleBeds,
        TentBeds,
    }

    #[derive(Display)]
    pub enum FirstSmell {
        FreshLinen,
        HotBread,
        Perfumes,
        Shisha,
        SpicedTobacco,
        Spices,
        StrongDrink,
        Tobacco,
        WearyTravellers,
        WoodSmoke,
    }

    #[derive(Display)]
    pub enum SecondSmell {
        BakingSweets,
        FermentingRye,
        FermentingWine,
        FoodsCooking,
        FreshPastries,
        Hops,
        HotSpicedCider,
        TheForests,
        TheOcean,
        TheOutsideSurroundings,
    }

    #[derive(Display)]
    pub enum PostedSignLocation {
        HungAroundTheNeckOfATrophyMountedStagsHead,
        HungFromTheFireplaceMantle,
        JustInsideTheDoor,
        JustOutsideTheDoor,
        OnTheFrontOfTheBar,
        OverTheBar,
    }

    #[derive(Display, PartialEq)]
    pub enum PostedSignMessage {
        AdventurersCanEatButNoAlcoholOrRooms,
        AlcoholNotServedToOrcsGoblinsOrTieflings,
        AventurersWelcomePercentOff,
        CheapRoomForGoodPerformancesPercentOff,
        ColorfulNamesOfPriorGuests,
        FreeMealForGoodPerformances,
        NoSpellCasting,
        WarlocksShotOnSightOnSite,
        WeaponsNotPermitedToBeDrawn,
        WeDontServeAdventurers,
        WeDontServeGoblins,
        WeDontServeTieflings,
    }

    #[derive(Display)]
    pub enum HouseDishHowCooked {
        Baked,
        Broiled,
        CharcoalGroundPit,
        DeepFried,
        DryAged,
        Fermented,
        FireRoasted,
        HoneyBraised,
        Poached,
        SlowRoasted,
        SmokedAndSeasoned,
        StuffedAndBaconWrapped,
    }

    #[derive(Display)]
    pub enum HouseDishWhatCooked {
        BerryAndCheesePies,
        BoarRibs,
        HandFish,
        MuttonLeg,
        Pheasant,
        PlainsStrider,
        PlatterFish,
        Sausage,
        SerpentSteak,
        Venison,
        WildBoarChops,
        WolfFlank,
    }

    #[derive(Display)]
    pub enum HouseDishWhatSide {
        BerrySauce,
        CheeseSauce,
        ChoppedPotatoes,
        HardBoiledGooseEggsAndSweetDates,
        HotCream,
        LeeksOnionsAndCatTails,
        MixedGreens,
        Mushrooms,
        OnionSoup,
        RoastedForestNuts,
        RootVegtables,
        SweetSavoryAndSpicyDippingSauces,
    }

    #[derive(Display)]
    pub enum DrinkList {
        Ales,
        Ciders,
        OtherStock,
        Rums,
        Whiskeys,
        Wines,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum DrinkMade {
        ALocallyMade,
        AnImported,
        TheHousesOwn,
    }

    #[derive(Display)]
    pub enum DrinkAlesDetail {
        Dark,
        Hoppy,
        Light,
        Pale,
    }

    #[derive(Display)]
    pub enum DrinkCidersDetail {
        Apple,
        Berry,
        Pear,
    }

    #[derive(Display)]
    pub enum DrinkRumsDetail {
        Amber,
        Dark,
        Spiced,
        White,
    }

    #[derive(Display)]
    pub enum DrinkWhiskeysDetail {
        Blended,
        SingleMalt,
    }

    #[derive(Display)]
    pub enum DrinkWinesDetail {
        Red,
        Rose,
        Sparkling,
        White,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum EstablishmentHistoryAge {
        Generational,
        Permanent,
        Recent,
        WellEstablished,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum EstablishmentAppearance {
        BrandNew,
        GoodCondition,
        MinorRepairs,
        WhiteWashed,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum EstablishmentReputuation {
        MerchantsLike,
        MilitaPatrol,
        MurderScene,
        PlotRumors,
    }
}

// ---- end of file ----
