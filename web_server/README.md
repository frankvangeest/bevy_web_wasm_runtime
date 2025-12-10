# Web-Server with support for sharedArrayBuffers

This starts an axum hyper webserver with support for sharedArrayBuffers.

The index.html file contains:
    <meta http-equiv="Cross-Origin-Embedder-Policy" content="require-corp">
    <meta http-equiv="Cross-Origin-Opener-Policy" content="same-origin">

The webserver sets:
    let app = Router::new()
        .fallback_service(static_files)
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-opener-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-embedder-policy"),
            HeaderValue::from_static("require-corp"),
        ));

The sharedArrayBuffers are needed to make use of an increase in GPU performance.
Running it on a webserver that does not support Cross-Origin headers, will reduce GPU performance.

The .htaccess can be used to enable Cross-Origin headers on an Apache web server.


## Building and running the Web Server

1. Run the command in this web_server folder:
```bash
cargo run --release
```