# Issue draft: an http:// S3 endpoint fails silently

For rivet-dev/rivet. Not filed. Before filing: retest on the latest release and search the issues again.

---

**Title:** 3.0.0: an `http://` S3 endpoint makes the engine hang without a request or an error, then exit with "config operation timed out"

## Environment

`rivetdev/engine:3.0.0`, with `object_storage.primary.s3.endpoint` set to an S3-compatible store over plain HTTP (`http://s3:7070`, VersityGW in Docker Compose). A local MinIO also serves plain HTTP by default.

## What happens

The engine starts, makes no request to the bucket, logs nothing about it, and after a while exits with `config operation timed out`. Nothing points at the bucket. Setting `AWS_ALLOW_HTTP=true` in the engine's environment fixes it.

## Cause

`engine/packages/engine-context/src/production/object_storage.rs` builds the S3 client with `AmazonS3Builder::from_env()` and `with_endpoint(endpoint)`, but never sets `allow_http`. The `object_store` crate refuses `http://` URLs unless `allow_http` is set, which `from_env` only does when `AWS_ALLOW_HTTP` is in the environment. The error it returns isn't logged on the path that ends in the timeout.

## Suggested fix

Allow HTTP when the configured endpoint asks for it:

```rust
if let Some(endpoint) = &s3.endpoint {
    builder = builder
        .with_endpoint(endpoint)
        .with_allow_http(endpoint.starts_with("http://"));
}
```

And log the object store's error where startup gives up, so the next misconfiguration names itself. The [storage page](https://rivet.dev/docs/deploy/self-host/control-plane/storage/) shows only an `https://` endpoint for S3-compatible stores; until the code changes, it should mention `AWS_ALLOW_HTTP`.
