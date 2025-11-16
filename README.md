# HeartOn Public API

**License**: MIT  
**Edition**: Community (Free)  
**Version**: 0.1.0

The HeartOn Public API provides the MIT-licensed bridge layer for HeartOn Engine, allowing developers to build voxel-based games and applications without commercial licensing restrictions.

## Features

### Community Edition (This Crate)

✅ **Free and Open Source** (MIT License)  
✅ **Basic Voxel Rendering** (up to 1,000,000 voxels)  
✅ **Performance Monitoring** (FPS, frame time tracking)  
✅ **Simple HUD System** (debug overlay)  
✅ **Vulkan Detection** (capability checking)  
✅ **CSV Budget Export** (performance analysis)  
✅ **Standard Rasterization** (instanced cube rendering)

### Professional Edition Features (Requires License)

❌ **Unlimited Voxels** (billions of voxels via SVDAG)  
❌ **Task/Mesh Shaders** (GPU-driven rendering)  
❌ **Neural Radiance Caching** (NRC for GI)  
❌ **Advanced Compression** (SVDAG, octree optimization)  
❌ **Pro Editor Tools** (visual editing, baking)  
❌ **Commercial Use** (no restrictions)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
hearton_public = { git = "https://github.com/hearthshire/HeartOn-public" }
```

## Quick Start

```rust
use bevy::prelude::*;
use hearton_public::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HeartOnPublicPlugin::default())
        .run();
}
```

## Usage Examples

### Creating a Voxel World

```rust
use hearton_public::voxel::{DummyVoxelWorld, Voxel};

fn setup(mut commands: Commands) {
    let mut world = DummyVoxelWorld::new();
    
    // Add individual voxels
    world.add_voxel(Voxel::from_rgb(0, 0, 0, 255, 0, 0)); // Red voxel
    world.add_voxel(Voxel::from_rgb(1, 0, 0, 0, 255, 0)); // Green voxel
    
    // Or generate a test world
    world.generate_test_world(10);
    
    commands.spawn(world);
}
```

### Performance Monitoring

```rust
use hearton_public::budget::HeartOnBudget;

fn update_budget(mut budget: ResMut<HeartOnBudget>) {
    budget.begin_frame();
    
    // Your game logic here
    
    budget.end_frame();
    
    println!("FPS: {:.1}", budget.average_fps());
    println!("Frame time: {:.2}ms", budget.last_frame_cpu_ms());
}
```

### Capability Detection

```rust
use hearton_public::capabilities::HeartOnCapabilities;

fn check_capabilities(caps: Res<HeartOnCapabilities>) {
    println!("Vulkan: {:?}", caps.vulkan_version);
    println!("Max voxels: {}", caps.max_voxels);
    
    if caps.is_community_edition {
        println!("Running Community Edition");
    }
}
```

### Exporting Performance Data

```rust
use hearton_public::budget::HeartOnBudget;
use std::fs;

fn export_performance(budget: Res<HeartOnBudget>) {
    let csv = budget.export_csv();
    fs::write("performance.csv", csv).unwrap();
}
```

## API Documentation

Generate and view the full API documentation:

```bash
cargo doc --no-deps --open
```

## Limitations

### Community Edition Constraints

- **Voxel Limit**: 1,000,000 voxels maximum
- **Rendering**: Simple instanced cube rasterization only
- **No GPU-Acceleration**: No task/mesh shaders
- **No Advanced Compression**: No SVDAG or octree optimization
- **No Neural GI**: No Neural Radiance Caching (NRC)
- **Basic HUD Only**: Limited debug information

These limitations are enforced at compile time and cannot be bypassed without a valid Professional Edition license.

## Upgrading to Professional Edition

To unlock unlimited voxels, advanced rendering pipelines, and commercial use rights:

1. Purchase a license at [hearton.com/pricing](https://hearton.com/pricing)
2. Gain access to `hearton-private` repository
3. Build with full workspace: `cargo build --workspace --release`

### Pricing

- **Indie Edition**: $99/year (single developer, unlimited features)
- **Studio Edition**: $299/year + custom (team license, source access)

## Architecture

HeartOn uses a workspace-submodule architecture:

```
hearton-workspace/           (Private workspace)
├── bevy/                    (Bevy 0.13.2, MIT, unmodified)
├── hearton-public/          (This crate, MIT)
└── hearton-private/         (Commercial features, PROPRIETARY)
```

This structure ensures:
- Clean license separation (MIT vs PROPRIETARY)
- No modifications to upstream Bevy
- Community edition always builds without proprietary code

## Contributing

Contributions to the public API are welcome!

1. Fork [HeartOn-public](https://github.com/hearthshire/HeartOn-public)
2. Create a feature branch
3. Add tests for new features
4. Submit a pull request

All contributions must be MIT-compatible.

## Support

- **Documentation**: https://docs.hearton.com
- **Discord**: https://discord.gg/hearton
- **Issues**: https://github.com/hearthshire/HeartOn-public/issues
- **Email**: support@hearton.com

## License

This crate is licensed under the MIT License. See [LICENSE](LICENSE) for details.

Commercial HeartOn features (in `hearton-private`) are PROPRIETARY and require a separate license.

## Credits

Built on [Bevy Engine](https://bevyengine.org/) by [HeartOn Studios](https://hearton.com).
