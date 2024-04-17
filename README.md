# Rocket.rs Framework Controller Usage Example

Here is an example project where I tried to achieve grouping endpoints via controller structure so that it's easier to register them on routes later and keeping endpoints in meaningful groups (as this is one of the industry standards).

There are a few challanges in Rust to complete this task so one of the first challlanges is to have globally a accesible store to dynamically register our routes. I have solved this problem by using lazy_static package and macro.

```
lazy_static! {
    static ref CONTROLLERS_MAP: Mutex<HashMap<&'static str, Vec<Route>>> = Mutex::new(HashMap::new());
}
```

Here I created a HashMap that can live throughout the application accepting an str as a route prefix and the list of endpoints (Route) in a Vec. I had to use Mutex since it will be accessed from everywhere and need a way to ensure on rust language.

Later, I made up an fn name in each controller called `pub fn endpoints()` where you define your index of endpoints like this below. We still use the routes! macro to create our Route objects for convenience from the original tutorial. This endpoints functions will be called later on the `main.rs` in which we will initialize the rocket and mount routes.

```
pub fn endpoints() {
    crate::register!("/", routes![
        create,
        read,
        update,
        delete
    ]);
}
```

The macro `register` is created by me to solve the problem of pushing all endpoints which can also be achieved by directly calling the generated code but it's not clean and repetitive so I prefered this way.

```
macro_rules! register {
    ( $($route:expr, $x:expr ),* ) => {
        $(
            crate::CONTROLLERS_MAP.lock().unwrap().insert($route, $x);
        )*
    };
}
```

The folder structure I prefered is like `controllers` mod.rs inside to expose modules. Then controller files like `user.rs` inside the folder. And standard route definitions of rocket framework

```
#[get("/<id>")]
pub fn read(id: usize, storage: &State<AccountStorage>) -> Option<Json<Account>> {
    storage.accounts.lock().unwrap().get(&id).cloned().map(Json)
}
```

Then, when we prepared all, here is my main looks like basically consists of very similar steps

1. Initialize rocket
2. Initialize all `endpoints` methods on the controllers (this could have been also achieved via loop to prevent repetition if multiple controllers are present )
3. Then mounting phase where we iterate our map and mount the routes one by one

```
fn rocket() -> _ {
    let mut app = rocket::build().manage(AccountStorage::new());
    controllers::user::endpoints();
    for (route,  routes) in CONTROLLERS_MAP.lock().unwrap().iter() {
        app = app.mount(*route, routes.clone());
    }
    app
}
```
