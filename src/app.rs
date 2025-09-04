use std::time::{ Instant, Duration };
use slint::Timer;
use crate::MainWindow;

pub const POMODORO_DURATION_MS: i32 = 25 * 60 * 1000;
pub const SHORT_BREAK_DURATION_MS: i32 = 5 * 60 * 1000;
pub const LONG_BREAK_DURATION_MS: i32 = 15 * 60 * 1000;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PomodoroState {
    Work,
    ShortBreak,
    LongBreak
}


impl PomodoroState {
    fn get_duration_ms(&self) -> i32 {
        match self {
            PomodoroState::Work => POMODORO_DURATION_MS,
            PomodoroState::ShortBreak => SHORT_BREAK_DURATION_MS,
            PomodoroState::LongBreak => LONG_BREAK_DURATION_MS,
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
    pomodoro_cycles: i32,
    remaining_ms: i32,
    last_tick_time: Option<Instant>
}


impl AppState {
    pub fn new() -> Self {
        Self {
            state: PomodoroState::Work,
            pomodoro_cycles: 0,
            remaining_ms: POMODORO_DURATION_MS,
            last_tick_time: None
        }
    }

    pub fn reset_timer(&mut self, ui: &MainWindow, timer: &Timer) {
        timer.stop();
        ui.set_timer_active(false);
        self.remaining_ms = self.state.get_duration_ms();
        self.update_ui(ui);
        self.last_tick_time = None;
    }

    pub fn skip_state(&mut self, ui: &MainWindow, timer: &Timer) {
        timer.stop();
        ui.set_timer_active(false);
        self.next_state_and_update_ui(ui);
        self.last_tick_time = None;
    }

    pub fn tick(&mut self, ui: &MainWindow, timer: &Timer) {
        let now = Instant::now();
        let elapsed_time = if let Some(last_tick) = self.last_tick_time {
            now.duration_since(last_tick)
        } else {
            Duration::from_millis(0)
        };

        self.remaining_ms -= elapsed_time.as_millis() as i32;
        self.last_tick_time = Some(now);

        if self.remaining_ms <= 0 {
            self.remaining_ms = 0;
            self.skip_state(ui, timer);
        }

        self.update_ui(ui);
    }

    fn update_ui(&self, ui: &MainWindow) {
        let remaining_secs = self.remaining_ms / 1000;
        ui.set_remaining_seconds(remaining_secs);
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
        self.remaining_ms = self.state.get_duration_ms();
        self.update_ui(ui);
    }
}
