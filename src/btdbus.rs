///////////////////////////////////////////////////////////////////////////////
// NAME:            btdbus.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Implementation of the D-Bus interface
//
// CREATED:         09/05/2021
//
// LAST EDITED:     09/05/2021
////

use tokio::time::sleep;
use std::time::Duration;
use dbus_crossroads::IfaceBuilder;

use crate::hello;

pub fn interface<T: Send + 'static>(builder: &mut IfaceBuilder<T>) {
    builder.signal::<(String,), _>("HelloHappened", ("sender",));
    builder.method_with_cr_async(
        "Hello", ("name",), ("reply",),
        |mut ctx, cr, (name,): (String,)| {
            let hello:&mut hello::Hello = cr.data_mut(ctx.path()).unwrap();
            println!("Incoming hello call from {}!", name);
            hello.increment();
            let s = format!("Hello {}! This API has been used {} times.",
                            name, hello.get());
            async move {
                sleep(Duration::from_millis(500)).await;
                let signal_msg = ctx.make_signal("HelloHappened", (name,));
                ctx.push_msg(signal_msg);
                ctx.reply(Ok((s,)))
            }
        });
}

///////////////////////////////////////////////////////////////////////////////
