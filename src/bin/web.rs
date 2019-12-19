use rocket_lamb::RocketExt;
use photography::create_rocket;

fn main() {
    create_rocket()
        .lambda()
        .launch();
}