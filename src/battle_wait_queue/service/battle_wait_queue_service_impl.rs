use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;
use crate::battle_ready_monitor::repository::battle_ready_monitor_repository::BattleReadyMonitorRepository;
use crate::battle_ready_monitor::repository::battle_ready_monitor_repository_impl::BattleReadyMonitorRepositoryImpl;

use crate::battle_wait_queue::repository::battle_wait_queue_repository::BattleWaitQueueRepository;
use crate::battle_wait_queue::repository::battle_wait_queue_repository_impl::BattleWaitQueueRepositoryImpl;
use crate::battle_wait_queue::service::battle_wait_queue_service::BattleWaitQueueService;
use crate::battle_wait_queue::service::request::battle_wait_queue_request::BattleWaitQueueRequest;
use crate::battle_wait_queue::service::response::battle_wait_queue_response::BattleWaitQueueResponse;
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct BattleWaitQueueServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    battle_ready_monitor_repository: Arc<AsyncMutex<BattleReadyMonitorRepositoryImpl>>,
    battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
}

impl BattleWaitQueueServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               battle_ready_monitor_repository: Arc<AsyncMutex<BattleReadyMonitorRepositoryImpl>>,
               battle_wait_queue_repository: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
            ) -> Self {

        BattleWaitQueueServiceImpl {
            redis_in_memory_repository,
            battle_ready_monitor_repository,
            battle_wait_queue_repository,
            match_waiting_timer_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleWaitQueueServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleWaitQueueServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleWaitQueueServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            BattleReadyMonitorRepositoryImpl::get_instance(),
                            BattleWaitQueueRepositoryImpl::get_instance(),
                            MatchWaitingTimerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn parse_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32")
    }
}

#[async_trait]
impl BattleWaitQueueService for BattleWaitQueueServiceImpl {

    async fn enqueue_player_id_to_wait_queue(&self, battle_wait_queue_request: BattleWaitQueueRequest) -> BattleWaitQueueResponse {
        println!("BattleWaitQueueServiceImpl: enqueue_player_id_to_wait_queue()");

        let account_unique_id = self.parse_account_unique_id(battle_wait_queue_request.get_session_id()).await;
        // let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        // let account_unique_id_option_string = redis_in_memory_repository.get(battle_wait_queue_request.get_session_id()).await;
        // let account_unique_id_string = account_unique_id_option_string.unwrap();
        // let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");

        let battle_wait_queue_repository = self.battle_wait_queue_repository.lock().await;
        // let battle_wait_queue_repository = self.battle_wait_queue_repository.lock().await;

        let mut match_waiting_timer_repository = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository.set_match_waiting_timer(account_unique_id).await;

        let mut battle_ready_monitor_repository = self.battle_ready_monitor_repository.lock().await;
        battle_ready_monitor_repository.save_battle_account_hash(account_unique_id, BattleReadyStatus::WAIT).await;

        let response = battle_wait_queue_repository.enqueue_player_id_for_wait(account_unique_id).await;

        if response.is_ok() {
            return BattleWaitQueueResponse::new(true)
        }

        return BattleWaitQueueResponse::new(false)
    }

    // async fn enqueue_player_id_to_ready_queue(&self, account_unique_id: i32) -> BattleMatchResponse {
    //     println!("BattleRoomServiceImpl: enqueue_player_id_to_ready_queue()");
    //
    //     let battle_room_ready_queue_repository = self.battle_room_ready_queue_repository.lock().await;
    //     battle_room_ready_queue_repository.enqueue_player_id_for_ready(account_unique_id).await;
    //
    //     BattleMatchResponse::new(false)
    // }
}