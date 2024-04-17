
use rocket::serde::json::Json;
use rocket::State;
use rocket::serde::{ Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: usize,
    pub username: String,
    pub email: String,
}

// In-memory storage for accounts
pub struct AccountStorage {
    pub accounts: Mutex<HashMap<usize, Account>>,
}

// Initialize storage with default data
impl AccountStorage {
    pub fn new() -> Self {
        let mut accounts = HashMap::new();

        AccountStorage {
            accounts: Mutex::new(accounts),
        }
    }
}

#[post("/", format = "json", data = "<account>")]
pub fn create(account: Json<Account>, storage: &State<AccountStorage>) -> Json<Account> {
    let mut accounts = storage.accounts.lock().unwrap();
    let new_id = accounts.len() + 1;
    let mut new_account = account.into_inner();
    new_account.id = new_id;
    accounts.insert(new_id, new_account.clone());
    Json(new_account)
}

#[get("/<id>")]
pub fn read(id: usize, storage: &State<AccountStorage>) -> Option<Json<Account>> {
    storage.accounts.lock().unwrap().get(&id).cloned().map(Json)
}

#[put("/<id>", format = "json", data = "<account>")]
pub fn update(id: usize, account: Json<Account>, storage: &State<AccountStorage>) -> Option<Json<Account>> {
    let mut accounts = storage.accounts.lock().unwrap();
    accounts.get_mut(&id).map(|existing_account| {
        existing_account.username = account.username.clone();
        existing_account.email = account.email.clone();
        Json(existing_account.clone())
    })
}

#[delete("/<id>")]
pub fn delete(id: usize, storage: &State<AccountStorage>) -> Option<Json<Account>> {
    storage.accounts.lock().unwrap().remove(&id).map(Json)
}

pub fn endpoints() {
    crate::register!("/", routes![
        create,
        read,
        update,
        delete
    ]);
}