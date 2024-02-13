use async_trait::async_trait;

use crate::game_field_unit::service::request::add_unit_to_game_field_request::AddUnitToGameFieldRequest;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::apply_instant_death_to_target_unit_index_request::ApplyInstantDeathToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::attach_single_energy_to_unit_index_request::AttachSingleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_special_energy_to_unit_index_request::AttachSpecialEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::find_active_skill_usage_unit_id_by_index_request::FindActiveSkillUsageUnitIdByIndexRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_field_unit_by_index_request::GetCurrentHealthPointOfFieldUnitByIndexRequest;

use crate::game_field_unit::service::response::add_unit_to_game_field_response::AddUnitToGameFieldResponse;
use crate::game_field_unit::service::response::apply_damage_to_target_unit_index_response::ApplyDamageToTargetUnitIndexResponse;
use crate::game_field_unit::service::response::apply_instant_death_to_target_unit_index_response::ApplyInstantDeathToTargetUnitIndexResponse;
use crate::game_field_unit::service::response::attach_single_energy_to_unit_index_response::AttachSingleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attach_multiple_energy_to_unit_index_response::AttachMultipleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attach_special_energy_to_unit_index_response::AttachSpecialEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::find_active_skill_usage_unit_id_by_index_response::FindActiveSkillUsageUnitIdByIndexResponse;
use crate::game_field_unit::service::response::find_target_unit_id_by_index_response::FindTargetUnitIdByIndexResponse;
use crate::game_field_unit::service::response::get_current_health_point_of_field_unit_by_index_response::GetCurrentHealthPointOfFieldUnitByIndexResponse;

#[async_trait]
pub trait GameFieldUnitService {
    async fn add_unit_to_game_field(&mut self, add_unit_to_game_field_request: AddUnitToGameFieldRequest) -> AddUnitToGameFieldResponse;
    async fn attach_energy_to_field_unit_index(&mut self, attach_energy_to_unit_index_request: AttachSingleEnergyToUnitIndexRequest) -> AttachSingleEnergyToUnitIndexResponse;
    async fn attach_multiple_energy_to_field_unit_index(&mut self, attach_multiple_energy_to_unit_index_request: AttachMultipleEnergyToUnitIndexRequest) -> AttachMultipleEnergyToUnitIndexResponse;
    async fn find_target_unit_id_by_index(&mut self, find_target_unit_id_by_index_request: FindTargetUnitIdByIndexRequest) -> FindTargetUnitIdByIndexResponse;
    async fn apply_damage_to_target_unit_index(&mut self, apply_damage_to_target_unit_index_response: ApplyDamageToTargetUnitIndexRequest) -> ApplyDamageToTargetUnitIndexResponse;
    async fn apply_instant_death_to_target_unit_index(&mut self, apply_instant_death_to_target_unit_index_request: ApplyInstantDeathToTargetUnitIndexRequest) -> ApplyInstantDeathToTargetUnitIndexResponse;
    async fn get_current_health_point_of_field_unit_by_index(&self, get_current_health_point_of_field_unit_by_index_request: GetCurrentHealthPointOfFieldUnitByIndexRequest) -> GetCurrentHealthPointOfFieldUnitByIndexResponse;
    async fn attach_special_energy_to_field_unit_index(&mut self, attach_special_energy_to_unit_index_request: AttachSpecialEnergyToUnitIndexRequest) -> AttachSpecialEnergyToUnitIndexResponse;
    async fn find_active_skill_usage_unit_id_by_index(&mut self, find_active_skill_usage_unit_id_by_index_request: FindActiveSkillUsageUnitIdByIndexRequest) -> FindActiveSkillUsageUnitIdByIndexResponse;
}