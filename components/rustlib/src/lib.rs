use std::cell::RefCell;
use std::rc::Rc;

use rand::{Rng, SeedableRng};

use iota_streams_app_channels::api::tangle::{Author, BucketTransport, ChannelType, Subscriber};

fn main() {
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

    // let mut subscriber = Subscriber::new(&seed, transport.clone());
    // println!("NEW SUBSCRIBER INSTANTIATED: {}", subscriber);
    // print_free_heap();

    let announcement_link = author.send_announce().unwrap();
    println!("ANNOUNCEMENT MESSAGE SENT: {:?}", announcement_link);
    print_free_heap();

    println!("TRANSPORT STATE:\n{:#?}", transport);

    // let _ = subscriber.receive_announcement(&announcement_link);
    // println!("SUBSCRIBER RECEIVED ANNOUNCEMENT LINK OOB AND RETRIEVED ANNOUNCEMENT");
    // print_free_heap();

    // let subscription_link = subscriber.send_subscribe(&announcement_link).unwrap();
    // println!("SUBSCRIBER SUBSCRIPTION MESSAGE SENT: {:?}", subscription_link);
    // print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);

    // author.sync_state();
    // println!("AUTHOR SYNC'ED STATE, RECEIVED SUBSCRIPTION MESSAGE");
    // print_free_heap();

    let (keyload_link, _) = author
        .send_keyload_for_everyone(&announcement_link)
        .unwrap();
    println!("AUTHOR SENT KEYLOAD TO EVERYONE: {:?}", keyload_link);
    print_free_heap();

    println!("TRANSPORT STATE:\n{:#?}", transport);

    let (signed_packet_link, _) = author
        .send_signed_packet(
            &keyload_link,
            (&b"public payload".to_vec()).into(),
            (&b"encryted payload".to_vec()).into(),
        )
        .unwrap();
    println!("SIGNED PACKET SENT: {:?}", signed_packet_link);
    print_free_heap();

    println!("TRANSPORT STATE:\n{:#?}", transport);

    let (signed_packet_link, _) = author
        .send_signed_packet(
            &signed_packet_link,
            (&b"public payload 2".to_vec()).into(),
            (&b"encryted payload 2".to_vec()).into(),
        )
        .unwrap();
    println!("SIGNED PACKET SENT: {:?}", signed_packet_link);
    print_free_heap();

    println!("TRANSPORT STATE:\n{:#?}", transport);

    // let subscriber_msgs = subscriber.fetch_all_next_msgs();
    // println!("MESSAGES RECEIVED BY SUBSCRIBER:\n{:#?}", subscriber_msgs);
    // print_free_heap();

    // println!("TRANSPORT STATE:\n{:#?}", transport);
}

pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[no_mangle]
pub extern "C" fn rust_main() {
    main()
}

fn print_e<E>(e: E) -> E where E: std::fmt::Display {
    println!("Error: {:#}", e);
    e
}

fn print_free_heap() {
    unsafe {
        sys::print_free_heap();
    }
}