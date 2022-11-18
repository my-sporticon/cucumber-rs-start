use tokio::main;
use tokio::time::{sleep, Duration};
use cucumber::{given, when, then, World};

// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

#[derive(Debug, World)]
// Accepts both sync/async and fallible/infallible functions.
#[world(init = Self::new)]
pub struct AnimalWorld {
    cat: Cat,
}

impl AnimalWorld {
    fn new() -> Self {
        Self {
            cat: Cat { hungry: true }
        }
    }
}

#[given(expr = "a {word} cat")]
async fn hungry_cat(world: &mut AnimalWorld, state: String) {
    sleep(Duration::from_secs(2)).await;
    match state.as_str() {
        "hungry" =>  world.cat.hungry = true,
        "satiated" =>  world.cat.hungry = false,
        s => panic!("expected 'hungry' or 'satiated', found: {s}"),
    }
}

#[when("I feed the cat")]
async fn feed_cat(world: &mut AnimalWorld) {
    sleep(Duration::from_secs(2)).await;
    world.cat.feed();
}

#[then("the cat is not hungry")]
async fn cat_is_fed(world: &mut AnimalWorld) {
    sleep(Duration::from_secs(2)).await;
    assert!(!world.cat.hungry);
}

#[tokio::main]
async fn main() {
    AnimalWorld::run("tests/features/book/animal.feature").await;
}