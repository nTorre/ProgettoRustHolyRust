use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::utils::LibError;
use crate::utils::LibError::{EmptyForecast, WrongHour};

/// Represents the weather conditions the world map can currently be in.
///
/// # Variants
/// - 'Sunny': Represents sunny weather.
/// - 'Rainy': Represents rainy weather.
/// - 'Foggy': Represents foggy weather.
/// - 'TropicalMonsoon': Represents a very windy and hot weather.
/// - 'TrentinoSnow': Represents the coldest winter weather possible, comprised of snow.
///
/// # Usage
///
/// ```rust
/// use robotics_lib::world::environmental_conditions::WeatherType;
/// let weather = WeatherType::TrentinoSnow;
///
/// match weather {
///     WeatherType::Foggy => println!("You can't see much"),
///     WeatherType::Sunny => println!("Solar charge up"),
///     WeatherType::Rainy => println!("It's difficult to move"),
///     WeatherType::TropicalMonsoon => println!("Very hot and low stamina"),
///     WeatherType::TrentinoSnow => println!("You need energy to heat up")
/// }
/// ```
#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum WeatherType {
    Sunny,
    Rainy,
    Foggy,
    TropicalMonsoon,
    TrentinoSnow,
    //...
}

/// Represents the different time periods of a day in the World.
///
/// # Variants
/// - 'Morning': Represents the morning time, from 7.00 to 11.59.
/// - 'Afternoon': Represents the afternoon time, from 12.00 to 20.59
/// - 'Night': Represents the night time, from 21.00 to 6.59.
///
/// # Usage
///
/// ```rust
/// use robotics_lib::world::environmental_conditions::DayTime;
/// let time_of_day = DayTime::Morning;
///
/// match time_of_day {
///     DayTime::Morning => println!("Wake up!"),
///     DayTime::Afternoon => println!("Working time!"),
///     DayTime::Night => println!("Sleepy time!")
/// }
/// ```
#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DayTime {
    Morning,
    Afternoon,
    Night,
}

/// Contains the environment information for the World struct. Keeps track of the weather and
/// daylight cycle, providing them to the outside such that they can be read and used.
///
/// # Usage
///
/// ```rust
/// use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
/// let weather_forecast = vec![
///     WeatherType::Sunny,
///     WeatherType::Rainy,
///     WeatherType::Foggy,
///     WeatherType::TropicalMonsoon,
/// ];
///
/// let mut environmental_conditions = EnvironmentalConditions::new(&weather_forecast, 30, 12).unwrap();
/// ```
/// # Fields
///
/// - 'time_progression_minutes': Controls how many minutes of time a tick progresses.
/// - 'time_of_day': The current time the day is in.
/// - 'weather_forecast': Cycling vector for the weather, keeps cycling once a day.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct EnvironmentalConditions {
    time_progression_minutes: u8,
    time_of_day: TimeOfDay,
    weather_forecast: VecDeque<WeatherType>,
}

impl EnvironmentalConditions {
    /// Creates a new EnvironmentalConditions instance with user parameters.
    ///
    /// This function allows you to create a new EnvironmentalConditions instance by providing an existing weather forecast,
    /// represented by a `VecDeque<WeatherType>`, a `time_progression_minutes` value and a `starting_hour` value
    ///
    /// # Arguments
    ///
    /// - weather_forecast: A &[WeatherType] representing the weather conditions that will be cycling in the World.
    /// - time_progression_minutes: A u8 representing the minutes the time will progress per tick.
    /// - starting_hour: A u8 representing the starting hour for the World.
    ///
    /// # Returns
    ///
    /// A new EnvironmentalConditions instance with the provided properties.
    ///
    /// # Panics
    /// The method panics if the provided forecast is empty. The world generator should handle its correctness.
    pub fn new(
        weather_forecast: &[WeatherType],
        time_progression_minutes: u8,
        starting_hour: u8,
    ) -> Result<Self, LibError> {
        if weather_forecast.is_empty() {
            return Err(EmptyForecast);
        } else if starting_hour > 24 {
            return Err(WrongHour);
        }
        Ok(EnvironmentalConditions {
            time_progression_minutes,
            time_of_day: TimeOfDay {
                hour: starting_hour,
                minute: 0,
            },
            weather_forecast: VecDeque::from(weather_forecast.to_vec()),
        })
    }

    /// Cycles the weather_forecast to the next element, as one day is passed.
    pub(crate) fn next_day(&mut self) {
        let front = self.weather_forecast.pop_front().unwrap(); // The queue will never be empty, it is initialized by the world generator
        self.weather_forecast.push_back(front);
    }

    /// Ticks the time by `time_progression_minutes` minutes, and cycles the weather if needed.
    /// # Returns
    /// `true` if the day changed, `false` otherwise
    pub fn tick(&mut self) -> bool {
        let is_day_changed = self.time_of_day.advance(self.time_progression_minutes);
        if is_day_changed {
            self.next_day();
        }
        is_day_changed
    }

    /// Getter for the current weather condition: WeatherType
    pub fn get_weather_condition(&self) -> WeatherType {
        *self.weather_forecast.front().unwrap()
    }

    // disable clippy warning that conflicts with rust fmt
    #[allow(clippy::manual_range_patterns)]
    /// Getter for the current daytime period: DayTime
    pub fn get_time_of_day(&self) -> DayTime {
        match self.time_of_day.hour {
            | 0..=6 | 21..=24 => DayTime::Night,
            | 7..=11 => DayTime::Morning,
            | 12..=20 => DayTime::Afternoon,
            | _ => unreachable!(),
        }
    }
    /// Getter for the current time of day as a string in the format HH:MM
    pub fn get_time_of_day_string(&self) -> String {
        format!("{:02}:{:02}", self.time_of_day.hour, self.time_of_day.minute)
    }
}

/// Handles time progression and day cycling.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct TimeOfDay {
    pub(crate) hour: u8,
    pub(crate) minute: u8,
}

impl TimeOfDay {
    pub(crate) fn advance(&mut self, time_progression_minutes: u8) -> bool {
        let mut m = self.minute + time_progression_minutes;
        while m > 59 {
            self.hour += 1;
            m -= 60
        }

        self.minute = m;

        if self.hour > 23 {
            self.hour -= 24;
            return true;
        }

        false
    }
}
