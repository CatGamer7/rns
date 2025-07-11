use super::*;
use std::sync::mpsc::Sender;

#[test]
#[should_panic]
/// Test n_worker range high retriction.
fn invalid_pool_high() {
    Pool::new(1024);
}
#[test]
#[should_panic]
/// Test n_worker range low retriction.
fn invalid_pool_low() {
    Pool::new(0);
}
#[test]
/// Test Pool with numerical computation.
fn pool_work() {
    let (tx, rx) = mpsc::channel();
    let pool = Pool::new(4);
    for i in 0..4 {
        let tx_clone: Sender<i32> = tx.clone();
        pool.execute(
            move || {
                let mut computation = 1;
                for _ in 0..i {
                    computation *= 2;
                }
                tx_clone.send(computation).unwrap();
            }
        );
    }
    drop(tx);
    let mut result = 0;
    for received in rx {
        result += received;
    }
    assert!(result == 15);
}
#[test]
#[ignore]
/// Test concurrent operation manually by stdout 
/// (visible with --nocapture flag). Always succeeds.
fn pool_concurrency() {
    let pool = Pool::new(4);
    for i in 0..4 {
        pool.execute(
            move || {
                println!("I have {i}!");
            }
        );
    }
}
