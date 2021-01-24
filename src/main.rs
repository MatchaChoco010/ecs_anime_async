use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use futures::future::{AbortHandle, AbortRegistration, Abortable};
use futures::{pin_mut, select, FutureExt};
use futures_timer::Delay;
use ggez::graphics::Color;

mod app;
use app::components::*;
use app::*;

fn main() -> Result<()> {
    // let mut app = App::new("app", "Orito Itsuki")?;

    // app.load_animation("anim1", "./anim1.json")?;
    // app.play_animation("anim1");
    // app.push((
    //     Name {
    //         name: "circle".to_string(),
    //     },
    //     Renderable::Circle {
    //         radius: 30.0,
    //         color: Color::from_rgb(255, 128, 128),
    //     },
    //     Position {
    //         x: 100.0,
    //         y: 100.0,
    //         z: 5.0,
    //     },
    // ));
    // app.push((
    //     Name {
    //         name: "rect".to_string(),
    //     },
    //     Renderable::Rectangle {
    //         width: 50.0,
    //         height: 30.0,
    //         color: Color::from_rgb(0, 128, 255),
    //     },
    //     Position {
    //         x: 600.0,
    //         y: 100.0,
    //         z: 10.0,
    //     },
    // ));

    // app.run()

    runtime::spawn(async {
        println!("Hi!");
        let handle = runtime::spawn(async {
            Delay::new(Duration::from_secs(1)).await;
            println!("Hoi!");
            1 + 2
        })
        .fuse();
        // let handle2 = runtime::spawn(async {
        //     Delay::new(Duration::from_secs(2)).await;
        //     println!("Hoi2!");
        //     Delay::new(Duration::from_secs(1)).await;
        //     println!("Hoi! Hoi2!");
        //     Delay::new(Duration::from_secs(1)).await;
        //     1 + 2
        // })
        // .fuse();
        // let handle2 = async {
        //     Delay::new(Duration::from_secs(2)).await;
        //     println!("Hoi2!");
        //     Delay::new(Duration::from_secs(1)).await;
        //     println!("Hoi! Hoi2!");
        //     Delay::new(Duration::from_secs(1)).await;
        //     1 + 2
        // }
        // .fuse();
        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        let handle2 = runtime::spawn(Abortable::new(
            async {
                runtime::delay(Duration::from_secs(2)).await;
                println!("Hoi2!");
                runtime::delay(Duration::from_secs(1)).await;
                println!("Hoi! Hoi2!");
                runtime::delay(Duration::from_secs(1)).await;
                1 + 2
            },
            abort_registration,
        ))
        .fuse();
        println!("Foo!");

        pin_mut!(handle);
        pin_mut!(handle2);

        select! {
            x = handle => println!("handle: {}", x),
            x = handle2 => println!("handle2: {:?}", x),
        }
        abort_handle.abort();

        println!("Fooooo!");

        for i in 0..10 {
            println!("{}", i);
            runtime::next_frame().await;
        }
    });

    loop {
        runtime::runtime_update();
        sleep(Duration::from_millis(100));
    }
}
