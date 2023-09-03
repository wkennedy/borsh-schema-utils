# borsh-schema-utils

[![Rust](https://github.com/wkennedy/borsh-schema-utils/actions/workflows/rust.yml/badge.svg)](https://github.com/wkennedy/borsh-schema-utils/actions/workflows/rust.yml) :: [![codecov](https://codecov.io/gh/wkennedy/borsh-schema-utils/graph/badge.svg?token=R0RJQC1E76)](https://codecov.io/gh/wkennedy/borsh-schema-utils)

These are two libraries that provide additional utility for serializing and deserializing data with Borsh using the
BorshSchemaContainer. Use cases where this is useful include when you want to serialize/deserialize data where you might
not know the schema ahead of time, or are not able to use create or compile code to support the serialized data.

[borsh-schema-writer](./borsh-schema-writer/) - [README](./borsh-schema-writer/README.md)


[borsh-serde-adapter](./borsh-serde-adapter/) - [README](./borsh-serde-adapter/README.md)


For example uses, please see the integration_test files in each library.