// HeartOn Budget Tracker (Community Edition)
// MIT Licensed

use bevy_ecs::system::Resource;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct FrameTimings {
    pub frame_number: u64,
    pub cpu_time: Duration,
    pub total_time: Duration,
}

#[derive(Debug, Resource)]
pub struct HeartOnBudget {
    frame_number: u64,
    frame_start: Instant,
    cpu_timings: Vec<FrameTimings>,
    max_history: usize,
}

impl Default for HeartOnBudget {
    fn default() -> Self {
        Self::new(100)
    }
}

impl HeartOnBudget {
    pub fn new(max_history: usize) -> Self {
        Self {
            frame_number: 0,
            frame_start: Instant::now(),
            cpu_timings: Vec::with_capacity(max_history),
            max_history,
        }
    }

    pub fn begin_frame(&mut self) {
        self.frame_start = Instant::now();
    }

    pub fn end_frame(&mut self) {
        let cpu_time = self.frame_start.elapsed();
        let timing = FrameTimings {
            frame_number: self.frame_number,
            cpu_time,
            total_time: cpu_time,
        };

        if self.cpu_timings.len() >= self.max_history {
            self.cpu_timings.remove(0);
        }
        self.cpu_timings.push(timing);
        self.frame_number += 1;
    }

    pub fn frame_number(&self) -> u64 {
        self.frame_number
    }

    pub fn average_cpu_ms(&self) -> f64 {
        if self.cpu_timings.is_empty() {
            return 0.0;
        }
        let sum: Duration = self.cpu_timings.iter().map(|t| t.cpu_time).sum();
        sum.as_secs_f64() * 1000.0 / self.cpu_timings.len() as f64
    }

    pub fn average_fps(&self) -> f64 {
        let avg_ms = self.average_cpu_ms();
        if avg_ms > 0.0 {
            1000.0 / avg_ms
        } else {
            0.0
        }
    }

    pub fn last_frame_cpu_ms(&self) -> f64 {
        self.cpu_timings
            .last()
            .map(|t| t.cpu_time.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }

    pub fn export_csv(&self) -> String {
        let mut csv = String::from("frame_number,cpu_ms,total_ms\n");
        for timing in &self.cpu_timings {
            csv.push_str(&format!(
                "{},{:.3},{:.3}\n",
                timing.frame_number,
                timing.cpu_time.as_secs_f64() * 1000.0,
                timing.total_time.as_secs_f64() * 1000.0
            ));
        }
        csv
    }

    pub fn timings(&self) -> &[FrameTimings] {
        &self.cpu_timings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn budget_tracks_frames() {
        let mut budget = HeartOnBudget::new(10);
        assert_eq!(budget.frame_number(), 0);

        budget.begin_frame();
        thread::sleep(Duration::from_millis(1));
        budget.end_frame();

        assert_eq!(budget.frame_number(), 1);
        assert!(budget.last_frame_cpu_ms() >= 1.0);
    }

    #[test]
    fn budget_limits_history() {
        let mut budget = HeartOnBudget::new(3);

        for _ in 0..5 {
            budget.begin_frame();
            budget.end_frame();
        }

        assert_eq!(budget.timings().len(), 3);
        assert_eq!(budget.frame_number(), 5);
    }

    #[test]
    fn csv_export_works() {
        let mut budget = HeartOnBudget::new(10);
        budget.begin_frame();
        budget.end_frame();

        let csv = budget.export_csv();
        assert!(csv.contains("frame_number,cpu_ms,total_ms"));
        assert!(csv.contains("0,"));
    }

    #[test]
    fn average_calculations() {
        let mut budget = HeartOnBudget::new(10);

        for _ in 0..3 {
            budget.begin_frame();
            thread::sleep(Duration::from_millis(10));
            budget.end_frame();
        }

        let avg_ms = budget.average_cpu_ms();
        assert!(avg_ms >= 10.0);
        assert!(avg_ms < 50.0);

        let fps = budget.average_fps();
        assert!(fps > 20.0);
        assert!(fps < 100.0);
    }
}
