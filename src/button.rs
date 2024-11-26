mod press_duration;

use crate::shared_const::{BUTTON_DEBOUNCE_DELAY, LONG_PRESS_DURATION};
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::{AnyPin, Input, Pull};
use embassy_time::Timer;

pub use press_duration::PressDuration;

/// Type representing the hardware button and its state.
pub struct Button<'a> {
    pin: Input<'a>,
}

impl Button<'_> {
    /// Constructor.  Inject the GPIO pin where the hardware button can be found.
    #[must_use]
    pub fn new(pin: AnyPin) -> Self {
        Button {
            // Define the pin as an input with a "default" of `Level::Low`, or no voltage
            pin: Input::new(pin, Pull::Down),
        }
    }

    pub async fn press_duration(&mut self) -> PressDuration {
        // wait for the button to be released
        self.wait_for_button_up().await;
        self.debounce_delay().await;

        // Wait for the voltage level on this pin to go high (button has been pressed)
        self.wait_for_button_down().await;

        // Sometimes the start (and end) of a press can be "noisy" (fluctuations between
        // "pressed" and "unpressed" states on the microsecond time scale as the physical
        // contacts changing from "not touching" through "almost touching" to "touching" (or
        // vice-versa)).  We're going to ignore the button's state during the noisy, fluctuating
        // "almost touching" state.  This is called "debouncing".
        self.debounce_delay().await;

        // The button is now fully depressed.

        // Wait for the button to be released or to be a "LONG" press.
        match select(self.wait_for_down_press(), Timer::after(LONG_PRESS_DURATION)).await {
            Either::First(_) => PressDuration::Short,
            Either::Second(()) => PressDuration::Long,
        }
    }

    /// Pause for a predetermined time to let the button's state become consistent.
    async fn debounce_delay(&mut self) -> &mut Self {
        Timer::after(BUTTON_DEBOUNCE_DELAY).await;
        self
    }

    /// Pause until voltage is present on the input pin.
    async fn wait_for_button_down(&mut self) -> &mut Self {
        self.pin.wait_for_high().await;
        self
    }

    // wait for the button to be released
    async fn wait_for_button_up(&mut self) -> &mut Self {
        self.pin.wait_for_low().await;
        self
    }

    /// Pause until voltage on the input pin begins to go away.
    async fn wait_for_down_press(&mut self) -> &mut Self {
        self.pin.wait_for_falling_edge().await;
        self
    }
}
