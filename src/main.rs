///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     bluez_player provides a BlueZ agent for accepting incoming
//                  Bluetooth connections and routing media from the BlueZ
//                  stack to the system PCM device.
//
// CREATED:         09/26/2021
//
// LAST EDITED:     09/26/2021
////

use dbus_tokio::connection;
use futures::future;
use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus_crossroads::Crossroads;

mod btdbus;
mod hello;

const BUS_NAME: &'static str = "com.example.dbustest";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (resource, conn) = connection::new_session_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    conn.request_name(BUS_NAME, false, true, false).await?;
    let mut cr = Crossroads::new();
    cr.set_async_support(Some((conn.clone(), Box::new(|x| {
        tokio::spawn(x);
    }))));

    let iface_token = cr.register(BUS_NAME, btdbus::interface);

    cr.insert("/hello", &[iface_token], hello::Hello::new());

    conn.start_receive(
        MatchRule::new_method_call(), Box::new(move |msg, conn| {
            cr.handle_message(msg, conn).unwrap();
            true
        }));

    future::pending::<()>().await;
    unreachable!()
}

///////////////////////////////////////////////////////////////////////////////
