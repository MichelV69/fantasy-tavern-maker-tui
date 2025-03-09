// ---- start of file ----
pub mod List {

    use strum::IntoEnumIterator;
    use strum_macros::Display;
    use strum_macros::EnumIter;
    use strum_macros::EnumString;

    use is_vowel::*;
    use rand::prelude::*;
    use rand_derive2::RandGen;
    use std::fmt;

    extern crate variant_count;
    use variant_count::VariantCount;
    // ---

    // ---
    #[derive(RandGen, Display)]
    pub enum NameVerb {
        Waltzing,
        Checkered,
        Lazy,
        Silver,
        Saucy,
        Flirting,
        Blue,
        Red,
        Green,
        Yellow,
        Fickle,
        Roaring,
        Carousing,
        Melting,
        Drifting,
        Spring,
        Winter,
        Summer,
        Autumn,
        Pouring,
        Heaving,
    }

    #[derive(RandGen, Display)]
    pub enum NameNoun {
        Werebear,
        Cockrel,
        Hen,
        Dragon,
        Wench,
        Dryad,
        Sky,
        Tide,
        Meadow,
        Sun,
        Fortune,
        Waters,
        Bard,
        Curmudgeon,
        Crystal,
        Mongrel,
        Ice,
        Tempest,
        Snows,
        Draft,
        Harvest,
        Chalice,
        Waves,
    }

    #[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
    pub enum MoodData {
        Jovial,
        Relaxing,
        Smoky,
        Erudite,
        Loud,
        Subdued,
        Rowdy,
        Seedy,
        Shady,
        Busy,
        LowerClass,
        MiddleClass,
        UpperClass,
        MerchantFriendly,
        EnthusiasticGamblers,
        Dour,
        Flirty,
    }

    #[derive(Display, RandGen)]
    pub enum LightingAdjectives {
        Brightly,
        Clearly,
        Evenly,
        Dimly,
        Shadowly,
    }

    #[derive(Display, RandGen)]
    pub enum LightingVerb {
        Lit,
        Illuminated,
    }

    #[derive(Display, RandGen)]
    pub enum LightingSources {
        Candles,
        AFireplace,
        OilLamps,
        MagicOrbsAndCrystals,
    }

    #[derive(Debug, Display, RandGen, Clone, Copy, Serialize)]
    pub enum EstablishmentQualityLevel {
        Squalid,
        Poor,
        Modest,
        Comfortable,
        Wealthy,
        Aristocratic,
    }
    // ---
    #[derive(Debug, Display, RandGen, Serialize)]
    pub enum SizeList {
        Tiny,
        Small,
        Modest,
        Large,
        Massive,
    }

    #[derive(Debug, Display, Serialize)]
    pub enum BedTypeList {
        Hammocks,
        BunkBeds,
        SingleBeds,
        TentBeds,
    }

    #[derive(Display, RandGen)]
    pub enum FirstSmell {
        WoodSmoke,
        Spices,
        Perfumes,
        WearyTravellers,
        StrongDrink,
        Tobacco,
        SpicedTobacco,
        Shisha,
        FreshLinen,
        HotBread,
    }

    #[derive(Display, RandGen)]
    pub enum SecondSmell {
        FreshPastries,
        FoodsCooking,
        TheOutsideSurroundings,
        TheOcean,
        TheForests,
        FermentingWine,
        Hops,
        FermentingRye,
        HotSpicedCider,
        BakingSweets,
    }

    #[derive(Display, RandGen)]
    pub enum PostedSignLocation {
        OverTheBar,
        OnTheFrontOfTheBar,
        JustInsideTheDoor,
        JustOutsideTheDoor,
        HungFromTheFireplaceMantle,
        HungAroundTheNeckOfATrophyMountedStagsHead,
    }

    #[derive(Display, RandGen, PartialEq)]
    pub enum PostedSignMessage {
        WeDontServeAdventurers,
        WeDontServeTieflings,
        FreeMealForGoodPerformances,
        CheapRoomForGoodPerformancesPercentOff,
        WeaponsNotPermitedToBeDrawn,
        NoSpellCasting,
        WeDontServeGoblins,
        AventurersWelcomePercentOff,
        AdventurersCanEatButNoAlcoholOrRooms,
        AlcoholNotServedToHalfOrcsHalflingsOrTieflings,
        ColorfulNamesOfPriorGuests,
        WarlocksShotOnSightOnSite,
    }

    #[derive(RandGen, Display)]
    pub enum HouseDishHowCooked {
        SlowRoasted,
        SmokedAndSeasoned,
        FireRoasted,
        DryAged,
        Broiled,
        Baked,
        HoneyBraised,
        Poached,
        Fermented,
        StuffedAndBaconWrapped,
        DeepFried,
        CharcoalGroundPit,
    }

    #[derive(RandGen, Display)]
    pub enum HouseDishWhatCooked {
        MuttonLeg,
        Venison,
        PlatterFish,
        HandFish,
        WildBoarChops,
        Sausage,
        BoarRibs,
        BerryAndCheesePies,
        WolfFlank,
        Pheasant,
        SerpentSteak,
        PlainsStrider,
    }

    #[derive(RandGen, Display)]
    pub enum HouseDishWhatSide {
        RootVegtables,
        Mushrooms,
        CheeseSauce,
        HotCream,
        HardBoiledGooseEggsAndSweetDates,
        OnionSoup,
        BerrySauce,
        ChoppedPotatoes,
        MixedGreens,
        LeeksOnionsAndCatTails,
        RoastedForestNuts,
        SweetSavoryAndSpicyDippingSauces,
    }

    #[derive(RandGen, Display)]
    pub enum DrinkList {
        Ales,
        Ciders,
        Whiskeys,
        Rums,
        Wines,
        OtherStock,
    }

    #[derive(RandGen, Display, VariantCount, EnumIter)]
    pub enum DrinkMade {
        AnImported,
        ALocallyMade,
        TheHousesOwn,
    }

    #[derive(RandGen, Display)]
    pub enum DrinkAlesDetail {
        Dark,
        Light,
        Hoppy,
        Pale,
    }

    #[derive(RandGen, Display)]
    pub enum DrinkCidersDetail {
        Apple,
        Pear,
        Berry,
    }

    #[derive(RandGen, Display)]
    pub enum DrinkRumsDetail {
        White,
        Amber,
        Dark,
        Spiced,
    }

    #[derive(RandGen, Display)]
    pub enum DrinkWhiskeysDetail {
        SingleMalt,
        Blended,
    }

    #[derive(RandGen, Display)]
    pub enum DrinkWinesDetail {
        Red,
        White,
        Rose,
        Sparkling,
    }

    #[derive(RandGen, Display, VariantCount, EnumIter)]
    pub enum EstablishmentHistoryAge {
        Generational,
        Permanent,
        WellEstablished,
        Recent,
    }

    #[derive(RandGen, Display, VariantCount, EnumIter)]
    pub enum EstablishmentAppearance {
        MinorRepairs,
        GoodCondition,
        BrandNew,
        WhiteWashed,
    }

    #[derive(RandGen, Display, VariantCount, EnumIter)]
    pub enum EstablishmentReputuation {
        PlotRumors,
        MerchantsLike,
        MilitaPatrol,
        MurderScene,
    }
}

// ---- end of file ----
