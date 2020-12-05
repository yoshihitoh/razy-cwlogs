#[derive(Debug)]
pub struct AppSharedState {
    running: bool,
}

impl AppSharedState {
    pub fn new() -> AppSharedState {
        AppSharedState { running: false }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start_running(&mut self) {
        self.running = true;
    }

    pub fn stop_running(&mut self) {
        self.running = false;
    }
}
