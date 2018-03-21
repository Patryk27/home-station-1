extern crate actix;
extern crate pwr_hd44780;

use actix::{ActorFuture, Addr, AsyncContext, Context, ContextFutureSpawner, Syn, WrapFuture};
use pwr_hd44780::Hd44780;
use self::components::Component;
pub use self::config::Configuration;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use utils::UnitResult;
use weather;

mod actor;
mod config;
mod constructor;
mod components;

/// The UI Actor.
/// Contains all the logic necessary to keep the LCD user-friendly.
pub struct Actor {
    // Actors required by the UI
    actors: Actors,

    // Reference to the HD44780; it's shared across all the UI components
    lcd: Rc<RefCell<pwr_hd44780::BufferedLcd>>,

    // UI components (left, right, bottom & separators)
    components: Vec<Box<Component>>,

    // Additional state, e.g. the current weather from the weather actor
    state: State,
}

/// This structure groups all the actors required by the UI actor to work.
struct Actors {
    weather: Addr<Syn, weather::Actor>,
}

/// This structure groups additional UI state - currently all the required weather data is placed
/// here as soon as it's possible.
#[derive(Clone, Debug)]
pub struct State {
    weather: Option<weather::State>,
    weather_updating: bool,
}

impl Actor {
    /// Sets up and keeps track of the UI's main thread.
    fn process(&self, ctx: &mut Context<Self>) {
        const FPS: u64 = 25;

        ctx.run_later(
            Duration::from_millis(1000 / FPS),
            |act, ctx| {
                act.update(ctx).unwrap();
                act.render().unwrap();

                act.process(ctx);
            },
        );
    }

    /// Updates the UI's state.
    /// Called automatically in the process() loop.
    fn update(&mut self, ctx: &mut Context<Self>) -> UnitResult {
        // update the weather state
        if !self.state.weather_updating {
            self.state.weather_updating = true;

            self.actors.weather.send(::weather::messages::GetState {})
                .into_actor(self)
                .then(|res, act, _| {
                    act.state.weather = Some(res.unwrap().unwrap());
                    act.state.weather_updating = false;

                    actix::fut::ok(())
                })
                .spawn(ctx);
        }

        // update components
        for component in self.components.iter_mut() {
            component.update(self.state.clone())?;
        }

        Ok(())
    }

    /// Refreshes the UI.
    /// Called automatically in the process() loop.
    fn render(&mut self) -> UnitResult {
        {
            self.lcd.borrow_mut().clear()?;
        }

        for component in self.components.iter_mut() {
            component.render()?;
        }

        {
            self.lcd.borrow_mut().render()?;
        }

        Ok(())
    }
}