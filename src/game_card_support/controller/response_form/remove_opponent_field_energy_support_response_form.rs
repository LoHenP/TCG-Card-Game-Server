use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex::{Opponent};
use crate::notify_player_action_info::service::response::notice_remove_field_energy_of_opponent_response::NoticeRemoveFieldEnergyOfOpponentResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldEnergySupportResponseForm {
    is_success: bool,
    player_field_energy_map: HashMap<PlayerIndex, i32>
}

impl RemoveOpponentFieldEnergySupportResponseForm {
    pub fn new(is_success: bool,
               player_field_energy_map: HashMap<PlayerIndex, i32>) -> Self {
        RemoveOpponentFieldEnergySupportResponseForm {
            is_success,
            player_field_energy_map
        }
    }

    pub fn from_response(notice_use_hand_card_response: NoticeUseHandCardResponse,
                         notice_remove_field_energy_of_opponent_response: NoticeRemoveFieldEnergyOfOpponentResponse)
        -> RemoveOpponentFieldEnergySupportResponseForm {

        RemoveOpponentFieldEnergySupportResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_remove_field_energy_of_opponent_response
                .get_player_field_energy_info()
                .get_player_field_energy_map().clone())
    }

    pub fn default() -> RemoveOpponentFieldEnergySupportResponseForm {

        RemoveOpponentFieldEnergySupportResponseForm::new(
            false,
            HashMap::new())
    }
}