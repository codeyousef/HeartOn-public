use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Debug, Clone)]
pub struct PostFxBudget {
    pub max_post_fx_ms: f32,
    pub current_frame_ms: f32,
    pub effect_timings: HashMap<String, EffectTiming>,
    pub throttled: bool,
    pub throttle_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EffectTiming {
    pub gpu_ms: f32,
    pub enabled: bool,
    pub priority: u32,
}

impl Default for PostFxBudget {
    fn default() -> Self {
        Self {
            max_post_fx_ms: 4.0,
            current_frame_ms: 0.0,
            effect_timings: HashMap::new(),
            throttled: false,
            throttle_reason: None,
        }
    }
}

impl PostFxBudget {
    pub fn new(max_ms: f32) -> Self {
        Self {
            max_post_fx_ms: max_ms,
            ..Default::default()
        }
    }

    pub fn begin_frame(&mut self) {
        self.current_frame_ms = 0.0;
        self.throttled = false;
        self.throttle_reason = None;
    }

    pub fn record_effect(&mut self, name: String, gpu_ms: f32, priority: u32) {
        self.current_frame_ms += gpu_ms;
        self.effect_timings.insert(
            name.clone(),
            EffectTiming {
                gpu_ms,
                enabled: true,
                priority,
            },
        );

        if self.current_frame_ms > self.max_post_fx_ms {
            self.throttled = true;
            self.throttle_reason = Some(format!(
                "Post-FX budget exceeded: {:.2}ms / {:.2}ms",
                self.current_frame_ms, self.max_post_fx_ms
            ));
        }
    }

    pub fn should_disable_effect(&self, name: &str) -> bool {
        if !self.throttled {
            return false;
        }

        if let Some(timing) = self.effect_timings.get(name) {
            timing.priority < 5
        } else {
            false
        }
    }

    pub fn get_total_time(&self) -> f32 {
        self.current_frame_ms
    }

    pub fn is_over_budget(&self) -> bool {
        self.current_frame_ms > self.max_post_fx_ms
    }

    pub fn get_budget_usage(&self) -> f32 {
        if self.max_post_fx_ms > 0.0 {
            (self.current_frame_ms / self.max_post_fx_ms).min(1.0)
        } else {
            0.0
        }
    }

    pub fn disable_effect(&mut self, name: &str) {
        if let Some(timing) = self.effect_timings.get_mut(name) {
            timing.enabled = false;
            self.current_frame_ms -= timing.gpu_ms;
        }
    }

    pub fn enable_effect(&mut self, name: &str) {
        if let Some(timing) = self.effect_timings.get_mut(name) {
            if !timing.enabled {
                timing.enabled = true;
                self.current_frame_ms += timing.gpu_ms;
            }
        }
    }

    pub fn get_effect_status(&self, name: &str) -> Option<bool> {
        self.effect_timings.get(name).map(|t| t.enabled)
    }

    pub fn clear_timings(&mut self) {
        self.effect_timings.clear();
        self.current_frame_ms = 0.0;
    }
}

pub fn update_budget_system(mut budget: ResMut<PostFxBudget>) {
    budget.begin_frame();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_budget() {
        let budget = PostFxBudget::default();
        assert_eq!(budget.max_post_fx_ms, 4.0);
        assert_eq!(budget.current_frame_ms, 0.0);
        assert!(!budget.throttled);
    }

    #[test]
    fn test_record_effect() {
        let mut budget = PostFxBudget::default();
        budget.record_effect("bloom".to_string(), 1.5, 8);
        assert_eq!(budget.current_frame_ms, 1.5);
        assert!(!budget.throttled);
    }

    #[test]
    fn test_budget_exceeded() {
        let mut budget = PostFxBudget::new(2.0);
        budget.record_effect("bloom".to_string(), 1.5, 8);
        budget.record_effect("tone_mapping".to_string(), 1.0, 10);
        assert!(budget.throttled);
        assert!(budget.is_over_budget());
    }

    #[test]
    fn test_should_disable_low_priority() {
        let mut budget = PostFxBudget::new(2.0);
        budget.record_effect("bloom".to_string(), 1.5, 3);
        budget.record_effect("tone_mapping".to_string(), 1.0, 10);
        
        assert!(budget.should_disable_effect("bloom"));
        assert!(!budget.should_disable_effect("tone_mapping"));
    }

    #[test]
    fn test_disable_enable_effect() {
        let mut budget = PostFxBudget::default();
        budget.record_effect("bloom".to_string(), 1.5, 8);
        
        budget.disable_effect("bloom");
        assert_eq!(budget.current_frame_ms, 0.0);
        assert_eq!(budget.get_effect_status("bloom"), Some(false));
        
        budget.enable_effect("bloom");
        assert_eq!(budget.current_frame_ms, 1.5);
        assert_eq!(budget.get_effect_status("bloom"), Some(true));
    }

    #[test]
    fn test_budget_usage() {
        let mut budget = PostFxBudget::new(4.0);
        budget.record_effect("effect1".to_string(), 2.0, 5);
        assert_eq!(budget.get_budget_usage(), 0.5);
        
        budget.record_effect("effect2".to_string(), 2.0, 5);
        assert_eq!(budget.get_budget_usage(), 1.0);
    }

    #[test]
    fn test_begin_frame_reset() {
        let mut budget = PostFxBudget::default();
        budget.record_effect("bloom".to_string(), 1.5, 8);
        budget.throttled = true;
        
        budget.begin_frame();
        assert_eq!(budget.current_frame_ms, 0.0);
        assert!(!budget.throttled);
        assert!(budget.throttle_reason.is_none());
    }

    #[test]
    fn test_clear_timings() {
        let mut budget = PostFxBudget::default();
        budget.record_effect("bloom".to_string(), 1.5, 8);
        budget.record_effect("tone_mapping".to_string(), 1.0, 10);
        
        budget.clear_timings();
        assert_eq!(budget.effect_timings.len(), 0);
        assert_eq!(budget.current_frame_ms, 0.0);
    }
}
