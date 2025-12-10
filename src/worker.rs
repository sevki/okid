#![cfg(target_arch = "wasm32")]
use worker::{
    crypto::{DigestStream, DigestStreamAlgorithm},
    worker_sys::web_sys,
    Error as WorkerError, Result as WorkerResult,
};

use crate::{BinaryType, Digest, OkId};

/// Internal helper that maps an `OkId` digest type to the corresponding Cloudflare
/// `DigestStreamAlgorithm`, returning the algorithm and the raw digest bytes.
fn worker_digest_info(okid: &OkId) -> WorkerResult<(DigestStreamAlgorithm, &[u8])> {
    match &okid.digest {
        #[cfg(feature = "sha2")]
        Digest::Sha256(d) => Ok((DigestStreamAlgorithm::Sha256, &d.0[..])),
        _ => Err(WorkerError::RustError(format!(
            "OkId type '{}' is not supported by Cloudflare DigestStream",
            okid.hash_type
        ))),
    }
}

/// Convert the provided raw digest bytes into an `OkId`, using the supplied algorithm.
fn okid_from_worker_digest_bytes(
    bytes: &[u8],
    algorithm: DigestStreamAlgorithm,
) -> WorkerResult<OkId> {
    match algorithm {
        #[cfg(feature = "sha2")]
        DigestStreamAlgorithm::Sha256 => {
            if bytes.len() != 32 {
                return Err(WorkerError::RustError(format!(
                    "SHA-256 digest must be 32 bytes, got {}",
                    bytes.len()
                )));
            }
            let mut buf = [0u8; 32];
            buf.copy_from_slice(bytes);
            Ok(OkId {
                hash_type: BinaryType::Sha256,
                digest: Digest::Sha256(crate::sha2::Sha256(buf)),
            })
        }
        _ => Err(WorkerError::RustError(format!(
            "DigestStreamAlgorithm '{:?}' is not supported by Cloudflare OkId",
            algorithm
        ))),
    }
}

#[cfg(feature = "worker")]
impl OkId {
    /// Construct an `OkId` by hashing the provided readable stream inside a Cloudflare Worker.
    ///
    /// This uses the Worker runtime `crypto.DigestStream` API and supports the same
    /// algorithms that OkId exposes for Cloudflare (`SHA-1` and `SHA-256`).
    pub async fn from_worker_stream(
        stream: &web_sys::ReadableStream,
        algorithm: DigestStreamAlgorithm,
    ) -> WorkerResult<Self> {
        let digest_stream = DigestStream::new(algorithm);
        let _ = stream.pipe_to(digest_stream.raw());
        let digest = digest_stream.digest().await?.to_vec();
        okid_from_worker_digest_bytes(&digest, algorithm)
    }

    /// Verify that the readable stream's digest matches the current `OkId`.
    ///
    /// ```rust,no_run
    /// use worker::{
    ///     crypto::{DigestStream, DigestStreamAlgorithm},
    ///     worker_sys::web_sys,
    /// };
    ///
    /// # async fn example(
    /// #     stream: &web_sys::ReadableStream,
    /// #     expected: okid::OkId,
    /// # ) -> worker::Result<bool> {
    /// let matches = expected.verify_worker_stream(stream).await?;
    /// # Ok(matches)
    /// # }
    /// ```
    pub async fn verify_worker_stream(
        &self,
        stream: &web_sys::ReadableStream,
    ) -> WorkerResult<bool> {
        let (algorithm, expected) = worker_digest_info(self)?;
        let digest_stream = DigestStream::new(algorithm);
        let _ = stream.pipe_to(digest_stream.raw());
        let digest = digest_stream.digest().await?.to_vec();
        Ok(digest.as_slice() == expected)
    }
}
