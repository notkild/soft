extern crate soft_core;

use soft_core::sync::SyncCacher;

fn main() {
    let cacher = SyncCacher::new("sync_cacher_example").unwrap();
    cacher.build_cache(".").unwrap();
    let cache = cacher.query_cache(".").unwrap();
    for (path, time) in cache {
        println!("{} {}", path, time);
    }
}
