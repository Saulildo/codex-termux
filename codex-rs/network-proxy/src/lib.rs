#![deny(clippy::print_stdout, clippy::print_stderr)]

#[cfg(target_os = "android")]
mod android_stub;
#[cfg(not(target_os = "android"))]
mod certs;
#[cfg(not(target_os = "android"))]
mod config;
#[cfg(not(target_os = "android"))]
mod http_proxy;
#[cfg(not(target_os = "android"))]
mod mitm;
#[cfg(not(target_os = "android"))]
mod network_policy;
#[cfg(not(target_os = "android"))]
mod policy;
#[cfg(not(target_os = "android"))]
mod proxy;
#[cfg(not(target_os = "android"))]
mod reasons;
#[cfg(not(target_os = "android"))]
mod responses;
#[cfg(not(target_os = "android"))]
mod runtime;
#[cfg(not(target_os = "android"))]
mod socks5;
#[cfg(not(target_os = "android"))]
mod state;
#[cfg(not(target_os = "android"))]
mod upstream;

#[cfg(target_os = "android")]
pub use android_stub::*;
#[cfg(not(target_os = "android"))]
pub use config::NetworkDomainPermission;
#[cfg(not(target_os = "android"))]
pub use config::NetworkDomainPermissionEntry;
#[cfg(not(target_os = "android"))]
pub use config::NetworkDomainPermissions;
#[cfg(not(target_os = "android"))]
pub use config::NetworkMode;
#[cfg(not(target_os = "android"))]
pub use config::NetworkProxyConfig;
#[cfg(not(target_os = "android"))]
pub use config::NetworkUnixSocketPermission;
#[cfg(not(target_os = "android"))]
pub use config::NetworkUnixSocketPermissions;
#[cfg(not(target_os = "android"))]
pub use config::host_and_port_from_network_addr;
#[cfg(not(target_os = "android"))]
pub use network_policy::NetworkDecision;
#[cfg(not(target_os = "android"))]
pub use network_policy::NetworkDecisionSource;
#[cfg(not(target_os = "android"))]
pub use network_policy::NetworkPolicyDecider;
#[cfg(not(target_os = "android"))]
pub use network_policy::NetworkPolicyDecision;
#[cfg(not(target_os = "android"))]
pub use network_policy::NetworkPolicyRequest;
#[cfg(not(target_os = "android"))]
pub use network_policy::NetworkPolicyRequestArgs;
#[cfg(not(target_os = "android"))]
pub use network_policy::NetworkProtocol;
#[cfg(not(target_os = "android"))]
pub use policy::normalize_host;
#[cfg(not(target_os = "android"))]
pub use proxy::ALL_PROXY_ENV_KEYS;
#[cfg(not(target_os = "android"))]
pub use proxy::ALLOW_LOCAL_BINDING_ENV_KEY;
#[cfg(not(target_os = "android"))]
pub use proxy::Args;
#[cfg(not(target_os = "android"))]
pub use proxy::DEFAULT_NO_PROXY_VALUE;
#[cfg(not(target_os = "android"))]
pub use proxy::NO_PROXY_ENV_KEYS;
#[cfg(not(target_os = "android"))]
pub use proxy::NetworkProxy;
#[cfg(not(target_os = "android"))]
pub use proxy::NetworkProxyBuilder;
#[cfg(not(target_os = "android"))]
pub use proxy::NetworkProxyHandle;
#[cfg(not(target_os = "android"))]
pub use proxy::PROXY_URL_ENV_KEYS;
#[cfg(not(target_os = "android"))]
pub use proxy::has_proxy_url_env_vars;
#[cfg(not(target_os = "android"))]
pub use proxy::proxy_url_env_value;
#[cfg(not(target_os = "android"))]
pub use runtime::BlockedRequest;
#[cfg(not(target_os = "android"))]
pub use runtime::BlockedRequestArgs;
#[cfg(not(target_os = "android"))]
pub use runtime::BlockedRequestObserver;
#[cfg(not(target_os = "android"))]
pub use runtime::ConfigReloader;
#[cfg(not(target_os = "android"))]
pub use runtime::ConfigState;
#[cfg(not(target_os = "android"))]
pub use runtime::NetworkProxyState;
#[cfg(not(target_os = "android"))]
pub use state::NetworkProxyAuditMetadata;
#[cfg(not(target_os = "android"))]
pub use state::NetworkProxyConstraintError;
#[cfg(not(target_os = "android"))]
pub use state::NetworkProxyConstraints;
#[cfg(not(target_os = "android"))]
pub use state::PartialNetworkConfig;
#[cfg(not(target_os = "android"))]
pub use state::PartialNetworkProxyConfig;
#[cfg(not(target_os = "android"))]
pub use state::build_config_state;
#[cfg(not(target_os = "android"))]
pub use state::validate_policy_against_constraints;
