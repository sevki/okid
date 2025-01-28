<img src="https://assets.ok.software/okid.png" align="right" width="200">

# okid
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fsevki%2Fokid.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fsevki%2Fokid?ref=badge_shield)


`okid` is a library for generating double clickable representations of various types of data,
such as `sha1` hashes, `uuid`s and more.

## sha1
```rust
#[cfg(feature = "sha1")]
{
    use sha1::Digest as sha1digest;
    let hasher = sha1::Sha1::new();
    let binary_id = okid::OkId::from(hasher);
}
```
## sha256
```rust
#[cfg(feature = "sha2")]
{
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hello world");
    let binary_id = okid::OkId::from(hasher);
}
```

The resulting strings look like this:
`2ː00b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9`
first character of the string is the type of the binary data
in this case 2 means sha256
the rest of the string is the hexadecimal representation of the binary data


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fsevki%2Fokid.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fsevki%2Fokid?ref=badge_large)