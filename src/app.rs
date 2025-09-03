use slint::{Timer};
use crate::MainWindow;

pub const POMODORO_DURATION_SECS: i32 = 25 * 60;
pub const SHORT_BREAK_DURATION_SECS: i32 = 5 * 60;
pub const LONG_BREAK_DURATION_SECS: i32 = 15 * 60;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PomodoroState {
    Work,
    ShortBreak,
    LongBreak
}


impl PomodoroState {
    fn get_duration(&self) -> i32 {
        match self {
            PomodoroState::Work => POMODORO_DURATION_SECS,
            PomodoroState::ShortBreak => SHORT_BREAK_DURATION_SECS,
            PomodoroState::LongBreak => LONG_BREAK_DURATION_SECS,
        }
    }

    fn to_string(&self) -> slint::SharedString {
        match self {
            PomodoroState::Work => "Work".into(),
            PomodoroState::ShortBreak => "Short Break".into(),
            PomodoroState::LongBreak => "Long Break".into(),
        }
    }
}


pub struct AppState {
    state: PomodoroState,
    pomodoro_cycles: i32
}


impl AppState {
    pub fn new() -> Self {
        Self {
            state: PomodoroState::Work,
            pomodoro_cycles: 0
        }
    }

    pub fn reset_timer(&mut self, ui: &MainWindow, timer: &Timer) {
        timer.stop();
        ui.set_timer_active(false);
        ui.set_remaining_seconds(self.state.get_duration());
    }

    pub fn skip_state(&mut self, ui: &MainWindow, timer: &Timer) {
        timer.stop();
        ui.set_timer_active(false);
        self.next_state_and_update_ui(ui);
    }

    pub fn tick(&mut self, ui: &MainWindow, timer: &Timer) {
        let remaining = ui.get_remaining_seconds();
        if remaining > 0 {
            ui.set_remaining_seconds(remaining - 1);
        } else {
            self.skip_state(ui, timer);
        }
    }

    fn next_state_and_update_ui(&mut self, ui: &MainWindow) {
        if self.state == PomodoroState::Work {
            self.pomodoro_cycles += 1;
            if self.pomodoro_cycles % 4 == 0 {
                self.state = PomodoroState::LongBreak;
            } else {
                self.state = PomodoroState::ShortBreak;
            }
        } else {
            self.state = PomodoroState::Work;
        }
        ui.set_current_state_text(self.state.to_string());
        ui.set_remaining_seconds(self.state.get_duration());
    }
}
