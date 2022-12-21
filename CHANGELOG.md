# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added new proof structure to finish epoch

### Changed
- Removed brkflag CSR
- Replaced minstret by icycleinstret CSR
- Replaced epoch input index with global input index
- Moved hashes from get epoch status to finish epoch
- Renamed voucher.address to voucher.destination

### Removed
- Removed keccak fields from vouchers and notices

## [0.9.0] - 2022-11-17
### Added
- Added microarchitecture configs
- Added TLB configs
- Added read/write virtual memory methods
- Added new CSRs related to the RISC-V specification

### Changed
- Removed DHD device

## [Previous Versions]
- [0.8.0]
- [0.7.0]
- [0.6.0]
- [0.5.0]
- [0.4.0]
- [0.3.0]
- [0.2.0]
- [0.1.3]
- [0.1.2]
- [0.1.1]

[Unreleased]: https://github.com/cartesi/grpc-interfaces/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.9.0
[0.8.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.8.0
[0.7.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.7.0
[0.6.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.6.0
[0.5.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.5.0
[0.4.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.4.0
[0.3.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.3.0
[0.2.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.2.0
[0.1.3]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.1.3
[0.1.2]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.1.2
[0.1.1]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.1.1
