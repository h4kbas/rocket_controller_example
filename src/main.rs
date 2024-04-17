#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;


mod controllers;

use std::{collections::HashMap, sync::Mutex};
use controllers::user::AccountStorage;
use rocket::Route; 


lazy_static! {
    static ref CONTROLLERS_MAP: Mutex<HashMap<&'static str, Vec<Route>>> = Mutex::new(HashMap::new()); 
}

macro_rules! register {
    ( $($route:expr, $x:expr ),* ) => {
        $(
            crate::CONTROLLERS_MAP.lock().unwrap().insert($route, $x);
        )*
    };
}

pub(crate) use register;



#[launch]
fn rocket() -> _ {
    let mut app = rocket::build().manage(AccountStorage::new());
    controllers::user::endpoints();
    for (route,  routes) in CONTROLLERS_MAP.lock().unwrap().iter() {
        app = app.mount(*route, routes.clone());
    }
    app
}