use std::sync::Arc;
use async_trait::async_trait;
use bcrypt::{hash, verify};
use diesel::dsl::not;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use uuid::Uuid;
use crate::account::entity::account::Account;
use crate::account::entity::account::accounts::password;

use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;

use crate::account::service::account_service::AccountService;

use crate::account::service::request::account_session_logout_request::AccountSessionLogoutRequest;
use crate::account::service::request::account_session_login_request::AccountSessionLoginRequest;

use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::account::service::request::account_logout_request::AccountLogoutRequest;
use crate::account::service::request::account_delete_request::AccountDeleteRequest;
use crate::account::service::request::account_modify_request::AccountModifyRequest;
use crate::account::service::request::account_login_request::AccountLoginRequest;

use crate::account::service::response::account_register_response::AccountRegisterResponse;
use crate::account::service::response::account_logout_response::AccountLogoutResponse;
use crate::account::service::response::account_delete_response::AccountDeleteResponse;
use crate::account::service::response::account_modify_response::AccountModifyResponse;
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;

use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::request_generator::session_request_generator::create_session_logout_request;


pub struct AccountServiceImpl {
    repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl AccountServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {

        AccountServiceImpl {
            repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountServiceImpl::new(
                            AccountRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    async fn account_register(&self, account_register_request: AccountRegisterRequest) -> AccountRegisterResponse {
        println!("AccountServiceImpl: account_register()");

        let account_repository = self.repository.lock().await;
        let account = account_register_request.to_account().unwrap();
        // 중복 확인 작업
        if account_repository.find_by_user_id(account.user_id()).await.unwrap().is_some() {
            // 중복 아이디 존재하는 경우
            return AccountRegisterResponse::new(false)
        }
        // 중복 아이디 존재하지 않는 경우 계정 저장
        let result = account_repository.save(account_register_request.to_account().unwrap()).await;
        if result.is_ok() {
            return AccountRegisterResponse::new(true)
        }
        return AccountRegisterResponse::new(false)
    }

    async fn account_login(&self, account_login_request: AccountLoginRequest) -> AccountLoginResponse {
        println!("AccountServiceImpl: account_login()");

        let account_repository = self.repository.lock().await;
        let account = account_login_request.to_account().unwrap();
        if let Some(found_account) = account_repository.find_by_user_id(account.user_id()).await.unwrap() {
            // 비밀번호 매칭 확인
            if verify(&account_login_request.password(), &found_account.password()).unwrap() {
                // 로그인 성공
                let redis_token = Uuid::new_v4();
                let mut redis_repository_gaurd = self.redis_in_memory_repository.lock().await;
                redis_repository_gaurd.set_with_expired_time(&*redis_token.to_string(), &found_account.id.to_string(), Some(3600)).await;

                return AccountLoginResponse::new(redis_token.to_string());
            } else {
                // 비밀번호 불일치 - 로그인 실패
                eprintln!("Password mismatch for user_id: {}", account.user_id());
                eprintln!("Password mismatch for password: {}", account.password());
                eprintln!("login_request password: {}", account_login_request.password());

                // Output hashed password stored in the database
                println!("Stored hashed password: {}", found_account.password());
                println!("Generated hashed password: {}", hash(&account_login_request.password(), 12).unwrap());
            }
        } else {
            // 계정이 없음 - 로그인 실패
            eprintln!("Account not found for user_id: {}", account.user_id());
        }

        // 로그인 실패 시 기본 응답 반환
        AccountLoginResponse::new("".to_string())
    }

    async fn account_logout(&self, account_logout_request: AccountLogoutRequest) -> AccountLogoutResponse {
        println!("AccountServiceImpl: account_logout()");

        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_unique_id = redis_repository_guard.get(account_logout_request.get_session_id()).await.unwrap();

        if let Some(id) = account_unique_id {
            redis_repository_guard.del(account_logout_request.get_session_id()).await;
            return AccountLogoutResponse::new(true)
        }
        return AccountLogoutResponse::new(false)
    }

    // TODO: Session Domain 혹은 Authentication Domain 을 별도로 구성하는 것이 더 좋을 것이다.
    async fn account_session_login(&self, account_session_login_request: AccountSessionLoginRequest) -> AccountLoginResponse {
        println!("AccountServiceImpl: account_session_login()");

        let mut redis_repository_gaurd = self.redis_in_memory_repository.lock().await;
        let account_unique_id = redis_repository_gaurd.get(account_session_login_request.get_session_id()).await;

        if let Some(id) = account_unique_id {
            AccountLoginResponse::new(account_session_login_request.get_session_id().to_string())
        } else {
            AccountLoginResponse::new("".to_string())
        }
    }

    async fn account_session_logout(&self, account_session_logout_request: AccountSessionLogoutRequest) -> AccountLogoutResponse {
        println!("AccountServiceImpl: account_session_logout()");

        let mut redis_repository_gaurd = self.redis_in_memory_repository.lock().await;
        let account_unique_id = redis_repository_gaurd.get(account_session_logout_request.get_session_id()).await;

        if let Some(id) = account_unique_id {
            redis_repository_gaurd.del(account_session_logout_request.get_session_id()).await;
            return AccountLogoutResponse::new(true)
        }
        return AccountLogoutResponse::new(false)
    }

    async fn account_delete(&self, account_delete_request: AccountDeleteRequest) -> AccountDeleteResponse {
        println!("AccountServiceImpl: account_delete()");

        let account_repository = self.repository.lock().await;
        let account = account_delete_request.to_account().unwrap();
        if let Some(found_account) = account_repository.find_by_user_id(account.user_id()).await.unwrap() {
            // 비밀번호 매칭 확인
            if verify(&account_delete_request.password(), &found_account.password()).unwrap() {
                // 비밀번호 일치 확인

                let result = account_repository.delete(account_delete_request.to_account().unwrap()).await;
                if result.is_ok() {
                    return AccountDeleteResponse::new(true)
                }
                return AccountDeleteResponse::new(false)
            }
            return AccountDeleteResponse::new(false)
        }
        return AccountDeleteResponse::new(false)
    }

    async fn account_modify(&self, account_modify_request: AccountModifyRequest) -> AccountModifyResponse {
        println!("AccountServiceImpl: account_modify()");

        let account_repository = self.repository.lock().await;
        let account = account_modify_request.to_account().unwrap();
        // Database accounts tables 에서 계정 찾기
        if let Some(found_account) = account_repository.find_by_user_id(account.user_id()).await.unwrap() {
            // 비밀번호 매칭 확인
            if verify(&account_modify_request.password(), &found_account.password()).unwrap() {
                let result = account_repository.update(account_modify_request.to_account().unwrap(), account_modify_request.new_password()).await;
                if result.is_ok() {
                    return AccountModifyResponse::new(true)
                }
                return AccountModifyResponse::new(false)
            }
            eprintln!("The password does not match.");
            return AccountModifyResponse::new(false)
        }
        // 계정이 없음 - 로그인 실패
        eprintln!("Account not found for user_id: {}", account.user_id());
        return AccountModifyResponse::new(false)
    }
}
