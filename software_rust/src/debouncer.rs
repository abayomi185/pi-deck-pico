pub struct Debouncer {
    current_state: bool,
    pub stabilised_state: bool,
    last_transition_time: u32,
    stability_period: u32,
}

impl Debouncer {
    pub fn new(stability_period: u32) -> Self {
        Debouncer {
            current_state: false,
            stabilised_state: false,
            last_transition_time: 0,
            stability_period,
        }
    }

    pub fn update(&mut self, current_time: u32, new_state: bool) {
        if self.current_state != new_state {
            self.current_state = new_state;
            self.last_transition_time = current_time;
        }

        if current_time - self.last_transition_time >= self.stability_period {
            self.stabilised_state = self.current_state;
        }
    }

    // pub fn is_stable(&self) -> bool {
    //     self.current_state == self.stabilised_state
    // }
}
