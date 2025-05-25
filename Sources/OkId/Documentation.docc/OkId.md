# OkId

A Swift wrapper for the OkId cryptographic hash library.

## Overview

OkId provides a simple interface for creating various types of cryptographic hashes from data. It supports multiple hash algorithms including SHA256, BLAKE3, as well as UUID and ULID generation.

## Topics

### Creating Hashes

- ``createSha256(data:)``
- ``createBlake3(data:)``
- ``createFingerprint(data:)``

### Creating Identifiers

- ``createUuid()``
- ``createUlid()``

### Working with OkId Objects

- ``OkId``
- ``OkId/getDigest()``
- ``OkId/getHashType()``
- ``OkId/newFromString(s:)``

### Utility Functions

- ``getSeparator()``
- ``okidToPathSafe(okid:)``

### Error Handling

- ``OkIdError``