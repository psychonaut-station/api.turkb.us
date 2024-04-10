use rocket::{routes, Build, Rocket};

mod test_merges;
mod v1;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![test_merges::recent_test_merges]);
    v1::mount(rocket)
}
