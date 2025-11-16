// HeartOn Engine - Community Post-FX Effects
// License: MIT

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectQuality {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectFormat {
    RGBA8,
    RGBA16F,
    RGB10A2,
}

pub struct EffectInput {
    pub width: u32,
    pub height: u32,
    pub format: EffectFormat,
}

pub struct EffectOutput {
    pub data: Vec<u8>,
}

impl EffectOutput {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

pub trait PostEffect: Send + Sync {
    fn name(&self) -> &str;
    fn apply(&self, input: &EffectInput, output: &mut EffectOutput) -> Result<(), String>;
    fn gpu_time_estimate(&self) -> f32;
    fn is_enabled(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct PostEffectParams {
    pub intensity: f32,
    pub enabled: bool,
}

impl Default for PostEffectParams {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleBloom {
    pub threshold: f32,
    pub intensity: f32,
    pub quality: EffectQuality,
    pub enabled: bool,
}

impl Default for SimpleBloom {
    fn default() -> Self {
        Self {
            threshold: 1.0,
            intensity: 0.5,
            quality: EffectQuality::Medium,
            enabled: true,
        }
    }
}

impl PostEffect for SimpleBloom {
    fn name(&self) -> &str {
        "SimpleBloom"
    }
    
    fn apply(&self, input: &EffectInput, output: &mut EffectOutput) -> Result<(), String> {
        let pixel_count = (input.width * input.height) as usize;
        let bytes_per_pixel = match input.format {
            EffectFormat::RGBA8 => 4,
            EffectFormat::RGBA16F => 8,
            EffectFormat::RGB10A2 => 4,
        };
        
        output.data.resize(pixel_count * bytes_per_pixel, 0);
        Ok(())
    }
    
    fn gpu_time_estimate(&self) -> f32 {
        match self.quality {
            EffectQuality::Low => 0.3,
            EffectQuality::Medium => 0.8,
            EffectQuality::High => 1.5,
        }
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToneMappingMode {
    None,
    Reinhard,
    Aces,
    Uncharted2,
}

#[derive(Debug, Clone)]
pub struct ToneMapping {
    pub mode: ToneMappingMode,
    pub exposure: f32,
    pub enabled: bool,
}

impl Default for ToneMapping {
    fn default() -> Self {
        Self {
            mode: ToneMappingMode::Aces,
            exposure: 1.0,
            enabled: true,
        }
    }
}

impl PostEffect for ToneMapping {
    fn name(&self) -> &str {
        "ToneMapping"
    }
    
    fn apply(&self, input: &EffectInput, output: &mut EffectOutput) -> Result<(), String> {
        let pixel_count = (input.width * input.height) as usize;
        let bytes_per_pixel = match input.format {
            EffectFormat::RGBA8 => 4,
            EffectFormat::RGBA16F => 8,
            EffectFormat::RGB10A2 => 4,
        };
        
        output.data.resize(pixel_count * bytes_per_pixel, 0);
        Ok(())
    }
    
    fn gpu_time_estimate(&self) -> f32 {
        0.2
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Clone)]
pub struct Vignette {
    pub intensity: f32,
    pub smoothness: f32,
    pub enabled: bool,
}

impl Default for Vignette {
    fn default() -> Self {
        Self {
            intensity: 0.3,
            smoothness: 0.5,
            enabled: true,
        }
    }
}

impl PostEffect for Vignette {
    fn name(&self) -> &str {
        "Vignette"
    }
    
    fn apply(&self, input: &EffectInput, output: &mut EffectOutput) -> Result<(), String> {
        let pixel_count = (input.width * input.height) as usize;
        let bytes_per_pixel = match input.format {
            EffectFormat::RGBA8 => 4,
            EffectFormat::RGBA16F => 8,
            EffectFormat::RGB10A2 => 4,
        };
        
        output.data.resize(pixel_count * bytes_per_pixel, 0);
        Ok(())
    }
    
    fn gpu_time_estimate(&self) -> f32 {
        0.1
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_bloom_defaults() {
        let bloom = SimpleBloom::default();
        assert_eq!(bloom.threshold, 1.0);
        assert_eq!(bloom.intensity, 0.5);
        assert_eq!(bloom.quality, EffectQuality::Medium);
        assert!(bloom.enabled);
    }
    
    #[test]
    fn test_simple_bloom_gpu_time() {
        let bloom_low = SimpleBloom {
            quality: EffectQuality::Low,
            ..Default::default()
        };
        assert_eq!(bloom_low.gpu_time_estimate(), 0.3);
        
        let bloom_high = SimpleBloom {
            quality: EffectQuality::High,
            ..Default::default()
        };
        assert_eq!(bloom_high.gpu_time_estimate(), 1.5);
    }
    
    #[test]
    fn test_tone_mapping_modes() {
        let tm = ToneMapping::default();
        assert_eq!(tm.mode, ToneMappingMode::Aces);
        assert_eq!(tm.exposure, 1.0);
        assert!(tm.enabled);
    }
    
    #[test]
    fn test_vignette_defaults() {
        let vignette = Vignette::default();
        assert_eq!(vignette.intensity, 0.3);
        assert_eq!(vignette.smoothness, 0.5);
        assert!(vignette.enabled);
    }
    
    #[test]
    fn test_effect_apply() {
        let bloom = SimpleBloom::default();
        let input = EffectInput {
            width: 1920,
            height: 1080,
            format: EffectFormat::RGBA8,
        };
        let mut output = EffectOutput::new(0);
        
        let result = bloom.apply(&input, &mut output);
        assert!(result.is_ok());
        assert_eq!(output.data.len(), 1920 * 1080 * 4);
    }
}
