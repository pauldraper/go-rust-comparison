use dashmap::DashMap;
use smallvec::{smallvec, SmallVec};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let num_workers = 100;
    let items_per_worker = 100000;

    let data: Arc<DashMap<i32, SmallVec<[i32; 4]>>> = Arc::new(DashMap::new());
    let mut handles = vec![];

    for worker_id in 0..num_workers {
        let data = Arc::clone(&data);
        handles.push(tokio::spawn(async move {
            for j in 0..items_per_worker {
                data.entry(j)
                    .or_insert_with(|| smallvec![])
                    .value_mut()
                    .push(worker_id);
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
