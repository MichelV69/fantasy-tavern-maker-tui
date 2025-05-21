// ---- implementations  ----
pub mod list {
    use inflector::string::singularize::to_singular;

    use crate::dice_bag::tower;
    use crate::dice_bag::tower::RollDice;
    use crate::tavern::enums::list::{
        DrinkAlesDetail, DrinkCidersDetail, DrinkList, DrinkRumsDetail, DrinkWhiskeysDetail,
        DrinkWinesDetail, EstablishmentQualityLevel, FirstSmell, HouseDishHowCooked,
        HouseDishWhatCooked, HouseDishWhatSide, LightingAdjectives, LightingSources, LightingVerb,
        MoodData, NameNoun, NameVerb, PostedSignLocation, PostedSignMessage, RSLCode, SecondSmell,
        SizeList,
    };
    use crate::tavern::functions::{
        get_establishment_history_notes, get_establishment_quality, get_house_dish,
        get_house_drink, get_lighting, get_mood, get_name, get_pb_house_size, get_posted_sign,
        get_red_light_services_list, get_smells,
    };
    use crate::tavern::structs::list::{App, PBHouse, RedlightService};
    use crate::tavern::traits::list::{AppFn, DiceSize, ToCapitalized};
    use crate::text_postproc::tpp::{
        enum_string_to_phrase, is_a_an, l1_heading, l2_heading, l3_heading, tidy, trim_whitespace,
    };
    use rand::Rng;
    use rand::distr::{Distribution, StandardUniform};
    use std::fmt;

    impl Distribution<HouseDishWhatSide> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HouseDishWhatSide {
            let index: u8 = rng.random_range(0..=11);
            match index {
                0 => HouseDishWhatSide::BerrySauce,
                1 => HouseDishWhatSide::CheeseSauce,
                2 => HouseDishWhatSide::ChoppedPotatoes,
                3 => HouseDishWhatSide::HardBoiledGooseEggsAndSweetDates,
                4 => HouseDishWhatSide::HotCream,
                5 => HouseDishWhatSide::LeeksOnionsAndCatTails,
                6 => HouseDishWhatSide::MixedGreens,
                7 => HouseDishWhatSide::Mushrooms,
                8 => HouseDishWhatSide::OnionSoup,
                9 => HouseDishWhatSide::RoastedForestNuts,
                10 => HouseDishWhatSide::RootVegtables,
                11 => HouseDishWhatSide::SweetSavoryAndSpicyDippingSauces,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<HouseDishWhatCooked> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HouseDishWhatCooked {
            let index: u8 = rng.random_range(0..=10);
            match index {
                0 => HouseDishWhatCooked::BerryAndCheesePies,
                1 => HouseDishWhatCooked::BoarRibs,
                2 => HouseDishWhatCooked::HandFish,
                3 => HouseDishWhatCooked::MuttonLeg,
                4 => HouseDishWhatCooked::Pheasant,
                5 => HouseDishWhatCooked::PlainsStrider,
                6 => HouseDishWhatCooked::Sausage,
                7 => HouseDishWhatCooked::SerpentSteak,
                8 => HouseDishWhatCooked::Venison,
                9 => HouseDishWhatCooked::WildBoarChops,
                10 => HouseDishWhatCooked::WolfFlank,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<HouseDishHowCooked> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HouseDishHowCooked {
            let index: u8 = rng.random_range(0..=9);
            match index {
                0 => HouseDishHowCooked::Baked,
                1 => HouseDishHowCooked::Broiled,
                2 => HouseDishHowCooked::CharcoalGroundPit,
                3 => HouseDishHowCooked::DeepFried,
                4 => HouseDishHowCooked::DryAged,
                5 => HouseDishHowCooked::Fermented,
                6 => HouseDishHowCooked::FireRoasted,
                7 => HouseDishHowCooked::HoneyBraised,
                8 => HouseDishHowCooked::SmokedAndSeasoned,
                9 => HouseDishHowCooked::StuffedAndBaconWrapped,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<DrinkWinesDetail> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DrinkWinesDetail {
            let index: u8 = rng.random_range(0..=3);
            match index {
                0 => DrinkWinesDetail::Red,
                1 => DrinkWinesDetail::Rose,
                2 => DrinkWinesDetail::Sparkling,
                3 => DrinkWinesDetail::White,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<DrinkRumsDetail> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DrinkRumsDetail {
            let index: u8 = rng.random_range(0..=3);
            match index {
                0 => DrinkRumsDetail::Amber,
                1 => DrinkRumsDetail::Dark,
                2 => DrinkRumsDetail::Spiced,
                3 => DrinkRumsDetail::White,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<DrinkWhiskeysDetail> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DrinkWhiskeysDetail {
            let index: u8 = rng.random_range(0..=1);
            match index {
                0 => DrinkWhiskeysDetail::Blended,
                1 => DrinkWhiskeysDetail::SingleMalt,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<DrinkCidersDetail> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DrinkCidersDetail {
            let index: u8 = rng.random_range(0..=2);
            match index {
                0 => DrinkCidersDetail::Apple,
                1 => DrinkCidersDetail::Berry,
                2 => DrinkCidersDetail::Pear,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<DrinkAlesDetail> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DrinkAlesDetail {
            let index: u8 = rng.random_range(0..=3);
            match index {
                0 => DrinkAlesDetail::Dark,
                1 => DrinkAlesDetail::Hoppy,
                2 => DrinkAlesDetail::Light,
                3 => DrinkAlesDetail::Pale,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<DrinkList> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DrinkList {
            let index: u8 = rng.random_range(0..=5);
            match index {
                0 => DrinkList::Ales,
                1 => DrinkList::Ciders,
                2 => DrinkList::OtherStock,
                3 => DrinkList::Rums,
                4 => DrinkList::Whiskeys,
                5 => DrinkList::Wines,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<EstablishmentQualityLevel> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EstablishmentQualityLevel {
            let index: u8 = rng.random_range(0..=5);
            match index {
                0 => EstablishmentQualityLevel::Aristocratic,
                1 => EstablishmentQualityLevel::Comfortable,
                2 => EstablishmentQualityLevel::Modest,
                3 => EstablishmentQualityLevel::Poor,
                4 => EstablishmentQualityLevel::Squalid,
                5 => EstablishmentQualityLevel::Wealthy,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<SizeList> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SizeList {
            let index: u8 = rng.random_range(0..=4);
            match index {
                0 => SizeList::Large,
                1 => SizeList::Massive,
                2 => SizeList::Modest,
                3 => SizeList::Small,
                4 => SizeList::Tiny,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<PostedSignMessage> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PostedSignMessage {
            let index: u8 = rng.random_range(0..=3);
            match index {
                0 => PostedSignMessage::AdventurersCanEatButNoAlcoholOrRooms,
                1 => PostedSignMessage::AlcoholNotServedToOrcsGoblinsOrTieflings,
                2 => PostedSignMessage::AventurersWelcomePercentOff,
                3 => PostedSignMessage::CheapRoomForGoodPerformancesPercentOff,
                4 => PostedSignMessage::ColorfulNamesOfPriorGuests,
                5 => PostedSignMessage::FreeMealForGoodPerformances,
                6 => PostedSignMessage::NoSpellCasting,
                7 => PostedSignMessage::WarlocksShotOnSightOnSite,
                8 => PostedSignMessage::WeDontServeAdventurers,
                9 => PostedSignMessage::WeDontServeGoblins,
                10 => PostedSignMessage::WeDontServeTieflings,
                11 => PostedSignMessage::WeaponsNotPermitedToBeDrawn,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<PostedSignLocation> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PostedSignLocation {
            let index: u8 = rng.random_range(0..=5);
            match index {
                0 => PostedSignLocation::HungAroundTheNeckOfATrophyMountedStagsHead,
                1 => PostedSignLocation::HungFromTheFireplaceMantle,
                2 => PostedSignLocation::JustInsideTheDoor,
                3 => PostedSignLocation::JustOutsideTheDoor,
                4 => PostedSignLocation::OnTheFrontOfTheBar,
                5 => PostedSignLocation::OverTheBar,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<SecondSmell> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SecondSmell {
            let index: u8 = rng.random_range(0..=9);
            match index {
                0 => SecondSmell::BakingSweets,
                1 => SecondSmell::FermentingRye,
                2 => SecondSmell::FermentingWine,
                3 => SecondSmell::FoodsCooking,
                4 => SecondSmell::FreshPastries,
                5 => SecondSmell::Hops,
                6 => SecondSmell::HotSpicedCider,
                7 => SecondSmell::TheForests,
                8 => SecondSmell::TheOcean,
                9 => SecondSmell::TheOutsideSurroundings,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<FirstSmell> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FirstSmell {
            let index: u8 = rng.random_range(0..=9);
            match index {
                0 => FirstSmell::FreshLinen,
                1 => FirstSmell::HotBread,
                2 => FirstSmell::Perfumes,
                3 => FirstSmell::Shisha,
                4 => FirstSmell::SpicedTobacco,
                5 => FirstSmell::Spices,
                6 => FirstSmell::StrongDrink,
                7 => FirstSmell::Tobacco,
                8 => FirstSmell::WearyTravellers,
                9 => FirstSmell::WoodSmoke,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<LightingSources> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LightingSources {
            let index: u8 = rng.random_range(0..=3);
            match index {
                0 => LightingSources::AFireplace,
                1 => LightingSources::Candles,
                2 => LightingSources::MagicOrbsAndCrystals,
                3 => LightingSources::OilLamps,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<LightingVerb> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LightingVerb {
            let index: u8 = rng.random_range(0..=1);
            match index {
                0 => LightingVerb::Illuminated,
                1 => LightingVerb::Lit,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<LightingAdjectives> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LightingAdjectives {
            let index: u8 = rng.random_range(0..=4);
            match index {
                0 => LightingAdjectives::Brightly,
                1 => LightingAdjectives::Clearly,
                2 => LightingAdjectives::Dimly,
                3 => LightingAdjectives::Evenly,
                4 => LightingAdjectives::Shadowly,
                _ => unreachable!(),
            }
        }
    }
    impl Distribution<MoodData> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoodData {
            let index: u8 = rng.random_range(0..=15);
            match index {
                0 => MoodData::Busy,
                1 => MoodData::Dour,
                2 => MoodData::EnthusiasticGamblers,
                3 => MoodData::Erudite,
                4 => MoodData::Flirty,
                5 => MoodData::Jovial,
                6 => MoodData::Loud,
                7 => MoodData::LowerClass,
                8 => MoodData::MerchantFriendly,
                9 => MoodData::MiddleClass,
                10 => MoodData::Relaxing,
                11 => MoodData::Rowdy,
                12 => MoodData::Seedy,
                13 => MoodData::Shady,
                14 => MoodData::Smoky,
                15 => MoodData::Lonely,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<NameNoun> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NameNoun {
            let index: u8 = rng.random_range(0..=24);
            match index {
                0 => NameNoun::Bard,
                1 => NameNoun::Chalice,
                2 => NameNoun::Cockrel,
                3 => NameNoun::Crystal,
                4 => NameNoun::Curmudgeon,
                5 => NameNoun::Draft,
                6 => NameNoun::Dragon,
                7 => NameNoun::Dryad,
                8 => NameNoun::Fortune,
                9 => NameNoun::Harvest,
                10 => NameNoun::Hen,
                11 => NameNoun::Ice,
                12 => NameNoun::Meadow,
                13 => NameNoun::Mongrel,
                14 => NameNoun::Sky,
                15 => NameNoun::Snows,
                16 => NameNoun::Sun,
                17 => NameNoun::Tempest,
                18 => NameNoun::Tide,
                19 => NameNoun::Waters,
                20 => NameNoun::Waves,
                21 => NameNoun::Wench,
                22 => NameNoun::Werebear,
                23 => NameNoun::Oak,
                24 => NameNoun::Pines,
                _ => unreachable!(),
            }
        }
    }

    impl Distribution<NameVerb> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NameVerb {
            let index: u8 = rng.random_range(0..=20);
            match index {
                0 => NameVerb::Autumn,
                1 => NameVerb::Blue,
                2 => NameVerb::Carousing,
                3 => NameVerb::Checkered,
                4 => NameVerb::Drifting,
                5 => NameVerb::Fickle,
                6 => NameVerb::Flirting,
                7 => NameVerb::Green,
                8 => NameVerb::Heaving,
                9 => NameVerb::Lazy,
                10 => NameVerb::Melting,
                11 => NameVerb::Pouring,
                12 => NameVerb::Red,
                13 => NameVerb::Roaring,
                14 => NameVerb::Saucy,
                15 => NameVerb::Silver,
                16 => NameVerb::Spring,
                17 => NameVerb::Summer,
                18 => NameVerb::Waltzing,
                19 => NameVerb::Winter,
                20 => NameVerb::Yellow,
                _ => unreachable!(),
            }
        }
    }

    impl AppFn for App {
        fn get_version(&self) -> String {
            format!(
                "{}.{}.{}",
                self.version_major, self.version_minor, self.version_fix
            )
        }
    }

    impl App {
        pub fn new() -> Self {
            App {
                name: "Blank App Name".into(),
                version_major: 0,
                version_minor: 0,
                version_fix: 0,
                version_build: 0,
            }
        }
    }

    impl ToCapitalized for str {
        fn to_capitalized(&self) -> String {
            match self.len() {
                0 => String::new(),
                _ => {
                    let mut s = String::with_capacity(self.len());
                    s.extend(self.chars().next().unwrap().to_uppercase());
                    s.extend(self.chars().skip(1).flat_map(|c| c.to_lowercase()));
                    s
                }
            }
        }
    }

    impl PBHouse {
        pub fn new() -> Self {
            let eql = get_establishment_quality();
            let new_name = get_name();
            let size_data = get_pb_house_size();
            PBHouse {
                name: new_name.clone(),
                mood: get_mood(),
                lighting: get_lighting(),
                smells: get_smells(),
                size: size_data.clone(),
                establishment_quality: eql.clone(),
                posted_sign: get_posted_sign(),
                house_drink: get_house_drink(eql.level),
                house_dish: get_house_dish(eql.level),
                establishment_history_notes: get_establishment_history_notes(&new_name),
                redlight_services: {
                    get_red_light_services_list(size_data.size_description)
                        .unwrap_or_else(|| vec![])
                },
            }
        }

        pub fn general_info(&self) -> Vec<String> {
            let mut pb_house_desc: Vec<String> = Vec::with_capacity(22);
            // ---
            let prep = is_a_an(&self.mood);

            let para1: String = format!(
                "'*The {}*' is the local Pub and Bed House for travellers in this area. The {}-quality establishment would be considered {}-sized, with {} tables.\n\n",
                self.name,
                trim_whitespace(enum_string_to_phrase(
                    self.establishment_quality.level.to_string()
                )),
                trim_whitespace(enum_string_to_phrase(
                    self.size.size_description.to_string()
                )),
                self.size.table_count
            );
            pb_house_desc.push(para1);

            let bed_type_name = if self.size.common_bed_count == 1 {
                to_singular(&tidy(self.size.common_bed_type.to_string()))
            } else {
                tidy(self.size.common_bed_type.to_string())
            };
            let para2: String = format!(
                "It has {} {} in the common room and {} private rooms. Rooms are *{}* per day, and meals are *{}* per day.\n\n",
                self.size.common_bed_count,
                bed_type_name,
                self.size.private_room_count,
                self.establishment_quality.rooms,
                self.establishment_quality.meals
            );
            pb_house_desc.push(para2);

            let para3: String = format!(
                "As you enter, the air is full of the scents of {}. The current patrons seem to be {prep} {} bunch, {}. {}\n\n",
                self.smells,
                self.mood,
                self.lighting,
                self.posted_sign.clone()
            );
            pb_house_desc.push(para3);

            let para4: String = format!(
                "The menu has the usual standard fare posted. The House specialty beverage is {}, for {}, while the House specialty dish is {}, for {}.\n\n",
                self.house_drink.desc,
                self.house_drink.price,
                self.house_dish.desc,
                self.house_dish.price
            );
            pb_house_desc.push(para4);

            // ---
            pb_house_desc
        }
    }

    impl fmt::Display for PBHouse {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "{}", l1_heading("Narrative Information".to_string()))?;
            writeln!(f, " ")?;

            writeln!(f, "{}", l2_heading("For the Characters".to_string()))?;
            writeln!(f, " ")?;
            for line in &self.general_info() {
                write!(f, "{}", line)?;
            }

            writeln!(f, "{}", l2_heading("GM Notes".to_string()))?;
            writeln!(f, " ")?;
            writeln!(f, "{}", l3_heading("Establishment History".to_string()))?;
            writeln!(f, " ")?;
            for line in &self.establishment_history_notes {
                writeln!(f, "{}", line)?;
            }
            writeln!(f, " ")?;

            writeln!(f, "{}", l3_heading("Redlight Services".to_string()))?;
            let _ = &self.redlight_services.iter().for_each(|rsl| {
                writeln!(f, "{}", rsl.display()).expect("Expected vaild RSL");
            });
            writeln!(f, " ")?;

            Ok(())
        }
    }

    impl DiceSize for RSLCode {
        fn get_dc(&self) -> i16 {
            match self {
                RSLCode::None => 0,
                RSLCode::Gambling => tower::DiceResult::from_string("1d4+8").get_total(),
                RSLCode::Brothel => tower::DiceResult::from_string("1d6+10").get_total(),
                RSLCode::Smuggling => tower::DiceResult::from_string("2d4+11").get_total(),
                RSLCode::PitFighting => tower::DiceResult::from_string("2d6+12").get_total(),
                RSLCode::OpioidDen => tower::DiceResult::from_string("3d6+13").get_total(),
                RSLCode::RogueGuild => tower::DiceResult::from_string("3d8+16").get_total(),
            }
        }
    }

    impl RedlightService {
        pub(crate) fn display(&self) -> String {
            format!(
                " * {} (access DC {}+)",
                enum_string_to_phrase(self.service.to_string()).to_capitalized(),
                self.dc
            )
        }
    }
}

// ---- end of file ----
