## Irelia 0.3.0

- Rework the way in_game and rest make requests
- Move to hyper_rustls
- Move to workspaces
- Feature gate batched requests, off by default
- Added Tauri feature where error implements ToString and serde::Serialize
- Reworked the error enum to use anonymous fields
- Requires nightly for array_chunks, int_rounding, and lazylock

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