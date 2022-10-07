// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug)]

enum LuggageState {
    CheckIn,
    OnLoading,
    OffLoading,
    AwaitingPickup,
    EndCustody,
}

#[derive(Debug)]
struct LuggageId(usize);

#[derive(Debug)]
struct Luggage<LuggageState> {
    tracking_id: LuggageId,
    state: LuggageState,
}

impl<LuggageState: std::fmt::Debug> Luggage<LuggageState> {
    fn transition<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage {
            tracking_id: self.tracking_id,
            state,
        }
    }
    fn print_state(&self) {
        println!("{:?}", self);
    }
}

impl Luggage<LuggageState> {
    fn new(tracking_id: LuggageId) -> Self {
        Self {
            tracking_id,
            state: LuggageState::CheckIn,
        }
    }

    fn on_load(self) -> Luggage<LuggageState> {
        self.transition(LuggageState::OnLoading)
    }

    fn off_load(self) -> Luggage<LuggageState> {
        self.transition(LuggageState::OffLoading)
    }

    fn await_pickup(self) -> Luggage<LuggageState> {
        self.transition(LuggageState::AwaitingPickup)
    }

    fn end_custody(self) -> Luggage<LuggageState> {
        self.transition(LuggageState::EndCustody)
    }
}

fn main() {
    let luggage = Luggage::new(LuggageId(123));
    luggage.print_state();

    let on_loaded = luggage.on_load();
    on_loaded.print_state();

    let off_loaded = on_loaded.off_load();
    off_loaded.print_state();

    let awaited_pick_up = off_loaded.await_pickup();
    awaited_pick_up.print_state();

    let end_custody = awaited_pick_up.end_custody();
    end_custody.print_state();
}
