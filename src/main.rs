// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(debug_assertions, windows_subsystem = "windows")]

mod offset;
mod trainer;
use env_logger::Env;
use log::{error, info};
use std::{cell::RefCell, error::Error, rc::Rc, thread};
use windows_sys::{core::*, Win32::UI::WindowsAndMessaging::*};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let trainer = Rc::new(RefCell::new(trainer::TrainerStates::default()));
    let ui = AppWindow::new()?;

    {
        let ui = ui.as_weak().unwrap();
        let trainer = Rc::clone(&trainer);
        ui.on_change_sun_value(move |sun| {
            info!("Sun value changed to: {}", sun);
            match trainer.borrow_mut().edit_sun_value(sun as u32) {
                Ok(_) => {}
                Err(e) => thread::spawn(move || unsafe {
                    MessageBoxW(
                        0 as _,
                        w!("Something wrong"),
                        w!("Trainer Error"),
                        MB_ICONERROR,
                    );
                    error!("Error: {}", e);
                    panic!("{}", "Something wrong".to_string());
                })
                .join()
                .unwrap(),
            }
        });
    }

    {
        let ui = ui.as_weak().unwrap();
        let trainer = Rc::clone(&trainer);
        ui.on_change_coins_value(move |coins| {
            info!("Coins value changed to: {}", coins);
            match trainer.borrow_mut().edit_coin_value(coins as u32) {
                Ok(_) => {}
                Err(e) => thread::spawn(move || unsafe {
                    MessageBoxW(
                        0 as _,
                        w!("Something wrong"),
                        w!("Trainer Error"),
                        MB_ICONERROR,
                    );
                    error!("Error: {}", e);
                    panic!("{}", "Something wrong".to_string());
                })
                .join()
                .unwrap(),
            }
        });
    }

    {
        let ui = ui.as_weak().unwrap();
        let trainer = Rc::clone(&trainer);
        ui.on_activate_auto_sun_collect(move |toggle| {
            info!("Auto collect sun toggled: {}", toggle);
            match trainer.borrow_mut().auto_collect_sun(toggle) {
                Ok(_) => {}
                Err(e) => thread::spawn(move || unsafe {
                    MessageBoxW(
                        0 as _,
                        w!("Something wrong"),
                        w!("Trainer Error"),
                        MB_ICONERROR,
                    );
                    error!("Error: {}", e);
                    panic!("{}", "Something wrong".to_string());
                })
                .join()
                .unwrap(),
            }
        });
    }

    {
        let ui = ui.as_weak().unwrap();
        ui.on_activate_no_plants_cooldown(move |toggle| {
            info!("Auto collect sun toggled: {}", toggle);
            match trainer.borrow_mut().no_cool_down(toggle) {
                Ok(_) => {}
                Err(e) => thread::spawn(move || unsafe {
                    MessageBoxW(
                        0 as _,
                        w!("Something wrong"),
                        w!("Trainer Error"),
                        MB_ICONERROR,
                    );
                    error!("Error: {}", e);
                    panic!("{}", "Something wrong".to_string());
                })
                .join()
                .unwrap(),
            }
        });
    }

    {
        ui.on_visit_developer_github(move || {
            open::that("https://github.com/un4ckn0wl3z").unwrap();
        });
    }

    ui.run()?;

    Ok(())
}
