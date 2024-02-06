## Irelia 0.4.2
- Instead of being public, in game types now have getters
- Updated to the latest version of all depedencies
- Fixed simd encoder on latest nightly toolchain

## Irelia 0.4.0
- Rework the request client system
- General imrpovements to the API usability
- Remove a global lock
- General code cleanup
- Implement std::error::Error for LCU Error
- Implement Searialize for LCU Error
- Allow batching system to take multiple types

## Irelia 0.3.0

- Rework the way in_game and rest make requests
- Move to hyper_rustls
- Move to workspaces
- Feature gate batched requests, off by default
- Added Tauri feature where error implements ToString and serde::Serialize
- Reworked the error enum to use anonymous fields
- Requires nightly for array_chunks, int_rounding, and lazylock
- Rewrite and re-export the base64 encoder

## Irelia 0.2.0

- Updated Websocket System
- Websocket now implements stream
- Websocket now has proper encapsalation
- Websocket makes use of enums to represent callbacks
- Implement batching system for LCU requests
- Improve docs
- Improve cargo.toml
- Implement tests for base64 encoder
- General code cleanup