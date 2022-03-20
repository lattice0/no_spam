#[cfg(feature = "random")]
use rand::Rng;
use std::time::Instant;

pub struct NoSpam {
    last_call: Option<Instant>,
    max_per_time: f32,
    pub calls_per_time: u32,
    time_factor: f32,
}

impl NoSpam {
    pub fn new_per_second(max_per_second: u32) -> NoSpam {
        NoSpam {
            last_call: None,
            max_per_time: max_per_second as f32,
            calls_per_time: 0,
            time_factor: 1.0,
        }
    }

    pub fn new_per_minute(max_per_minute: u32) -> NoSpam {
        NoSpam {
            last_call: None,
            max_per_time: max_per_minute as f32,
            calls_per_time: 0,
            time_factor: (1 / 60) as f32,
        }
    }

    pub fn new_per_hour(max_per_hour: u32) -> NoSpam {
        NoSpam {
            last_call: None,
            max_per_time: max_per_hour as f32,
            calls_per_time: 0,
            time_factor: (1 / (60 * 60)) as f32,
        }
    }

    pub fn new_per_day(max_per_day: u32) -> NoSpam {
        NoSpam {
            last_call: None,
            max_per_time: max_per_day as f32,
            calls_per_time: 0,
            time_factor: (1 / (60 * 60 * 24)) as f32,
        }
    }

    pub fn on<F>(&mut self, mut f: F)
    where
        F: FnMut(u32),
    {
        // If we're on debug mode and `ignore_on_debug` feature is on, simply ignore
        // the counter and always execute the function 
        #[cfg(all(debug_assertions, ignore_on_debug))]
        {
            f(self.calls_per_time);
            return;
        }
        if let Some(last_call) = self.last_call {
            if last_call.elapsed().as_secs() as f32 * self.time_factor >= 1.0 {
                self.calls_per_time = 0;
            }
        }
        if (self.calls_per_time as f32) < self.max_per_time {
            f(self.calls_per_time);
            self.last_call = Some(Instant::now());
        }
        self.calls_per_time += 1;
    }
}

#[cfg(feature = "random")]
/// Will call `f`, on average, `per_time` times. `estimated_per_time` is the estimation
/// on how many times this function will be called.
/// Example:
/// If you want your video render, that runs at 30fps, to show an error but does not want
/// this error to be shown the 30 times (in the worst case where it happens 30 times per second),
/// then `per_time = 1` and  `estimated_per_time = 30` will make `f` be called on average
/// 1 times per second
///
/// Usage example:
///
/// ```
/// use NoSpam::per_time;
/// fn main () {
///     let t = 500;
///     for _ in 0..t {
///         per_time(10, t, |_|{println!("called")});
///     }
/// }
/// ```
/// `"called"` will be displayed on average, 10 times
pub fn per_time<F>(per_time: u32, estimated_per_time: u32, mut f: F)
where
    F: FnMut(u32),
{
    let factor = estimated_per_time / per_time;
    let num = rand::thread_rng().gen_range(0..factor);
    if num == 0 {
        f(per_time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respects_maximum_per_second() {
        let mut collect: Vec<Option<u32>> = Vec::new();
        let mut no_spam = NoSpam::new_per_second(5);
        for _ in 0..10 {
            no_spam.on(|_| collect.push(None));
        }
        assert!(collect.len() == 5, "collect size: {}", collect.len());
    }
    #[test]
    fn respects_maximum_per_second_after_seconds() {
        let mut collect: Vec<Option<u32>> = Vec::new();
        let mut no_spam = NoSpam::new_per_second(5);
        for _ in 0..10 {
            no_spam.on(|_| collect.push(None));
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
        for _ in 0..10 {
            no_spam.on(|_| collect.push(None));
        }
        assert!(collect.len() == 10, "collect size: {}", collect.len());
    }
    #[test]
    fn respects_maximum_per_minute() {
        let mut collect: Vec<Option<u32>> = Vec::new();
        let mut no_spam = NoSpam::new_per_minute(5);
        for _ in 0..10 {
            no_spam.on(|_| collect.push(None));
        }
        assert!(collect.len() == 5, "collect size: {}", collect.len());
    }
    #[test]
    fn respects_maximum_per_second_after_minutes() {
        let mut collect: Vec<Option<u32>> = Vec::new();
        let mut no_spam = NoSpam::new_per_second(5);
        for _ in 0..10 {
            no_spam.on(|_| collect.push(None));
        }
        std::thread::sleep(std::time::Duration::from_secs(120));
        for _ in 0..10 {
            no_spam.on(|_| collect.push(None));
        }
        assert!(collect.len() == 10, "collect size: {}", collect.len());
    }
}
