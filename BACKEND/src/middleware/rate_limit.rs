use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use std::time::Duration;
use tower_governor::{
    governor::{GovernorConfig, GovernorConfigBuilder},
    key_extractor::SmartIpKeyExtractor,
};

pub fn build_rate_limit_config()
-> GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(30)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            tracing::info!("Rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    governor_conf
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::get,
    };
    use std::sync::Arc;
    use tower::ServiceExt;
    use tower_governor::GovernorLayer;
    use tower_governor::key_extractor::GlobalKeyExtractor;
    #[tokio::test]
    async fn test_rate_limit_blocking() {
        let governor_conf = Arc::new(
            GovernorConfigBuilder::default()
                .per_second(1)
                .burst_size(2)
                .key_extractor(GlobalKeyExtractor)
                .finish()
                .unwrap(),
        );

        let app = Router::new()
            .route("/test", get(|| async { "OK" }))
            .layer(GovernorLayer::new(governor_conf));

        for _ in 0..2 {
            let response = app
                .clone()
                .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }
        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }
}
