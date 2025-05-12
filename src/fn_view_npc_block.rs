use crate::npc::build::Profile;

pub fn view_npc_block(npc_data: &Profile) -> String {
    let npc_type = &npc_data.npc_type;
    let task = &npc_data.task_description;
    let eye_color = npc_data.eye_color.to_string();
    let height_desc = &npc_data.height_desc;
    let build_desc = &npc_data.build_desc;
    let gender = npc_data.gender.to_string();
    let species = &npc_data.species;
    let hair_color = npc_data.hair_color.to_string();
    let hair_style = npc_data.hair_style.to_string();
    let quirk_emotional = npc_data.quirk_emotional.to_string();
    let quirk_physical = npc_data.quirk_physical.to_string();
    let notable_attribute_positive = npc_data.notable_attribute_positive.to_string();
    let notable_attribute_negative = npc_data.notable_attribute_negative.to_string();

    let mut text_block = format!("{npc_type} {task} \n");

    // --- calc and print encounter_chance_timeslots
    let ect: Vec<crate::narrative_time_manager::ntm::TimeSlot> = npc_data.encounter_slots.clone();
    text_block += &format!(
        "_[chance of being at currently present (2d6): 10+ {:?} | 8+ {:?} | 9+ {:?}]_",
        ect[0].name, ect[1].name, ect[3].name
    );

    // --- end calc and print encounter_chance_timeslots

    text_block += &format!(
        "\n {species:?} with {eye_color} eyes and {hair_color} hair in {hair_style} style."
    );
    text_block += &format!("\n They are {height_desc} {build_desc} {gender}.");
    text_block += "\n ";
    text_block += "\n Quirks:";
    if quirk_emotional.is_empty() && quirk_physical.is_empty() {
        text_block += "\n (none)"
    };
    if !quirk_emotional.is_empty() {
        text_block += &format!("\n  + {quirk_emotional}");
    };
    if !quirk_physical.is_empty() {
        text_block += &format!("\n  + {quirk_physical}");
    };
    text_block += "\n ";

    text_block += "\n Notable Attributes:";
    if notable_attribute_positive.is_empty() && notable_attribute_negative.is_empty() {
        text_block += "\n (none)"
    };

    if !notable_attribute_positive.is_empty() {
        text_block += &format!("\n  + {notable_attribute_positive}");
    };
    if !notable_attribute_negative.is_empty() {
        text_block += &format!("\n  + {notable_attribute_negative}");
    };
    text_block += "\n -----  -----  -----";

    text_block.to_string()
}
