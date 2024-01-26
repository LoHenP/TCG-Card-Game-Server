use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::deck_card::repository::deck_card_repository::DeckCardRepository;

use crate::deck_card::repository::deck_card_repository_impl::DeckCardRepositoryImpl;
use crate::deck_card::service::deck_card_service::DeckCardService;
use crate::deck_card::service::request::deck_configuration_request::DeckConfigurationRequest;
use crate::deck_card::service::response::deck_configuration_response::DeckConfigurationResponse;

pub struct DeckCardServiceImpl {
    repository: Arc<AsyncMutex<DeckCardRepositoryImpl>>
}

impl DeckCardServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<DeckCardRepositoryImpl>>) -> Self {
        DeckCardServiceImpl {
            repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<DeckCardServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<DeckCardServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        DeckCardServiceImpl::new(
                            DeckCardRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl DeckCardService for DeckCardServiceImpl {
    async fn deck_configuration_register(&self, deck_configuration_request: DeckConfigurationRequest) -> DeckConfigurationResponse {
        let deck_card_repository = self.repository.lock().await;
        let result = deck_card_repository.save(deck_configuration_request).await;
        match result {
            Ok(success_message) => {
                DeckConfigurationResponse::new(true, success_message)
            }
            Err(error_message) => {
                DeckConfigurationResponse::new(false, error_message)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_deck_config_save() {
        let deck_card_service_mutex = DeckCardServiceImpl::get_instance();
        let deck_card_service_mutex_guard = deck_card_service_mutex.lock().await;

        let deck_id = 8;

        // let card_id_list_very_long = [1, 1, 1, 2, 2, 3, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7,
        //     8, 8, 9, 9, 9, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 16, 17, 17, 17];
        // let card_id_list_too_many_duplicated_cards = [1, 1, 1, 1, 2, 2, 3, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7,
        //     8, 8, 9, 9, 9, 11, 11, 11, 12, 12, 12];
        let test_card_id_list = [1, 1, 1, 2, 2, 3, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7,
            8, 8, 9, 9, 9, 11, 11, 11, 12, 12, 12];

        let mut card_vec = Vec::new();
        for id in test_card_id_list {
            card_vec.push(id);
        }
        let deck_config_request = DeckConfigurationRequest::new(deck_id, card_vec);

        let result = deck_card_service_mutex_guard.deck_configuration_register(deck_config_request).await;
        println!("is_success: {}", result.get_is_success());
        println!("message: {}", result.get_message());
    }

    #[tokio::test]
    #[cfg(not(feature = "deck_card_test"))]
    async fn dummy_test() {
        assert!(true);
    }
}