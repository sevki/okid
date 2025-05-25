# TypeScript

OkId provides TypeScript/JavaScript bindings through WebAssembly, enabling you to use OkId in Node.js and browser environments.

## Installation

Install the package from npm:

```bash
npm install okid
```

Or using yarn:

```bash
yarn add okid
```

## Usage

### Node.js

```typescript
import { OkId } from 'okid';

// Create OkIds from different hash functions
const data = new TextEncoder().encode('Hello, World!');

// SHA256
const sha256Id = OkId.fromSha256(data);
console.log('SHA256:', sha256Id.toString());

// Blake3
const blake3Id = OkId.fromBlake3(data);
console.log('Blake3:', blake3Id.toString());

// Fingerprint
const fingerprintId = OkId.fingerprint(data);
console.log('Fingerprint:', fingerprintId.toString());

// Generate UUIDs and ULIDs
const uuid = OkId.newUuid();
console.log('UUID:', uuid.toString());

const ulid = OkId.newUlid();
console.log('ULID:', ulid.toString());
```

### Browser

```html
<script type="module">
    import init, { OkId } from './node_modules/okid/okid.js';
    
    async function run() {
        // Initialize the WASM module
        await init();
        
        // Now you can use OkId
        const data = new TextEncoder().encode('Hello from browser!');
        const okid = OkId.fromSha256(data);
        console.log(okid.toString());
    }
    
    run();
</script>
```

## API Reference

### Creating OkIds

#### `OkId.fromSha256(data: Uint8Array): OkId`
Creates an OkId using SHA256 hash of the provided data.

#### `OkId.fromBlake3(data: Uint8Array): OkId`
Creates an OkId using Blake3 hash of the provided data.

#### `OkId.fingerprint(data: Uint8Array): OkId`
Creates an OkId using a fingerprinting algorithm.

#### `OkId.newUuid(): OkId`
Generates a new UUID-based OkId.

#### `OkId.newUlid(): OkId`
Generates a new ULID-based OkId.

### Parsing and Converting

#### `OkId.fromString(s: string): OkId`
Parses an OkId from its string representation.

```typescript
const okid = OkId.fromString('2:b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9');
```

#### `okid.toString(): string`
Returns the string representation of the OkId.

#### `okid.toPathSafe(): string`
Returns a path-safe representation of the OkId (using `-` as separator).

#### `okid.display_safe(): string`
Returns a display-safe representation of the OkId.

#### `OkId.from_display_safe(s: string): OkId`
Creates an OkId from a display-safe string.

### Properties

#### `okid.hashType(): string`
Returns the hash type character of the OkId.

## Examples

### Hashing Files

```typescript
import { readFile } from 'fs/promises';
import { OkId } from 'okid';

async function hashFile(path: string) {
    const buffer = await readFile(path);
    const data = new Uint8Array(buffer);
    
    const sha256 = OkId.fromSha256(data);
    const blake3 = OkId.fromBlake3(data);
    
    console.log(`SHA256: ${sha256.toString()}`);
    console.log(`Blake3: ${blake3.toString()}`);
    console.log(`Path-safe: ${sha256.toPathSafe()}`);
}
```

### Round-trip Conversion

```typescript
// Create an OkId
const original = OkId.fromSha256(new TextEncoder().encode('test'));

// Convert to string and back
const str = original.toString();
const parsed = OkId.fromString(str);

console.log(original.toString() === parsed.toString()); // true

// Convert to path-safe format
const pathSafe = original.toPathSafe();
console.log(pathSafe); // Uses '-' as separator
```

### Working with UUIDs and ULIDs

```typescript
// Generate multiple UUIDs
const uuids = Array.from({ length: 5 }, () => OkId.newUuid());
uuids.forEach((uuid, i) => {
    console.log(`UUID ${i}: ${uuid.toString()}`);
});

// Generate time-sorted ULIDs
const ulids = Array.from({ length: 5 }, () => OkId.newUlid());
ulids.forEach((ulid, i) => {
    console.log(`ULID ${i}: ${ulid.toString()}`);
});
```

## TypeScript Types

The package includes full TypeScript definitions. The main types are:

```typescript
export class OkId {
    static fromSha256(data: Uint8Array): OkId;
    static fromBlake3(data: Uint8Array): OkId;
    static fingerprint(data: Uint8Array): OkId;
    static newUuid(): OkId;
    static newUlid(): OkId;
    static fromString(s: string): OkId;
    static from_display_safe(s: string): OkId;
    
    toString(): string;
    toPathSafe(): string;
    display_safe(): string;
    hashType(): string;
}
```
