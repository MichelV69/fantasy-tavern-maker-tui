use super::build::{
    Attribute, EyeColorCode, GenderCode, HairColorCode, HairStyleCode, NpcTypeCode, Profile,
    QuirkEmotional, QuirkPhysical, SpeciesCode,
};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use tracing::{Level, event};

use super::lib::fnset::{RollTable, read_psv_file};
use crate::{
    dice_bag::tower::{self, DiceResult, RollDice},
    tavern::structs::list::App,
    text_postproc::tpp::{enum_string_to_phrase, is_a_an},
};

impl Profile {
    pub fn new() -> Self {
        Profile {
            npc_type: NpcTypeCode::Patron,
            gender: GenderCode::Androgynous,
            task_description: "Realm's Most Interesting Person".into(),
            species: SpeciesCode::Dragonborn,
            height_desc: "about average".into(),
            build_desc: "about average".into(),
            hair_color: HairColorCode::Blonde,
            hair_style: HairStyleCode::BeadedBraided,
            eye_color: EyeColorCode::Red,
            quirk_emotional: QuirkEmotional::Manic,
            quirk_physical: QuirkPhysical::SubstantialWineStain,
            notable_attribute_positive: Attribute {
                description: "nothing interesting".into(),
                modifier: -13,
            },
            notable_attribute_negative: Attribute {
                description: "nothing interesting".into(),
                modifier: -13,
            },
            //schtick_ability_description: "nothing interesting".into(),
        }
    }

    pub fn set_random_schticks_attributes(&mut self, app: &App) {
        self.set_notable_attribute_positive(&app);
        self.set_notable_attribute_negative(&app);
        // todo!("schtick_ability_description");
    }

    pub fn random_appearance(&mut self, app: &App) {
        self.set_random_gender();
        self.set_random_species(&app);
        self.set_random_height_desc();
        self.set_random_build_desc();
        self.set_random_hair_color(&app);
        self.set_random_hair_style(&app);
        self.set_random_eye_color(&app);
        self.set_random_quirk_physical(&app);
    }

    pub fn get_random_notable_attribute(&mut self, app: &App) -> Attribute {
        let mut test_file = "table-RandomNotableAttributeStat.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let description = Self::roll_from_table(psv_file_contents);

        test_file = "table-RandomNotableAttributeBonus.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let bonus = Self::roll_from_table(psv_file_contents)
            .parse::<i8>()
            .expect("Should have gotten an integer here!");

        Attribute {
            description: description,
            modifier: bonus.into(),
        }
    }

    pub fn set_notable_attribute_positive(&mut self, app: &App) {
        self.notable_attribute_positive = self.get_random_notable_attribute(app);
    }

    pub fn set_notable_attribute_negative(&mut self, app: &App) {
        let mut temp_attr = self.get_random_notable_attribute(app);
        temp_attr.modifier = temp_attr.modifier * -1;
        self.notable_attribute_negative = temp_attr;
    }

    pub fn set_random_quirk_physical(&mut self, app: &App) {
        let test_file = "table-RandomQuirkPhysical.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let result = Self::roll_from_table(psv_file_contents);
        self.quirk_physical = match result {
            val if val == "None" => QuirkPhysical::None,
            val if val == "SlightScar" => QuirkPhysical::SlightScar,
            val if val == "NoticeableScar" => QuirkPhysical::NoticeableScar,
            val if val == "SubstantialScar" => QuirkPhysical::SubstantialScar,
            val if val == "InfrequentSquint" => QuirkPhysical::InfrequentSquint,
            val if val == "FrequentSquint" => QuirkPhysical::FrequentSquint,
            val if val == "ConstantSquint" => QuirkPhysical::ConstantSquint,
            val if val == "SmallTattoo" => QuirkPhysical::SmallTattoo,
            val if val == "MinorScarification" => QuirkPhysical::MinorScarification,
            val if val == "NoticeableTattoo" => QuirkPhysical::NoticeableTattoo,
            val if val == "NoticeableScarification" => QuirkPhysical::NoticeableScarification,
            val if val == "SubstantialTattoo" => QuirkPhysical::SubstantialTattoo,
            val if val == "SubstantialScarification" => QuirkPhysical::SubstantialScarification,
            val if val == "SlightWineStain" => QuirkPhysical::SlightWineStain,
            val if val == "NoticeableWineStain" => QuirkPhysical::NoticeableWineStain,
            val if val == "SubstantialWineStain" => QuirkPhysical::SubstantialWineStain,
            _ => panic!("set_random_quirk_physical result: [{result}]"),
        };
    }

    pub fn set_random_quirk_emotional(&mut self, app: &App) {
        let test_file = "table-RandomQuirkEmotional.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let result = Self::roll_from_table(psv_file_contents);
        self.quirk_emotional = match result {
            val if val == "None" => QuirkEmotional::None,
            val if val == "DistrustfulOfAdventurers" => QuirkEmotional::DistrustfulOfAdventurers,
            val if val == "CheerfulToAdventurers" => QuirkEmotional::CheerfulToAdventurers,
            val if val == "Shy" => QuirkEmotional::Shy,
            val if val == "Grumpy" => QuirkEmotional::Grumpy,
            val if val == "Playful" => QuirkEmotional::Playful,
            val if val == "Loud" => QuirkEmotional::Loud,
            val if val == "Miserly" => QuirkEmotional::Miserly,
            val if val == "Hyperfocused" => QuirkEmotional::Hyperfocused,
            val if val == "Belligerent" => QuirkEmotional::Belligerent,
            val if val == "PhysicallyAffectionate" => QuirkEmotional::PhysicallyAffectionate,
            val if val == "Generous" => QuirkEmotional::Generous,
            val if val == "EasilyDistracted" => QuirkEmotional::EasilyDistracted,
            val if val == "Depressive" => QuirkEmotional::Depressive,
            val if val == "Manic" => QuirkEmotional::Manic,
            _ => panic!("set_random_quirk_emotional result: [{result}]"),
        };
    }

    pub fn set_random_eye_color(&mut self, app: &App) {
        let test_file = "table-RandomEyeColor.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let result = Self::roll_from_table(psv_file_contents);
        self.eye_color = match result {
            val if val == "Brown" => EyeColorCode::Brown,
            val if val == "Hazel" => EyeColorCode::Hazel,
            val if val == "Blue" => EyeColorCode::Blue,
            val if val == "Green" => EyeColorCode::Green,
            val if val == "Grey" => EyeColorCode::Grey,
            val if val == "Amber" => EyeColorCode::Amber,
            val if val == "Purple" => EyeColorCode::Purple,
            val if val == "Red" => EyeColorCode::Red,
            _ => panic!("set_random_eye_color result: [{result}]"),
        };
    }

    pub fn set_random_hair_style(&mut self, app: &App) {
        let test_file = "table-RandomHairStyle.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let result = Self::roll_from_table(psv_file_contents);
        self.hair_style = match result {
            val if val == "ShortBraided" => HairStyleCode::ShortBraided,
            val if val == "ShortPonytail" => HairStyleCode::ShortPonytail,
            val if val == "ShortLoose" => HairStyleCode::ShortLoose,
            val if val == "ShortCurls" => HairStyleCode::ShortCurls,
            val if val == "LongPonytail" => HairStyleCode::LongPonytail,
            val if val == "ShortMohawk" => HairStyleCode::ShortMohawk,
            val if val == "LongLoose" => HairStyleCode::LongLoose,
            val if val == "LongCurls" => HairStyleCode::LongCurls,
            val if val == "LongBraided" => HairStyleCode::LongBraided,
            val if val == "TopKnot" => HairStyleCode::TopKnot,
            val if val == "ShavedClean" => HairStyleCode::ShavedClean,
            val if val == "TallMohawk" => HairStyleCode::TallMohawk,
            val if val == "CroppedMohawk" => HairStyleCode::CroppedMohawk,
            val if val == "CrewCut" => HairStyleCode::CrewCut,
            val if val == "BeadedBraided" => HairStyleCode::BeadedBraided,

            _ => panic!("set_random_hair_color result: [{result}]"),
        };
    }

    pub fn set_random_hair_color(&mut self, app: &App) {
        let test_file = "table-RandomHairColor.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let result = Self::roll_from_table(psv_file_contents);
        self.hair_color = match result {
            val if val == "Dark" => HairColorCode::Dark,
            val if val == "Brown" => HairColorCode::Brown,
            val if val == "Blonde" => HairColorCode::Blonde,
            val if val == "White" => HairColorCode::White,
            val if val == "Silver-Grey" => HairColorCode::SilverGrey,
            val if val == "Red" => HairColorCode::Red,
            val if val == "Green" => HairColorCode::Green,
            val if val == "Blue" => HairColorCode::Blue,
            val if val == "Purple" => HairColorCode::Purple,
            _ => panic!("set_random_hair_color result: [{result}]"),
        };
    }

    pub fn set_random_build_desc(&mut self) {
        let roll_2d6: i16 = <tower::DiceResult>::from_string("2d6").get_total();
        self.build_desc = match roll_2d6 {
            2 => <tower::DiceResult>::inline_replace("gaunt (-[3d8+6]%)"),
            3 => <tower::DiceResult>::inline_replace("lean (-[2d8+3]%)"),
            4..=5 => <tower::DiceResult>::inline_replace("slightly angular (-[1d8+1]%)"),
            6..=8 => <tower::DiceResult>::inline_replace("medium build ([2d4-4]%)"),
            9..=10 => <tower::DiceResult>::inline_replace("slightly husky (+[1d8+1]%)"),
            11 => <tower::DiceResult>::inline_replace("stout (+[2d8+3]%)"),
            12 => <tower::DiceResult>::inline_replace("portly (+[3d8+6]%)"),
            _ => panic!("Rolled weird on 2d6 : [{roll_2d6}] "),
        };
    }

    pub fn set_random_height_desc(&mut self) {
        let roll_2d6: i16 = <tower::DiceResult>::from_string("2d6").get_total();
        self.height_desc = match roll_2d6 {
            2 => <tower::DiceResult>::inline_replace("very short (-[3d8+6]%)"),
            3 => <tower::DiceResult>::inline_replace("short (-[2d8+3]%)"),
            4..=5 => <tower::DiceResult>::inline_replace("short-ish (-[1d8+1]%)"),
            6..=8 => <tower::DiceResult>::inline_replace("average height ([2d4-4]%)"),
            9..=10 => <tower::DiceResult>::inline_replace("tall-ish (+[1d8+1]%)"),
            11 => <tower::DiceResult>::inline_replace("tall (+[2d8+3]%)"),
            12 => <tower::DiceResult>::inline_replace("very tall (+[3d8+6]%)"),
            _ => panic!("Rolled weird on 2d6 : [{roll_2d6}] "),
        };
    }

    pub fn set_random_species(&mut self, app: &App) {
        let test_file = "table-RandomSpeciesByWeight.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        let result = Self::roll_from_table(psv_file_contents);
        self.species = match result {
            val if val == "Human" => SpeciesCode::Human,
            val if val == "Dwarf" => SpeciesCode::Dwarf,
            val if val == "Halfling" => SpeciesCode::Halfling,
            val if val == "Elf" => SpeciesCode::Elf,
            val if val == "Gnome" => SpeciesCode::Gnome,
            val if val == "Tiefling" => SpeciesCode::Tiefling,
            val if val == "Dragonborn" => SpeciesCode::Dragonborn,
            _ => panic!("set_random_species result: [{result}]"),
        };
    }

    fn roll_from_table(psv_file_contents: Vec<(i16, String)>) -> String {
        // build the table
        let mut result_table: Vec<RollTable> = Vec::with_capacity(42);

        let mut high: i16 = 0;
        for line in psv_file_contents {
            let mut low: i16 = 1;
            if result_table.len() > 0 {
                low = result_table[result_table.len() - 1].high + 1;
            }
            high = low + line.0 - 1;
            let result = line.1;

            let to_push: RollTable = RollTable { low, high, result };
            result_table.push(to_push);
        }

        // roll from the table
        let mut rng = rand::rng();
        let table_roll = rng.random_range(1..=high);
        let mut table_result: String = "".into();
        for line in result_table {
            if (table_roll >= line.low) && (table_roll <= line.high) {
                table_result = line.result
            };
        }

        event!(Level::INFO, "table_result[{:#?}]", table_result);
        if table_result.is_empty() {
            panic!("table_result should never be empty! [{table_roll}:{high}]")
        }

        table_result
    }

    pub fn set_random_npc_type_code(&mut self) {
        self.npc_type = rand::random();
    }

    pub fn set_random_gender(&mut self) {
        let set_of_rolls = tower::DiceResult::from_string("1d10");
        let total_of_rolls = set_of_rolls.get_total();

        self.gender = match total_of_rolls {
            1..=5 => GenderCode::Male,
            6..=8 => GenderCode::Female,
            9..=10 => GenderCode::Androgynous,
            _ => panic!(
                "Unexpected roll out of bounds {}:{}",
                set_of_rolls.get_request(),
                total_of_rolls
            ),
        }
    }

    pub fn set_random_task_description(&mut self, app: &App) {
        let test_file = "table-RandomTaskDesc.psv";
        let psv_file_contents = read_psv_file(test_file, &app);
        self.task_description = Self::roll_from_table(psv_file_contents);
        self.task_description = <DiceResult as RollDice>::inline_replace(&self.task_description);
    }
}

// ---
impl Distribution<NpcTypeCode> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NpcTypeCode {
        let index: u8 = rng.random_range(0..=2);
        match index {
            0 => NpcTypeCode::Patron,
            1 => NpcTypeCode::Staff,
            2 => NpcTypeCode::StoryCharacter,
            _ => unreachable!(),
        }
    }
}

// ---
impl ToString for Attribute {
    fn to_string(&self) -> String {
        if self.description == "None" {
            return "".to_string();
        };
        let mut mod_sign = "";
        if self.modifier > -1 {
            mod_sign = "+"
        }
        format!("{}{}{}", self.description, mod_sign, self.modifier)
    }
}

impl ToString for QuirkPhysical {
    fn to_string(&self) -> String {
        let qp_string = match self {
            QuirkPhysical::ConstantSquint => enum_string_to_phrase("ConstantSquint".to_string()),
            QuirkPhysical::FrequentSquint => enum_string_to_phrase("FrequentSquint".to_string()),
            QuirkPhysical::InfrequentSquint => {
                enum_string_to_phrase("InfrequentSquint".to_string())
            }
            QuirkPhysical::None => "".to_string(),
            QuirkPhysical::SlightScar => enum_string_to_phrase("SlightScar".to_string()),
            QuirkPhysical::NoticeableScar => enum_string_to_phrase("NoticeableScar".to_string()),
            QuirkPhysical::SubstantialScar => enum_string_to_phrase("SubstantialScar".to_string()),
            QuirkPhysical::SmallTattoo => enum_string_to_phrase("SmallTattoo".to_string()),
            QuirkPhysical::NoticeableTattoo => {
                enum_string_to_phrase("NoticeableTattoo".to_string())
            }
            QuirkPhysical::SubstantialTattoo => {
                enum_string_to_phrase("SubstantialTattoo".to_string())
            }
            QuirkPhysical::MinorScarification => {
                enum_string_to_phrase("MinorScarification".to_string())
            }
            QuirkPhysical::NoticeableScarification => {
                enum_string_to_phrase("NoticeableScarification".to_string())
            }
            QuirkPhysical::SubstantialScarification => {
                enum_string_to_phrase("SubstantialScarification".to_string())
            }
            QuirkPhysical::SlightWineStain => enum_string_to_phrase("SlightWineStain".to_string()),
            QuirkPhysical::NoticeableWineStain => {
                enum_string_to_phrase("NoticeableWineStain".to_string())
            }
            QuirkPhysical::SubstantialWineStain => {
                enum_string_to_phrase("SubstantialWineStain".to_string())
            }
        };

        if qp_string.is_empty() {
            return qp_string;
        } else {
            return format!("a {}", qp_string.to_string().trim());
        }
    }
}

impl ToString for QuirkEmotional {
    fn to_string(&self) -> String {
        let qe_string = match self {
            QuirkEmotional::Belligerent => "Belligerent".to_string(),
            QuirkEmotional::CheerfulToAdventurers => {
                enum_string_to_phrase("CheerfulToAdventurers".to_string())
            }
            QuirkEmotional::Depressive => "Depressive".to_string(),
            QuirkEmotional::DistrustfulOfAdventurers => {
                enum_string_to_phrase("DistrustfulOfAdventurers".to_string())
            }
            QuirkEmotional::EasilyDistracted => {
                enum_string_to_phrase("EasilyDistracted".to_string())
            }
            QuirkEmotional::Generous => "Generous".to_string(),
            QuirkEmotional::Hyperfocused => "Hyperfocused".to_string(),
            QuirkEmotional::Grumpy => "Grumpy".to_string(),
            QuirkEmotional::Loud => "Loud".to_string(),
            QuirkEmotional::Manic => "Manic".to_string(),
            QuirkEmotional::None => "".to_string(),
            QuirkEmotional::PhysicallyAffectionate => {
                enum_string_to_phrase("PhysicallyAffectionate".to_string())
            }
            QuirkEmotional::Miserly => "Miserly".to_string(),
            QuirkEmotional::Playful => "Playful".to_string(),
            QuirkEmotional::Shy => "Shy".to_string(),
        };
        format!("{}", qe_string.to_lowercase().trim())
    }
}

impl ToString for HairColorCode {
    fn to_string(&self) -> String {
        let hcc_string = match self {
            HairColorCode::Blonde => "Blonde",
            HairColorCode::Blue => "Blue",
            HairColorCode::Brown => "Brown",
            HairColorCode::Dark => "Dark",
            HairColorCode::Green => "Green",
            HairColorCode::Purple => "Purple",
            HairColorCode::Red => "Red",
            HairColorCode::SilverGrey => "SilverGrey",
            HairColorCode::White => "White",
        };
        format!(
            "{}",
            enum_string_to_phrase(hcc_string.to_string()).to_lowercase().trim()
        )
    }
}

impl ToString for HairStyleCode {
    fn to_string(&self) -> String {
        let hsc_string = match self {
            HairStyleCode::BeadedBraided => "BeadedBraided",
            HairStyleCode::CrewCut => "CrewCut",
            HairStyleCode::CroppedMohawk => "CroppedMohawk",
            HairStyleCode::LongBraided => "LongBraided",
            HairStyleCode::LongCurls => "LongCurls",
            HairStyleCode::LongLoose => "LongLoose",
            HairStyleCode::LongPonytail => "LongPonytail",
            HairStyleCode::ShavedClean => "ShavedClean",
            HairStyleCode::ShortBraided => "ShortBraided",
            HairStyleCode::ShortCurls => "ShortCurls",
            HairStyleCode::ShortLoose => "ShortLoose",
            HairStyleCode::TallMohawk => "TallMohawk",
            HairStyleCode::ShortMohawk => "ShortMohawk",
            HairStyleCode::ShortPonytail => "ShortPonytail",
            HairStyleCode::TopKnot => "TopKnot",
        };
        format!(
            "{} {}", is_a_an(hsc_string),
            enum_string_to_phrase(hsc_string.to_string()).to_lowercase().trim()
        )
    }
}


impl ToString for EyeColorCode  {
    fn to_string(&self) -> String {
        let ecc_string = match self {
            EyeColorCode::Amber => "Amber",
            EyeColorCode::Brown => "Brown",
            EyeColorCode::Blue => "Blue",
            EyeColorCode::Hazel => "Hazel",
            EyeColorCode::Green => "Green",
            EyeColorCode::Grey => "Grey",
            EyeColorCode::Purple => "Purple",
            EyeColorCode::Red => "Red",
        };

        format!("{}",ecc_string.to_lowercase().trim())
    }

}
// ---- end of file ----
