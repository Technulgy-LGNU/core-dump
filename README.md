# Core Dump

Shared Rust support crate for my RoboCup Small Size League robot code.

This repository keeps the common pieces that other SSL projects should not have
to redefine:

- a small generic vector library for robot/world geometry
- generated Rust types for all protobuf messages used by the codebase
- protobuf sources for both my own CrashPilot communication and the upstream SSL
  protocols used around vision, game controller, referee, CI, and remote control

## Crate Layout

```text
src/
  lib.rs                  Crate entry point
  vec.rs                  Vector module exports
  vec/types.rs            Vec2, Vec3, Axis
  vec/generic_operators.rs
                           Vector/vector and vector/scalar operators
  vec/scalar_operators.rs Numeric helper methods
  proto.rs                Generated Prost types

proto/
  crashpilot/             My own robot, interface, and CrashPilot messages
  vision_tracked/         RoboCup SSL vision and tracker messages
  state/                  SSL game controller state and referee messages
  geom/                   Shared SSL geometry messages
  api/, engine/,
  statemachine/, ci/,
  rcon/                   SSL game controller API, engine, CI, and RCON messages
```

## Using The Crate

Add the crate as a dependency from this repository:

```toml
[dependencies]
core_dump = { git = "https://github.com/Technulgy-LGNU/core-dump.git" }
```

For local development from another workspace, use a path dependency:

```toml
[dependencies]
core_dump = { path = "../core-dump" }
```

Enable the optional `serde` feature when vectors need to be serialized or
deserialized:

```toml
[dependencies]
core_dump = { path = "../core-dump", features = ["serde"] }
```

## Vector Types

The vector module exposes generic `Vec2<T>` and `Vec3<T>` types with common
operator overloads and geometry helpers.

```rust
use core_dump::vec::types::{Axis, Vec2, Vec3};

let position = Vec2::new(3.0_f32, 4.0);

assert_eq!(position.get(Axis::X), 3.0);
assert_eq!(position.norm(), 5.0);
assert_eq!(position.normalized().norm(), 1.0);

let velocity = Vec2::new(2.0_f32, 0.0);
let next_position = position + velocity * 0.1;

let pose = Vec3::new(next_position.x, next_position.y, 1.57_f32);
let xy = pose.xy();
```

Available vector functionality includes:

- `Vec2::new`, `Vec3::new`, `Default`, and `zero`
- vector addition, subtraction, multiplication, and division
- scalar multiplication and division
- `dot`, `det`, `length`, `norm`, `norm_squared`, and `normalized`
- distance helpers such as `distance` and `distance_to_segment`
- speed limiting through `with_speed_clamped`
- angle helpers such as `angle_in_u16` and `angle_from_y_axis`
- conversion helpers for generated protobuf vector messages

## Protobuf Types

The generated Rust protobuf types are exported through `core_dump::proto` and are
generated with Prost from the files under `proto/`.

```rust
use core_dump::proto::{CpVector2, Referee, SslWrapperPacket, TrackerWrapperPacket};
use prost::Message;

let point = CpVector2 { x: 1.0, y: 2.0 };

let packet = SslWrapperPacket::decode(&bytes[..])?;
```

The `proto/` directory contains two broad groups of messages:

- `proto/crashpilot/`: project-specific communication between robots,
  CrashPilot, and the interface
- the SSL protocol files used by RoboCup SSL tooling, including vision,
  tracked vision, game-controller state, referee messages, CI, API, RCON, and
  geometry messages

## Regenerating Protobuf Code

Generated code is committed in `src/proto.rs`. After changing files under
`proto/`, regenerate the Rust types with Buf:

```bash
buf generate
```

The generation config is in `buf.gen.yaml` and currently uses the remote
`buf.build/community/neoeinstein-prost` plugin. Keep the checked-in
`src/proto.rs` output in sync with the `.proto` sources.

After regenerating, verify the crate:

```bash
cargo test
cargo fmt
```

## Development

Useful commands:

```bash
cargo check
cargo test
cargo fmt
```

The crate currently targets Rust edition 2024 and depends on:

- `prost` and `prost-types` for protobuf support
- `num-traits` for generic numeric vector operations
- optional `serde` support for vector serialization
