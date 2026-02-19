use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use std::time::Duration;
use tower_governor::{
    governor::{GovernorConfig, GovernorConfigBuilder},
    key_extractor::PeerIpKeyExtractor,
};
pub fn build_rate_limit_config() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>
{
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    governor_conf
}
