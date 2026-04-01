//! Android/Termux stub implementation.
//!
//! The network proxy runtime (Rama-based) is not supported on Android/Termux,
//! but the rest of the workspace depends on these types unconditionally.
//! Provide a small, no-op implementation so Codex can run with networking
//! effectively "unmanaged" on Android.

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum NetworkMode {
    Limited,
    #[default]
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct NetworkProxyConfig {
    #[serde(default)]
    pub network: NetworkProxySettings,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum NetworkDomainPermission {
    None,
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkDomainPermissionEntry {
    pub pattern: String,
    pub permission: NetworkDomainPermission,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetworkDomainPermissions {
    pub entries: Vec<NetworkDomainPermissionEntry>,
}

impl Serialize for NetworkDomainPermissions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.effective_entries()
            .into_iter()
            .map(|entry| (entry.pattern, entry.permission))
            .collect::<BTreeMap<_, _>>()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NetworkDomainPermissions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries = BTreeMap::<String, NetworkDomainPermission>::deserialize(deserializer)?
            .into_iter()
            .map(|(pattern, permission)| NetworkDomainPermissionEntry {
                pattern,
                permission,
            })
            .collect();
        Ok(Self { entries })
    }
}

impl NetworkDomainPermissions {
    fn effective_entries(&self) -> Vec<NetworkDomainPermissionEntry> {
        let mut order = Vec::new();
        let mut effective_permissions = BTreeMap::new();

        for entry in &self.entries {
            if !effective_permissions.contains_key(&entry.pattern) {
                order.push(entry.pattern.clone());
            }

            let permission = effective_permissions
                .entry(entry.pattern.clone())
                .or_insert(entry.permission);
            if entry.permission > *permission {
                *permission = entry.permission;
            }
        }

        order
            .into_iter()
            .filter_map(|pattern| {
                effective_permissions.remove(&pattern).map(|permission| {
                    NetworkDomainPermissionEntry {
                        pattern,
                        permission,
                    }
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum NetworkUnixSocketPermission {
    Allow,
    None,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkUnixSocketPermissions {
    #[serde(flatten)]
    pub entries: BTreeMap<String, NetworkUnixSocketPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NetworkProxySettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub proxy_url: String,
    #[serde(default)]
    pub enable_socks5: bool,
    #[serde(default)]
    pub socks_url: String,
    #[serde(default)]
    pub enable_socks5_udp: bool,
    #[serde(default)]
    pub allow_upstream_proxy: bool,
    #[serde(default)]
    pub dangerously_allow_non_loopback_proxy: bool,
    #[serde(default)]
    pub dangerously_allow_all_unix_sockets: bool,
    #[serde(default)]
    pub mode: NetworkMode,
    #[serde(default)]
    pub domains: Option<NetworkDomainPermissions>,
    #[serde(default)]
    pub unix_sockets: Option<NetworkUnixSocketPermissions>,
    #[serde(default)]
    pub allow_local_binding: bool,
    #[serde(default)]
    pub mitm: bool,
}

impl Default for NetworkProxySettings {
    fn default() -> Self {
        Self {
            enabled: false,
            proxy_url: "http://127.0.0.1:3128".to_string(),
            enable_socks5: true,
            socks_url: "http://127.0.0.1:8081".to_string(),
            enable_socks5_udp: true,
            allow_upstream_proxy: true,
            dangerously_allow_non_loopback_proxy: false,
            dangerously_allow_all_unix_sockets: false,
            mode: NetworkMode::default(),
            domains: None,
            unix_sockets: None,
            allow_local_binding: false,
            mitm: false,
        }
    }
}

impl NetworkProxySettings {
    pub fn allowed_domains(&self) -> Option<Vec<String>> {
        self.domain_entries(NetworkDomainPermission::Allow)
    }

    pub fn denied_domains(&self) -> Option<Vec<String>> {
        self.domain_entries(NetworkDomainPermission::Deny)
    }

    pub fn allow_unix_sockets(&self) -> Vec<String> {
        self.unix_sockets
            .as_ref()
            .map(|unix_sockets| {
                unix_sockets
                    .entries
                    .iter()
                    .filter(|(_, permission)| matches!(permission, NetworkUnixSocketPermission::Allow))
                    .map(|(path, _)| path.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn set_allowed_domains(&mut self, allowed_domains: Vec<String>) {
        self.set_domain_entries(allowed_domains, NetworkDomainPermission::Allow);
    }

    pub fn set_denied_domains(&mut self, denied_domains: Vec<String>) {
        self.set_domain_entries(denied_domains, NetworkDomainPermission::Deny);
    }

    pub fn upsert_domain_permission(
        &mut self,
        host: String,
        permission: NetworkDomainPermission,
        normalize: impl Fn(&str) -> String,
    ) {
        let mut domains = self.domains.take().unwrap_or_default();
        let normalized_host = normalize(&host);
        domains
            .entries
            .retain(|entry| normalize(&entry.pattern) != normalized_host);
        domains.entries.push(NetworkDomainPermissionEntry {
            pattern: host,
            permission,
        });
        self.domains = (!domains.entries.is_empty()).then_some(domains);
    }

    pub fn set_allow_unix_sockets(&mut self, allow_unix_sockets: Vec<String>) {
        self.set_unix_socket_entries(allow_unix_sockets, NetworkUnixSocketPermission::Allow);
    }

    fn domain_entries(&self, permission: NetworkDomainPermission) -> Option<Vec<String>> {
        self.domains
            .as_ref()
            .map(|domains| {
                domains
                    .effective_entries()
                    .iter()
                    .filter(|entry| entry.permission == permission)
                    .map(|entry| entry.pattern.clone())
                    .collect()
            })
            .filter(|entries: &Vec<String>| !entries.is_empty())
    }

    fn set_domain_entries(&mut self, entries: Vec<String>, permission: NetworkDomainPermission) {
        let mut domains = self.domains.take().unwrap_or_default();
        domains
            .entries
            .retain(|entry| entry.permission != permission);
        for entry in entries {
            if !domains
                .entries
                .iter()
                .any(|existing| existing.pattern == entry && existing.permission == permission)
            {
                domains.entries.push(NetworkDomainPermissionEntry {
                    pattern: entry,
                    permission,
                });
            }
        }
        self.domains = (!domains.entries.is_empty()).then_some(domains);
    }

    fn set_unix_socket_entries(
        &mut self,
        entries: Vec<String>,
        permission: NetworkUnixSocketPermission,
    ) {
        let mut unix_sockets = self.unix_sockets.take().unwrap_or_default();
        unix_sockets
            .entries
            .retain(|_, existing| *existing != permission);
        for entry in entries {
            unix_sockets.entries.insert(entry, permission);
        }
        self.unix_sockets = (!unix_sockets.entries.is_empty()).then_some(unix_sockets);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NetworkProxyConstraints {
    pub enabled: Option<bool>,
    pub mode: Option<NetworkMode>,
    pub allow_upstream_proxy: Option<bool>,
    pub dangerously_allow_non_loopback_proxy: Option<bool>,
    pub dangerously_allow_all_unix_sockets: Option<bool>,
    pub allowed_domains: Option<Vec<String>>,
    pub allowlist_expansion_enabled: Option<bool>,
    pub denied_domains: Option<Vec<String>>,
    pub denylist_expansion_enabled: Option<bool>,
    pub allow_unix_sockets: Option<Vec<String>>,
    pub allow_local_binding: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PartialNetworkProxyConfig {
    #[serde(default)]
    pub network: PartialNetworkConfig,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PartialNetworkConfig {
    pub enabled: Option<bool>,
    pub mode: Option<NetworkMode>,
    pub allow_upstream_proxy: Option<bool>,
    pub dangerously_allow_non_loopback_proxy: Option<bool>,
    pub dangerously_allow_all_unix_sockets: Option<bool>,
    #[serde(default)]
    pub domains: Option<NetworkDomainPermissions>,
    #[serde(default)]
    pub unix_sockets: Option<NetworkUnixSocketPermissions>,
    #[serde(default)]
    pub allow_local_binding: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NetworkProxyConstraintError {
    message: String,
}

impl NetworkProxyConstraintError {
    pub fn into_anyhow(self) -> anyhow::Error {
        anyhow::anyhow!(self.message)
    }
}

impl std::fmt::Display for NetworkProxyConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl std::error::Error for NetworkProxyConstraintError {}

pub fn validate_policy_against_constraints(
    _config: &NetworkProxyConfig,
    _constraints: &NetworkProxyConstraints,
) -> std::result::Result<(), NetworkProxyConstraintError> {
    // Android/Termux: no managed proxy enforcement, but keep config loading working.
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct ConfigState {
    pub config: NetworkProxyConfig,
    pub constraints: NetworkProxyConstraints,
}

pub fn build_config_state(
    config: NetworkProxyConfig,
    constraints: NetworkProxyConstraints,
) -> Result<ConfigState> {
    Ok(ConfigState {
        config,
        constraints,
    })
}

#[async_trait]
pub trait ConfigReloader: Send + Sync {
    fn source_label(&self) -> String;
    async fn maybe_reload(&self) -> Result<Option<ConfigState>>;
    async fn reload_now(&self) -> Result<ConfigState>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NetworkProxyAuditMetadata {
    pub conversation_id: Option<String>,
    pub app_version: Option<String>,
    pub user_account_id: Option<String>,
    pub auth_mode: Option<String>,
    pub originator: Option<String>,
    pub user_email: Option<String>,
    pub terminal_type: Option<String>,
    pub model: Option<String>,
    pub slug: Option<String>,
}

#[derive(Clone)]
pub struct NetworkProxyState {
    state: ConfigState,
    _reloader: Arc<dyn ConfigReloader>,
    _audit_metadata: NetworkProxyAuditMetadata,
}

impl std::fmt::Debug for NetworkProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Avoid printing trait object details; this is a stub on Android anyway.
        f.debug_struct("NetworkProxyState").finish_non_exhaustive()
    }
}

impl NetworkProxyState {
    pub fn with_reloader(state: ConfigState, reloader: Arc<dyn ConfigReloader>) -> Self {
        Self {
            state,
            _reloader: reloader,
            _audit_metadata: NetworkProxyAuditMetadata::default(),
        }
    }

    pub fn with_reloader_and_audit_metadata(
        state: ConfigState,
        reloader: Arc<dyn ConfigReloader>,
        audit_metadata: NetworkProxyAuditMetadata,
    ) -> Self {
        Self {
            state,
            _reloader: reloader,
            _audit_metadata: audit_metadata,
        }
    }

    pub fn current_cfg(&self) -> Result<NetworkProxyConfig> {
        Ok(self.state.config.clone())
    }

    pub fn enabled(&self) -> Result<bool> {
        Ok(false)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Args;

impl Args {
    pub fn parse() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct NetworkProxy {
    _state: Option<Arc<NetworkProxyState>>,
    http_addr: SocketAddr,
    socks_addr: SocketAddr,
    admin_addr: SocketAddr,
}

impl Default for NetworkProxy {
    fn default() -> Self {
        Self {
            _state: None,
            http_addr: SocketAddr::from(([127, 0, 0, 1], 3128)),
            socks_addr: SocketAddr::from(([127, 0, 0, 1], 8081)),
            admin_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct NetworkProxyBuilder {
    state: Option<Arc<NetworkProxyState>>,
}

#[derive(Debug, Default, Clone)]
pub struct NetworkProxyHandle;

impl NetworkProxy {
    pub fn builder() -> NetworkProxyBuilder {
        NetworkProxyBuilder { state: None }
    }

    pub fn http_addr(&self) -> SocketAddr {
        self.http_addr
    }

    pub fn socks_addr(&self) -> SocketAddr {
        self.socks_addr
    }

    pub fn admin_addr(&self) -> SocketAddr {
        self.admin_addr
    }

    pub async fn add_allowed_domain(&self, _host: &str) -> Result<()> {
        Ok(())
    }

    pub async fn add_denied_domain(&self, _host: &str) -> Result<()> {
        Ok(())
    }

    pub async fn current_cfg(&self) -> Result<NetworkProxyConfig> {
        Ok(self
            ._state
            .as_ref()
            .map(|state| state.state.config.clone())
            .unwrap_or_default())
    }

    pub fn apply_to_env(&self, _env: &mut HashMap<String, String>) {
        // Android/Termux: managed proxy is disabled; do not inject proxy env vars.
    }

    pub fn apply_to_env_for_attempt(
        &self,
        _env: &mut HashMap<String, String>,
        _attempt_id: Option<&str>,
    ) {
        // Android/Termux: no-op
    }
}

impl NetworkProxyBuilder {
    pub fn state(mut self, state: Arc<NetworkProxyState>) -> Self {
        self.state = Some(state);
        self
    }

    pub fn policy_decider<D>(self, _decider: D) -> Self
    where
        D: NetworkPolicyDecider,
    {
        self
    }

    pub fn policy_decider_arc(self, _decider: Arc<dyn NetworkPolicyDecider>) -> Self {
        self
    }

    pub fn blocked_request_observer<O>(self, _observer: O) -> Self
    where
        O: BlockedRequestObserver,
    {
        self
    }

    pub fn blocked_request_observer_arc(self, _observer: Arc<dyn BlockedRequestObserver>) -> Self {
        self
    }

    pub async fn build(self) -> Result<NetworkProxy> {
        Ok(NetworkProxy {
            _state: self.state,
            http_addr: SocketAddr::from(([127, 0, 0, 1], 3128)),
            socks_addr: SocketAddr::from(([127, 0, 0, 1], 8081)),
            admin_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
        })
    }
}

impl NetworkProxy {
    pub async fn run(&self) -> Result<NetworkProxyHandle> {
        Ok(NetworkProxyHandle)
    }
}

impl NetworkProxyHandle {
    pub async fn wait(self) -> Result<()> {
        Ok(())
    }
}

pub fn host_and_port_from_network_addr(network_addr: &str, default_port: u16) -> String {
    let trimmed = network_addr.trim();
    if trimmed.is_empty() {
        return format!("127.0.0.1:{default_port}");
    }

    let without_scheme = trimmed
        .split("://")
        .nth(1)
        .unwrap_or(trimmed)
        .split('/')
        .next()
        .unwrap_or(trimmed);

    if without_scheme.contains(':') {
        without_scheme.to_string()
    } else {
        format!("{without_scheme}:{default_port}")
    }
}

pub fn normalize_host(host: &str) -> String {
    let host = host.trim();
    if host.starts_with('[')
        && let Some(end) = host.find(']')
    {
        return normalize_dns_host(&host[1..end]);
    }

    if host.bytes().filter(|b| *b == b':').count() == 1 {
        let host = host.split(':').next().unwrap_or_default();
        return normalize_dns_host(host);
    }

    normalize_dns_host(host)
}

fn normalize_dns_host(host: &str) -> String {
    let host = host.to_ascii_lowercase();
    host.trim_end_matches('.').to_string()
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NetworkPolicyDecision {
    Deny,
    Ask,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkDecisionSource {
    BaselinePolicy,
    ModeGuard,
    ProxyState,
    Decider,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkDecision {
    Allow,
    Deny {
        reason: String,
        source: NetworkDecisionSource,
        decision: NetworkPolicyDecision,
    },
}

impl NetworkDecision {
    pub fn deny(reason: impl Into<String>) -> Self {
        Self::deny_with_source(reason, NetworkDecisionSource::Decider)
    }

    pub fn ask(reason: impl Into<String>) -> Self {
        Self::ask_with_source(reason, NetworkDecisionSource::Decider)
    }

    pub fn deny_with_source(reason: impl Into<String>, source: NetworkDecisionSource) -> Self {
        let reason = reason.into();
        Self::Deny {
            reason: if reason.is_empty() {
                "policy_denied".to_string()
            } else {
                reason
            },
            source,
            decision: NetworkPolicyDecision::Deny,
        }
    }

    pub fn ask_with_source(reason: impl Into<String>, source: NetworkDecisionSource) -> Self {
        let reason = reason.into();
        Self::Deny {
            reason: if reason.is_empty() {
                "policy_denied".to_string()
            } else {
                reason
            },
            source,
            decision: NetworkPolicyDecision::Ask,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkProtocol {
    Http,
    HttpsConnect,
    Socks5Tcp,
    Socks5Udp,
}

impl NetworkProtocol {
    pub const fn as_policy_protocol(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::HttpsConnect => "https_connect",
            Self::Socks5Tcp => "socks5_tcp",
            Self::Socks5Udp => "socks5_udp",
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetworkPolicyRequest {
    pub protocol: NetworkProtocol,
    pub host: String,
    pub port: u16,
    pub client_addr: Option<String>,
    pub method: Option<String>,
    pub command: Option<String>,
    pub exec_policy_hint: Option<String>,
    pub attempt_id: Option<String>,
}

pub struct NetworkPolicyRequestArgs {
    pub protocol: NetworkProtocol,
    pub host: String,
    pub port: u16,
    pub client_addr: Option<String>,
    pub method: Option<String>,
    pub command: Option<String>,
    pub exec_policy_hint: Option<String>,
    pub attempt_id: Option<String>,
}

impl NetworkPolicyRequest {
    pub fn new(args: NetworkPolicyRequestArgs) -> Self {
        let NetworkPolicyRequestArgs {
            protocol,
            host,
            port,
            client_addr,
            method,
            command,
            exec_policy_hint,
            attempt_id,
        } = args;
        Self {
            protocol,
            host,
            port,
            client_addr,
            method,
            command,
            exec_policy_hint,
            attempt_id,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BlockedRequest {
    pub host: String,
    pub reason: String,
    pub client: Option<String>,
    pub method: Option<String>,
    pub mode: Option<NetworkMode>,
    pub protocol: String,
    pub attempt_id: Option<String>,
    pub decision: Option<String>,
    pub source: Option<String>,
    pub port: Option<u16>,
    pub timestamp: i64,
}

pub struct BlockedRequestArgs {
    pub host: String,
    pub reason: String,
    pub client: Option<String>,
    pub method: Option<String>,
    pub mode: Option<NetworkMode>,
    pub protocol: String,
    pub attempt_id: Option<String>,
    pub decision: Option<String>,
    pub source: Option<String>,
    pub port: Option<u16>,
}

impl BlockedRequest {
    pub fn new(args: BlockedRequestArgs) -> Self {
        let BlockedRequestArgs {
            host,
            reason,
            client,
            method,
            mode,
            protocol,
            attempt_id,
            decision,
            source,
            port,
        } = args;
        Self {
            host,
            reason,
            client,
            method,
            mode,
            protocol,
            attempt_id,
            decision,
            source,
            port,
            timestamp: unix_timestamp(),
        }
    }
}

#[async_trait]
pub trait BlockedRequestObserver: Send + Sync + 'static {
    async fn on_blocked_request(&self, _request: BlockedRequest);
}

#[async_trait]
impl<O: BlockedRequestObserver + ?Sized> BlockedRequestObserver for Arc<O> {
    async fn on_blocked_request(&self, request: BlockedRequest) {
        (**self).on_blocked_request(request).await;
    }
}

#[async_trait]
impl<F, Fut> BlockedRequestObserver for F
where
    F: Fn(BlockedRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
{
    async fn on_blocked_request(&self, request: BlockedRequest) {
        (self)(request).await;
    }
}

#[async_trait]
pub trait NetworkPolicyDecider: Send + Sync + 'static {
    async fn decide(&self, req: NetworkPolicyRequest) -> NetworkDecision;
}

#[async_trait]
impl<D: NetworkPolicyDecider + ?Sized> NetworkPolicyDecider for Arc<D> {
    async fn decide(&self, req: NetworkPolicyRequest) -> NetworkDecision {
        (**self).decide(req).await
    }
}

#[async_trait]
impl<F, Fut> NetworkPolicyDecider for F
where
    F: Fn(NetworkPolicyRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = NetworkDecision> + Send,
{
    async fn decide(&self, req: NetworkPolicyRequest) -> NetworkDecision {
        (self)(req).await
    }
}

fn unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}
