pub mod cors_setting {
    use axum::http::{
        Method
    };
    use http::header::{CONTENT_TYPE};
    use tower_http::cors::{Any, CorsLayer};
    

    pub fn cors_setting() -> CorsLayer {
        let cors: CorsLayer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any)
            .allow_headers([CONTENT_TYPE]);

        cors
    }
}