use std::cell::RefCell;
use std::rc::Rc;

use rand::{Rng, SeedableRng};

use iota_streams_app_channels::api::tangle::{Author, BucketTransport, ChannelType, Subscriber};

async fn main() {
    println!("STARTING STREAMS IN ESP-32 QEMU!");

    let transport = Rc::new(RefCell::new(BucketTransport::new()));
    println!("NEW BUCKET TRANSPORT INSTANTIATED");
    print_free_heap();

    let seed = hex::encode(rand::rngs::StdRng::from_entropy().gen::<[u8; 32]>());
    println!("GENERATED SEED '{}' for author", seed);
    print_free_heap();

    let mut author = Author::new(&seed, ChannelType::MultiBranch, transport.clone());
    println!("NEW AUTHOR INSTANTIATED: {}", author);
    print_free_heap();

    let seed = hex::encode(rand::rngs::StdRng::from_entropy().gen::<[u8; 32]>());
    println!("GENERATED SEED '{}' for subscriber", seed);
    print_free_heap();

    let mut subscriber = Subscriber::new(&seed, transport.clone());
    println!("NEW SUBSCRIBER INSTANTIATED: {}", subscriber);
    print_free_heap();

    let announcement_link = author.send_announce().await.unwrap();
    println!("ANNOUNCEMENT MESSAGE SENT: {:?}", announcement_link);
    print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);

    subscriber
        .receive_announcement(&announcement_link)
        .await
        .unwrap();
    println!("SUBSCRIBER RECEIVED ANNOUNCEMENT LINK OOB AND RETRIEVED ANNOUNCEMENT MESSAGE");
    print_free_heap();

    let subscription_link = subscriber.send_subscribe(&announcement_link).await.unwrap();
    println!(
        "SUBSCRIBER SUBSCRIPTION MESSAGE SENT: {:?}",
        subscription_link
    );
    print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);

    author.receive_subscribe(&subscription_link).await.unwrap();
    println!("AUTHOR RECEIVED SUBSCRIPTION MESSAGE");
    author.sync_state().await;
    println!("AUTHOR SYNC'ED STATE");
    print_free_heap();

    let (keyload_link, _) = author
        .send_keyload_for_everyone(&announcement_link)
        .await
        .unwrap();
    println!("AUTHOR SENT KEYLOAD TO EVERYONE: {:?}", keyload_link);
    print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);

    let (signed_packet_link, _) = author
        .send_signed_packet(
            &keyload_link,
            (&b"public payload".to_vec()).into(),
            (&b"encryted payload".to_vec()).into(),
        )
        .await
        .unwrap();
    println!("SIGNED PACKET SENT: {:?}", signed_packet_link);
    print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);

    let (signed_packet_link, _) = author
        .send_signed_packet(
            &signed_packet_link,
            (&b"public payload 2".to_vec()).into(),
            (&b"encryted payload 2".to_vec()).into(),
        )
        .await
        .unwrap();
    println!("SIGNED PACKET SENT: {:?}", signed_packet_link);
    print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);

    let subscriber_msgs = subscriber.fetch_all_next_msgs().await;
    println!("MESSAGES RECEIVED BY SUBSCRIBER:\n{:#?}", subscriber_msgs);
    print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);
}

pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use lock_api::{GuardSend, RawMutex};

// 1. Define our raw lock type
pub struct RawNoLock;

// 2. Implement RawMutex for this type
unsafe impl RawMutex for RawNoLock {
    const INIT: Self = Self;

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}
}

// 3. Export the wrappers. This are the types that your users will actually use.
pub type NoMutex<T> = lock_api::Mutex<RawNoLock, T>;
pub type NoMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawNoLock, T>;

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut executor: embedded_executor::AllocExecutor<RawNoLock, embedded_executor::SpinSleep> =
        embedded_executor::AllocExecutor::new();
    executor.spawn(main());
    executor.run()
}

fn print_e<E>(e: E) -> E
where
    E: std::fmt::Display,
{
    println!("Error: {:#}", e);
    e
}

fn print_free_heap() {
    unsafe {
        sys::print_free_heap();
    }
}
