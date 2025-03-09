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
    #[derive(Display)]
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

    #[derive(Display)]
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

    #[derive(Debug, Display, EnumString, Eq, PartialEq)]
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

    #[derive(Display)]
    pub enum LightingAdjectives {
        Brightly,
        Clearly,
        Evenly,
        Dimly,
        Shadowly,
    }

    #[derive(Display)]
    pub enum LightingVerb {
        Lit,
        Illuminated,
    }

    #[derive(Display)]
    pub enum LightingSources {
        Candles,
        AFireplace,
        OilLamps,
        MagicOrbsAndCrystals,
    }

    #[derive(Debug, Display, Clone, Copy)]
    pub enum EstablishmentQualityLevel {
        Squalid,
        Poor,
        Modest,
        Comfortable,
        Wealthy,
        Aristocratic,
    }
    // ---
    #[derive(Debug, Display)]
    pub enum SizeList {
        Tiny,
        Small,
        Modest,
        Large,
        Massive,
    }

    #[derive(Debug, Display)]
    pub enum BedTypeList {
        Hammocks,
        BunkBeds,
        SingleBeds,
        TentBeds,
    }

    #[derive(Display)]
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

    #[derive(Display)]
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

    #[derive(Display)]
    pub enum PostedSignLocation {
        OverTheBar,
        OnTheFrontOfTheBar,
        JustInsideTheDoor,
        JustOutsideTheDoor,
        HungFromTheFireplaceMantle,
        HungAroundTheNeckOfATrophyMountedStagsHead,
    }

    #[derive(Display, PartialEq)]
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

    #[derive(Display)]
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

    #[derive(Display)]
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

    #[derive(Display)]
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

    #[derive(Display)]
    pub enum DrinkList {
        Ales,
        Ciders,
        Whiskeys,
        Rums,
        Wines,
        OtherStock,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum DrinkMade {
        AnImported,
        ALocallyMade,
        TheHousesOwn,
    }

    #[derive(Display)]
    pub enum DrinkAlesDetail {
        Dark,
        Light,
        Hoppy,
        Pale,
    }

    #[derive(Display)]
    pub enum DrinkCidersDetail {
        Apple,
        Pear,
        Berry,
    }

    #[derive(Display)]
    pub enum DrinkRumsDetail {
        White,
        Amber,
        Dark,
        Spiced,
    }

    #[derive(Display)]
    pub enum DrinkWhiskeysDetail {
        SingleMalt,
        Blended,
    }

    #[derive(Display)]
    pub enum DrinkWinesDetail {
        Red,
        White,
        Rose,
        Sparkling,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum EstablishmentHistoryAge {
        Generational,
        Permanent,
        WellEstablished,
        Recent,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum EstablishmentAppearance {
        MinorRepairs,
        GoodCondition,
        BrandNew,
        WhiteWashed,
    }

    #[derive(Display, VariantCount, EnumIter)]
    pub enum EstablishmentReputuation {
        PlotRumors,
        MerchantsLike,
        MilitaPatrol,
        MurderScene,
    }
}

// ---- end of file ----
