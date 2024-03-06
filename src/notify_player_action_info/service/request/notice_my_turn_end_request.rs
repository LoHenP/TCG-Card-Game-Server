use std::collections::HashMap;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug)]
pub struct NoticeMyTurnEndRequest {
    opponent_unique_id: i32,
    player_drawn_card_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
    player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
    player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
}

impl NoticeMyTurnEndRequest {
    pub fn new(opponent_unique_id: i32,
               player_drawn_card_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
               player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
               player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
               player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> Self {
        NoticeMyTurnEndRequest {
            opponent_unique_id,
            player_drawn_card_list_map_for_notice,
            player_field_energy_map_for_notice,
            player_field_unit_health_point_map_for_notice,
            player_field_unit_harmful_effect_map_for_notice,
            player_field_unit_death_map_for_notice
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_player_drawn_card_list_map_for_notice(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_drawn_card_list_map_for_notice
    }

    pub fn get_player_field_energy_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_field_energy_map_for_notice
    }

    pub fn get_player_field_unit_health_point_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitHealthPointInfo> {
        &self.player_field_unit_health_point_map_for_notice
    }

    pub fn get_player_field_unit_harmful_effect_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo> {
        &self.player_field_unit_harmful_effect_map_for_notice
    }

    pub fn get_player_field_unit_death_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitDeathInfo> {
        &self.player_field_unit_death_map_for_notice
    }
}