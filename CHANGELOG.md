
## [v0.3.8] - 2021-11-22
### Changed
- Replaced `winapi` with `windows-sys` - `CompletionStatus` now guarantees `#[repr(transparent)]`.

### Internal
- Added CI integration.

## [v0.3.7] - 2021-03-22
### Changed
- Upgrade `rand` dev-dependency from 0.4 -> 0.8
- Upgrade `socket2` dependency from 0.3 to 0.4 and make it a dev-dependency
