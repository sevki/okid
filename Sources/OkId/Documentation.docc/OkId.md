# ``OkId``

A Swift wrapper for the OkId cryptographic hash library.

## Overview

OkId provides a simple interface for creating various types of cryptographic hashes from data. It supports multiple hash algorithms including SHA1, SHA256, SHA512, SHA3, and BLAKE3.

## Topics

### Creating Hashes

- ``OkId/createSha1(data:)``
- ``OkId/createSha256(data:)``
- ``OkId/createSha512(data:)``
- ``OkId/createSha3_256(data:)``
- ``OkId/createSha3_512(data:)``
- ``OkId/createBlake3(data:)``

### Working with OkId Objects

- ``OkId/hashType``
- ``OkId/digest``
- ``OkId/toString()``