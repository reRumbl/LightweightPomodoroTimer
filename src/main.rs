#![windows_subsystem = "windows"]  // Preventing console open on windows

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use slint::{Timer, TimerMode};

mod app;
use app::AppState;
use app::POMODORO_DURATION_MS;

slint::include_modules!();


fn main() -> Result<(), slint::PlatformError> {
    // Create ui and timer
    let ui = MainWindow::new()?;
    let timer = Rc::new(Timer::default());
    
    // Create app state with Rc and RefCell
    let app_state = Rc::new(RefCell::new(AppState::new()));

    // Init ui with default values
    ui.set_remaining_seconds(POMODORO_DURATION_MS / 1000);
    ui.set_current_state_text("Work".into());

    // Pointers for Start/Pause button
    let app_state_for_start_pause: Rc<RefCell<AppState>> = app_state.clone();
    let ui_handle_for_start_pause = ui.as_weak();
    let timer_for_start_pause = timer.clone();

    // Handler for Start/Pause button
    ui.on_start_pause_clicked(move || {
        let ui = ui_handle_for_start_pause.unwrap();
        let is_active = ui.get_timer_active();
        ui.set_timer_active(!is_active);

        if is_active {
            timer_for_start_pause.stop();
        } else {
            let app_state_for_tick = app_state_for_start_pause.clone();
            let ui_handle_for_tick = ui_handle_for_start_pause.clone();
            let timer_for_tick = timer_for_start_pause.clone();

            timer_for_start_pause.start(
                TimerMode::Repeated, 
                Duration::from_millis(100), 
                move || {
                    app_state_for_tick
                        .borrow_mut()
                        .tick(&ui_handle_for_tick.unwrap(), &timer_for_tick);
                });
        }
    });

    // Pointers for Reset button
    let app_state_for_reset = app_state.clone();
    let ui_handle_for_reset = ui.as_weak();
    let timer_for_reset = timer.clone();

    // Handler for Reset button
    ui.on_reset_clicked(move || {
        app_state_for_reset
            .borrow_mut()
            .reset_timer(&ui_handle_for_reset.unwrap(), &timer_for_reset);
    });

    // Pointers for Skip button
    let app_state_for_skip = app_state.clone();
    let ui_handle_for_skip = ui.as_weak();
    let timer_for_skip = timer.clone();

    // Handler for Reset button
    ui.on_skip_clicked(move || {
        app_state_for_skip
            .borrow_mut()
            .skip_state(&ui_handle_for_skip.unwrap(), &timer_for_skip);
    });

    ui.run()
}
