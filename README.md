> :warning: The Cartesi team keeps working internally on the next version of this repository, following its regular development roadmap. Whenever there's a new version ready or important fix, these are published to the public source tree as new releases.

# Cartesi GRPC Interfaces

The Cartesi GRPC Interfaces repository contains all GRPC and Protobuf definitions used in the GRPC interfaces of the Cartesi Project modules. Currently these comprehend:

- A definition of basic message types used in multiple interfaces (cartesi-base.proto)
- A definition of the services exported by the machine emulator and used by the machine manager (core.proto)
- A definition of the services and higher level message types used by the machine emulator to interact with the machine manager (manager-low.proto) 
- A definition of the services and higher level message types used to interact with the machine manager sessions (manager-high.proto) 
- A definition of the services and higher level message types used to interact with the logger manager sessions (logger-high.proto)

## Getting Started

This repository is not intended for standalone usage. Every repository that makes use of a GRPC interface, either serving or consuming a certain API, includes this repository as submodule and builds the language specific auto-generated code that implements the desired services and messages. Specifics on those can be checked in the individual repositories that include this as a submodule.

## Generate Python Code

### Requirements

- Python >= 3.6
- Python module `grpcio-tools`

### Run Script to Generate Python Code

```shell
./generate_python_grpc_code.sh
```

The generated Python modules can be found in the `py` directory.

## Contributing

Thank you for your interest in Cartesi! Head over to our [Contributing Guidelines](https://github.com/cartesi/grpc-interfaces/blob/master/CONTRIBUTING.md) for instructions on how to sign our Contributors Agreement and get started with Cartesi!

Please note we have a [Code of Conduct](https://github.com/cartesi/grpc-interfaces/blob/master/CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Authors

* *Diego Nehab*
* *Carlo Fragni*
* *Augusto Teixeira*

## License

The grpc-interfaces repository and all contributions are licensed under [APACHE 2.0](https://www.apache.org/licenses/LICENSE-2.0). Please review our [LICENSE](https://github.com/cartesi/grpc-interfaces/blob/master/LICENSE) file.

## Acknowledgments

- Original work 
