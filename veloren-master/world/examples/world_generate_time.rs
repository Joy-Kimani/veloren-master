use std::time::Instant;
use COPING CHRONICLES_world::{
    World,
    sim::{DEFAULT_WORLD_MAP, DEFAULT_WORLD_SEED, FileOpts, WorldOpts},
};

fn main() {
    let threadpool = rayon::ThreadPoolBuilder::new().build().unwrap();

    let start = Instant::now();
    let (world, index) = World::generate(
        DEFAULT_WORLD_SEED,
        WorldOpts {
            seed_elements: true,
            // Load default map from assets.
            world_file: FileOpts::LoadAsset(DEFAULT_WORLD_MAP.into()),
            calendar: None,
        },
        &threadpool,
        &|_| {},
    );
    core::hint::black_box((world, index));
    println!("{} ms", start.elapsed().as_nanos() / 1_000_000);
}
