use core::mem::MaybeUninit;
use std::time::Duration;
use crossbeam_channel::Receiver;
use trayicon::*;
use winapi::um::winuser;
use workctl::sync_flag::SyncFlagRx;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Events {
    ClickTrayIcon,
    DoubleClickTrayIcon,
    Exit,
    Item1,
    Item2,
    Item3,
    Item4,
    CheckItem1,
    SubItem1,
    SubItem2,
    SubItem3,
}

pub struct TrayIcon {
    tray_icon: trayicon::TrayIcon<Events>,
    pub message_channel: crossbeam_channel::Receiver<Events>,
}

impl TrayIcon {
    pub fn setup(icon: &[u8]) -> TrayIcon {
        let (s, r) = crossbeam_channel::unbounded();
        let icon1 = include_bytes!("./../icon1.ico");
        let icon2 = include_bytes!("./../icon2.ico");

        let second_icon = Icon::from_buffer(icon2, None, None).unwrap();
        let first_icon = Icon::from_buffer(icon1, None, None).unwrap();

        // Needlessly complicated tray icon with all the whistles and bells
        let tray_icon = TrayIconBuilder::new()
            .sender_crossbeam(s)
            .icon_from_buffer(icon1)
            .tooltip("Cool Tray 👀 Icon")
            .on_click(Events::ClickTrayIcon)
            .on_double_click(Events::DoubleClickTrayIcon)
            .menu(
                MenuBuilder::new()
                    .item("Item 3 Replace Menu 👍", Events::Item3)
                    .item("Item 2 Change Icon Green", Events::Item2)
                    .item("Item 1 Change Icon Red", Events::Item1)
                    .separator()
                    .checkable("This is checkable", true, Events::CheckItem1)
                    .submenu(
                        "Sub Menu",
                        MenuBuilder::new()
                            .item("Sub item 1", Events::SubItem1)
                            .item("Sub Item 2", Events::SubItem2)
                            .item("Sub Item 3", Events::SubItem3),
                    )
                    .with(MenuItem::Item {
                        name: "Item Disabled".into(),
                        disabled: true, // Disabled entry example
                        id: Events::Item4,
                        icon: None,
                    })
                    .separator()
                    .item("E&xit", Events::Exit),
            )
            .build()
            .unwrap();

        TrayIcon {
            tray_icon,
            message_channel: r,
        }
    }

    pub fn check_tray_icon_messages(&self) {
        unsafe {
            let mut msg = MaybeUninit::uninit();
            let bret = winuser::PeekMessageA(msg.as_mut_ptr(), 0 as _, 0, 0, 1);

            if bret > 0 {
                winuser::TranslateMessage(msg.as_ptr());
                winuser::DispatchMessageA(msg.as_ptr());
            } else {
                return;
            }
        }
    }

    pub fn handle_tray_messages(message_channel: &Receiver<Events>) {
        let message = message_channel.recv_timeout(Duration::from_millis(10));
        match message {
            Err(_) => return,
            Ok(message) => {
                match message {
                    Events::DoubleClickTrayIcon => {
                        println!("Double click");
                    }
                    Events::ClickTrayIcon => {
                        println!("Single click");
                    }
                    Events::Exit => {
                        println!("Please exit");
                        todo!()
                    }
                    Events::Item1 => {
                        println!("item1");
                        //tray_icon.set_icon(&second_icon).unwrap();
                    }
                    Events::Item2 => {
                        println!("item2");
                        //tray_icon.set_icon(&first_icon).unwrap();
                    }
                    Events::Item3 => {
                        println!("item3");
                        /*tray_icon
                        .set_menu(
                            &MenuBuilder::new()
                                .item("New menu item", Events::Item1)
                                .item("Exit", Events::Exit),
                        )
                        .unwrap();*/
                    }
                    e => {
                        println!("{:?}", e);
                    }
                }
            }
        }
    }
}
