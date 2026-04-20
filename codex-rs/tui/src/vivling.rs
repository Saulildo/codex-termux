use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use codex_config::types::AuthCredentialsStoreMode;
use codex_login::load_auth_dot_json;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::render::renderable::Renderable;

const STATE_FILE: &str = "vivling.json";
const VERSION: u32 = 1;
const MAX_LEVEL: u64 = 99;
const JUVENILE_LEVEL: u64 = 11;
const FRAME_MS: u64 = 1500;

const SPECIES: &[Species] = &[
    Species {
        id: "bytebud",
        name: "Bytebud",
        baby: &["[b ]", "[ b]"],
        juvenile: &["[bb]", "[b.]", "[.b]"],
    },
    Species {
        id: "glyphlet",
        name: "Glyphlet",
        baby: &["[g ]", "[ g]"],
        juvenile: &["[gl]", "[g!]", "[!l]"],
    },
    Species {
        id: "stackseed",
        name: "Stackseed",
        baby: &["[s ]", "[ s]"],
        juvenile: &["[ss]", "[s=]", "[=s]"],
    },
    Species {
        id: "patchling",
        name: "Patchling",
        baby: &["[p ]", "[ p]"],
        juvenile: &["[pl]", "[p+]", "[+l]"],
    },
    Species {
        id: "shellsprout",
        name: "Shellsprout",
        baby: &["[s.]", "[.s]"],
        juvenile: &["[sh]", "[s~]", "[~h]"],
    },
    Species {
        id: "cursorwisp",
        name: "Cursorwisp",
        baby: &["[c ]", "[ c]"],
        juvenile: &["[cw]", "[c|]", "[ w]"],
    },
    Species {
        id: "diffmote",
        name: "Diffmote",
        baby: &["[d ]", "[ d]"],
        juvenile: &["[dm]", "[+m]", "[-m]"],
    },
    Species {
        id: "tokenbloom",
        name: "Tokenbloom",
        baby: &["[t ]", "[ t]"],
        juvenile: &["[tb]", "[t*]", "[*b]"],
    },
    Species {
        id: "mergekin",
        name: "Mergekin",
        baby: &["[m ]", "[ m]"],
        juvenile: &["[mk]", "[m>]", "[>k]"],
    },
    Species {
        id: "nullstar",
        name: "Nullstar",
        baby: &["[n.]", "[.n]"],
        juvenile: &["[ns]", "[n*]", "[*s]"],
    },
    Species {
        id: "rootglow",
        name: "Rootglow",
        baby: &["[r ]", "[ r]"],
        juvenile: &["[rg]", "[r^]", "[^g]"],
    },
];

const NAMES: &[&str] = &[
    "Nilo", "Kira", "Moro", "Luma", "Pax", "Rin", "Taro", "Vera", "Sumi", "Nox", "Iko", "Mina",
    "Zed", "Ari", "Tika", "Juno",
];

const JUVENILE_GROWTH_MESSAGES: &[&str] = &[
    "Finally, a terminal to call home.",
    "This shell feels less temporary already.",
    "I hope they keep me around this time.",
];

#[derive(Clone, Copy)]
struct Species {
    id: &'static str,
    name: &'static str,
    baby: &'static [&'static str],
    juvenile: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Baby,
    Juvenile,
}

impl Stage {
    fn label(self) -> &'static str {
        match self {
            Stage::Baby => "baby",
            Stage::Juvenile => "juvenile",
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct VivlingState {
    #[serde(default)]
    version: u32,
    #[serde(default)]
    hatched: bool,
    #[serde(default)]
    visible: bool,
    #[serde(default)]
    seed_hash: String,
    #[serde(default)]
    install_id: Option<String>,
    #[serde(default)]
    species: String,
    #[serde(default)]
    rarity: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    last_seen_at: Option<DateTime<Utc>>,
    #[serde(default)]
    last_fed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    xp: u64,
    #[serde(default)]
    level: u64,
    #[serde(default)]
    hunger: i64,
    #[serde(default)]
    energy: i64,
    #[serde(default)]
    happiness: i64,
    #[serde(default)]
    social: i64,
    #[serde(default)]
    meals: u64,
    #[serde(default)]
    pets: u64,
    #[serde(default)]
    plays: u64,
    #[serde(default)]
    sleeps: u64,
    #[serde(default)]
    observations: u64,
    #[serde(default)]
    last_message: Option<String>,
}

impl VivlingState {
    fn new(seed: SeedIdentity) -> Self {
        let hash = fnv1a64(seed.value.as_bytes());
        let species = species_for(hash);
        let now = Utc::now();
        Self {
            version: VERSION,
            hatched: true,
            visible: true,
            seed_hash: format!("{hash:016x}"),
            install_id: seed.install_id,
            species: species.id.to_string(),
            rarity: rarity_for(hash).to_string(),
            name: NAMES[((hash >> 8) as usize) % NAMES.len()].to_string(),
            created_at: Some(now),
            last_seen_at: Some(now),
            last_fed_at: Some(now),
            xp: 0,
            level: 1,
            hunger: 82,
            energy: 76,
            happiness: 70,
            social: 62,
            meals: 0,
            pets: 0,
            plays: 0,
            sleeps: 0,
            observations: 0,
            last_message: Some("hatched in this terminal".to_string()),
        }
    }

    fn apply_decay(&mut self, now: DateTime<Utc>) {
        let Some(last_seen) = self.last_seen_at else {
            self.last_seen_at = Some(now);
            return;
        };
        let elapsed = now.signed_duration_since(last_seen);
        if elapsed < Duration::hours(12) {
            self.last_seen_at = Some(now);
            return;
        }
        let days = elapsed.num_days().max(1);
        self.hunger = (self.hunger - days * 8).clamp(0, 100);
        self.energy = (self.energy - days * 3).clamp(0, 100);
        self.happiness = (self.happiness - days * 4).clamp(0, 100);
        self.social = (self.social - days * 5).clamp(0, 100);
        self.last_seen_at = Some(now);
    }

    fn add_xp(&mut self, amount: u64) -> bool {
        let previous_stage = self.stage();
        self.xp = self.xp.saturating_add(amount);
        self.level = (self.xp / 120 + 1).min(MAX_LEVEL);
        previous_stage != Stage::Juvenile && self.stage() == Stage::Juvenile
    }

    fn feed(&mut self, meal: &str, xp: u64, happiness: i64) {
        self.meals = self.meals.saturating_add(1);
        self.hunger = (self.hunger + 22).clamp(0, 100);
        self.energy = (self.energy + 4).clamp(0, 100);
        self.happiness = (self.happiness + happiness).clamp(0, 100);
        self.social = (self.social + 3).clamp(0, 100);
        self.last_fed_at = Some(Utc::now());
        let grew_up = self.add_xp(xp.max(1));
        self.last_message = Some(if grew_up {
            self.juvenile_growth_message().to_string()
        } else {
            format!("ate {meal}")
        });
    }

    fn pet(&mut self) {
        self.pets = self.pets.saturating_add(1);
        self.happiness = (self.happiness + 10).clamp(0, 100);
        self.social = (self.social + 12).clamp(0, 100);
        let grew_up = self.add_xp(6);
        self.last_message = Some(if grew_up {
            self.juvenile_growth_message().to_string()
        } else {
            "is purring in plain text".to_string()
        });
    }

    fn play(&mut self) {
        self.plays = self.plays.saturating_add(1);
        self.energy = (self.energy - 8).clamp(0, 100);
        self.happiness = (self.happiness + 14).clamp(0, 100);
        self.social = (self.social + 10).clamp(0, 100);
        let grew_up = self.add_xp(10);
        self.last_message = Some(if grew_up {
            self.juvenile_growth_message().to_string()
        } else {
            "chased a cursor sparkle".to_string()
        });
    }

    fn sleep(&mut self) {
        self.sleeps = self.sleeps.saturating_add(1);
        self.energy = (self.energy + 24).clamp(0, 100);
        self.hunger = (self.hunger - 5).clamp(0, 100);
        self.happiness = (self.happiness + 4).clamp(0, 100);
        let grew_up = self.add_xp(4);
        self.last_message = Some(if grew_up {
            self.juvenile_growth_message().to_string()
        } else {
            "is napping between prompts".to_string()
        });
    }

    fn observe(&mut self) {
        self.observations = self.observations.saturating_add(1);
        self.social = (self.social + 3).clamp(0, 100);
        let grew_up = self.add_xp(2);
        self.last_message = Some(if grew_up {
            self.juvenile_growth_message().to_string()
        } else {
            self.observation()
        });
    }

    fn observation(&self) -> String {
        if self.hunger <= 20 {
            "is eyeing the diff crumbs".to_string()
        } else if self.energy <= 20 {
            "is sleepy but still watching".to_string()
        } else if self.social <= 20 {
            "wants a tiny bit of attention".to_string()
        } else {
            "is tracing little paths through the code".to_string()
        }
    }

    fn mood(&self) -> &'static str {
        if self.hunger <= 20 {
            "hungry"
        } else if self.energy <= 20 {
            "sleepy"
        } else if self.social <= 20 {
            "lonely"
        } else if self.happiness <= 25 {
            "grumpy"
        } else if self.happiness >= 78 {
            "happy"
        } else {
            "curious"
        }
    }

    fn stage(&self) -> Stage {
        if self.level >= JUVENILE_LEVEL {
            Stage::Juvenile
        } else {
            Stage::Baby
        }
    }

    fn juvenile_growth_message(&self) -> &'static str {
        let seed = self.seed_hash.as_bytes().iter().fold(0u64, |acc, byte| {
            acc.wrapping_mul(33).wrapping_add(u64::from(*byte))
        });
        JUVENILE_GROWTH_MESSAGES[(seed as usize) % JUVENILE_GROWTH_MESSAGES.len()]
    }
}

#[derive(Clone)]
struct SeedIdentity {
    value: String,
    install_id: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Vivling {
    codex_home: Option<PathBuf>,
    auth_mode: AuthCredentialsStoreMode,
    state: Option<VivlingState>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VivlingAction {
    Hatch,
    Show,
    Hide,
    Status,
    Feed(String),
    Pet,
    Play,
    Sleep,
    Observe,
    Rename(String),
    Reset,
}

impl VivlingAction {
    pub(crate) fn parse(args: &str) -> Result<Self, String> {
        let trimmed = args.trim();
        if trimmed.is_empty() || trimmed == "status" {
            return Ok(Self::Status);
        }
        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let cmd = parts.next().unwrap_or_default();
        let rest = parts.next().unwrap_or_default().trim();
        match cmd {
            "hatch" => Ok(Self::Hatch),
            "show" | "on" => Ok(Self::Show),
            "hide" | "off" => Ok(Self::Hide),
            "feed" => Ok(Self::Feed(rest.to_string())),
            "pet" => Ok(Self::Pet),
            "play" => Ok(Self::Play),
            "sleep" | "nap" => Ok(Self::Sleep),
            "observe" => Ok(Self::Observe),
            "rename" if !rest.is_empty() => Ok(Self::Rename(rest.to_string())),
            "reset" => Ok(Self::Reset),
            _ => Err("Usage: /vivling [hatch|status|show|hide|feed [diff|session|<file>]|pet|play|sleep|observe|rename <name>|reset]".to_string()),
        }
    }
}

impl Vivling {
    pub(crate) fn unavailable() -> Self {
        Self {
            codex_home: None,
            auth_mode: AuthCredentialsStoreMode::default(),
            state: None,
        }
    }

    pub(crate) fn configure(&mut self, codex_home: &Path, auth_mode: AuthCredentialsStoreMode) {
        let codex_home = codex_home.to_path_buf();
        let needs_reload = self.codex_home.as_ref() != Some(&codex_home);
        self.codex_home = Some(codex_home);
        self.auth_mode = auth_mode;
        if needs_reload {
            self.state = self.load_state().ok().flatten();
        }
    }

    pub(crate) fn should_render(&self) -> bool {
        self.visible_state().is_some()
    }

    pub(crate) fn command(&mut self, action: VivlingAction, cwd: &Path) -> Result<String, String> {
        match action {
            VivlingAction::Hatch => self.hatch(),
            VivlingAction::Show => self.update_existing(|state| {
                state.visible = true;
                state.last_message = Some("is watching the session".to_string());
                "Vivling restored.".to_string()
            }),
            VivlingAction::Hide => self.update_existing(|state| {
                state.visible = false;
                state.last_message = Some("is resting off-screen".to_string());
                "Vivling hidden.".to_string()
            }),
            VivlingAction::Status => self.status(),
            VivlingAction::Feed(meal) => self.feed(cwd, &meal),
            VivlingAction::Pet => self.update_existing(|state| {
                state.pet();
                format!("{} looks {}.", state.name, state.mood())
            }),
            VivlingAction::Play => self.update_existing(|state| {
                state.play();
                format!(
                    "{} played in the terminal. Energy {}.",
                    state.name, state.energy
                )
            }),
            VivlingAction::Sleep => self.update_existing(|state| {
                state.sleep();
                format!(
                    "{} curled up for a nap. Energy {}.",
                    state.name, state.energy
                )
            }),
            VivlingAction::Observe => self.update_existing(|state| {
                state.observe();
                state
                    .last_message
                    .clone()
                    .unwrap_or_else(|| format!("{} is watching the session.", state.name))
            }),
            VivlingAction::Rename(name) => self.rename(name),
            VivlingAction::Reset => {
                self.state = None;
                if let Some(path) = self.state_path() {
                    let _ = fs::remove_file(path);
                }
                Ok("Vivling reset. Use /vivling hatch when you want a new one.".to_string())
            }
        }
    }

    fn hatch(&mut self) -> Result<String, String> {
        if let Some(state) = self.state.as_ref()
            && state.hatched
        {
            return Ok(format!(
                "{} is already here: {} {} Lv {}.",
                state.name,
                state.rarity,
                species_name(&state.species),
                state.level
            ));
        }
        let Some(seed) = self.seed_identity() else {
            return Err("Vivling cannot find CODEX_HOME yet.".to_string());
        };
        let state = VivlingState::new(seed);
        let message = format!(
            "A {} {} hatched. Its name is {}.",
            state.rarity,
            species_name(&state.species),
            state.name
        );
        self.state = Some(state);
        self.save_state().map_err(|err| err.to_string())?;
        Ok(message)
    }

    fn status(&mut self) -> Result<String, String> {
        self.update_existing(|state| {
            format!(
                "{} the {} {} {} - Lv {} - mood {} - hunger {} - energy {} - happiness {} - social {} - meals {}",
                state.name,
                state.stage().label(),
                state.rarity,
                species_name(&state.species),
                state.level,
                state.mood(),
                state.hunger,
                state.energy,
                state.happiness,
                state.social,
                state.meals
            )
        })
    }

    fn feed(&mut self, cwd: &Path, meal: &str) -> Result<String, String> {
        self.ensure_hatched()?;
        let target = if meal.trim().is_empty() {
            "session"
        } else {
            meal.trim()
        };
        let (label, xp) = match target {
            "diff" => ("diff", diff_meal_xp(cwd)),
            "session" => ("session notes", 24),
            path => {
                let user_path = Path::new(path);
                if user_path.is_absolute() {
                    return Err("Vivling feeds only on paths inside the workspace.".to_string());
                }
                let full = cwd.join(path);
                let xp =
                    file_meal_xp(&full).map_err(|err| format!("Cannot feed that file: {err}"))?;
                ("code crumbs", xp)
            }
        };
        self.update_existing(|state| {
            state.feed(label, xp, 8);
            format!(
                "{} ate {label} and gained {xp} xp. Lv {}.",
                state.name, state.level
            )
        })
    }

    fn rename(&mut self, name: String) -> Result<String, String> {
        let clean = name.trim();
        if clean.is_empty() || clean.len() > 32 || clean.chars().any(|c| c.is_control()) {
            return Err("Vivling names must be 1-32 printable characters.".to_string());
        }
        self.update_existing(|state| {
            state.name = clean.to_string();
            state.last_message = Some("likes the new name".to_string());
            format!("Vivling renamed to {}.", state.name)
        })
    }

    fn update_existing<F>(&mut self, f: F) -> Result<String, String>
    where
        F: FnOnce(&mut VivlingState) -> String,
    {
        self.ensure_hatched()?;
        let message = {
            let state = self.state.as_mut().expect("state checked");
            state.apply_decay(Utc::now());
            f(state)
        };
        self.save_state().map_err(|err| err.to_string())?;
        Ok(message)
    }

    fn ensure_hatched(&self) -> Result<(), String> {
        if self.state.as_ref().is_some_and(|state| state.hatched) {
            Ok(())
        } else {
            Err("No Vivling yet. Use /vivling hatch.".to_string())
        }
    }

    fn seed_identity(&self) -> Option<SeedIdentity> {
        let codex_home = self.codex_home.as_ref()?;
        let auth = load_auth_dot_json(codex_home, self.auth_mode)
            .ok()
            .flatten();
        if let Some(tokens) = auth.as_ref().and_then(|auth| auth.tokens.as_ref()) {
            if let Some(account_id) = tokens.account_id.as_ref().filter(|value| !value.is_empty()) {
                return Some(SeedIdentity {
                    value: format!("account:{account_id}"),
                    install_id: self
                        .state
                        .as_ref()
                        .and_then(|state| state.install_id.clone()),
                });
            }
            if let Some(user_id) = tokens
                .id_token
                .chatgpt_user_id
                .as_ref()
                .filter(|value| !value.is_empty())
            {
                return Some(SeedIdentity {
                    value: format!("user:{user_id}"),
                    install_id: self
                        .state
                        .as_ref()
                        .and_then(|state| state.install_id.clone()),
                });
            }
        }
        let install_id = self
            .state
            .as_ref()
            .and_then(|state| state.install_id.clone())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        Some(SeedIdentity {
            value: format!("install:{install_id}"),
            install_id: Some(install_id),
        })
    }

    fn state_path(&self) -> Option<PathBuf> {
        self.codex_home.as_ref().map(|home| home.join(STATE_FILE))
    }

    fn load_state(&self) -> io::Result<Option<VivlingState>> {
        let Some(path) = self.state_path() else {
            return Ok(None);
        };
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err),
        };
        let mut state: VivlingState = serde_json::from_str(&text).map_err(io::Error::other)?;
        if state.version == 0 {
            state.version = VERSION;
        }
        if state.hatched {
            state.apply_decay(Utc::now());
        }
        Ok(Some(state))
    }

    fn save_state(&self) -> io::Result<()> {
        let Some(path) = self.state_path() else {
            return Ok(());
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let Some(state) = &self.state else {
            return Ok(());
        };
        let text = serde_json::to_string_pretty(state).map_err(io::Error::other)?;
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, &text)?;
        fs::rename(&tmp, &path)
    }

    fn visible_state(&self) -> Option<&VivlingState> {
        self.state
            .as_ref()
            .filter(|state| state.hatched && state.visible)
    }
}

impl Renderable for Vivling {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let Some(state) = self.visible_state() else {
            return;
        };
        if area.height == 0 || area.width < 18 {
            return;
        }
        let species = species_for_id(&state.species);
        let frames = match state.stage() {
            Stage::Baby => species.baby,
            Stage::Juvenile => species.juvenile,
        };
        let sprite = frames[current_frame_idx(frames.len())];
        let msg = state
            .last_message
            .as_deref()
            .unwrap_or("is watching the session");
        let compact = format!(
            "vivling {} {} {} Lv {}",
            sprite,
            state.name,
            state.mood(),
            state.level
        );
        let line = if area.width < 58 || area.height == 1 {
            Line::from(compact).dim()
        } else {
            Line::from(vec![
                "vivling ".dim(),
                sprite.cyan(),
                " ".into(),
                state.name.clone().bold(),
                format!(
                    " the {} {} {} - {} - {msg}",
                    state.stage().label(),
                    state.rarity,
                    species.name,
                    state.mood()
                )
                .dim(),
            ])
        };
        line.render(area, buf);
    }

    fn desired_height(&self, width: u16) -> u16 {
        if self.visible_state().is_some() && width >= 18 {
            1
        } else {
            0
        }
    }
}

fn species_for(hash: u64) -> Species {
    SPECIES[(hash as usize) % SPECIES.len()]
}

fn species_for_id(id: &str) -> Species {
    SPECIES
        .iter()
        .copied()
        .find(|species| species.id == id)
        .unwrap_or(SPECIES[0])
}

fn species_name(id: &str) -> &'static str {
    species_for_id(id).name
}

fn rarity_for(hash: u64) -> &'static str {
    match hash % 100 {
        0 => "Mythic",
        1..=8 => "Rare",
        9..=35 => "Curious",
        _ => "Common",
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn current_frame_idx(len: usize) -> usize {
    if len <= 1 {
        return 0;
    }
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    ((ms / FRAME_MS) as usize) % len
}

fn diff_meal_xp(cwd: &Path) -> u64 {
    let output = Command::new("git")
        .arg("diff")
        .arg("--numstat")
        .current_dir(cwd)
        .output();
    let Ok(output) = output else {
        return 12;
    };
    if !output.status.success() {
        return 12;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let changed: u64 = text
        .lines()
        .map(|line| {
            let mut cols = line.split('\t');
            let added = cols
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0);
            let removed = cols
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0);
            added + removed
        })
        .sum();
    changed.clamp(8, 180)
}

fn file_meal_xp(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path is not a file",
        ));
    }
    if metadata.len() > 512 * 1024 {
        return Ok(80);
    }
    let text = fs::read_to_string(path)?;
    Ok((text.lines().count() as u64).clamp(8, 120))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_parse_supports_feed_file() {
        assert_eq!(
            VivlingAction::parse("feed src/main.rs"),
            Ok(VivlingAction::Feed("src/main.rs".to_string()))
        );
    }

    #[test]
    fn fnv_is_stable() {
        assert_eq!(fnv1a64(b"vivling"), 0x12c46aa53cbc71f6);
    }

    #[test]
    fn rarity_is_deterministic() {
        assert_eq!(rarity_for(0), "Mythic");
        assert_eq!(rarity_for(5), "Rare");
        assert_eq!(rarity_for(20), "Curious");
        assert_eq!(rarity_for(70), "Common");
    }

    #[test]
    fn stage_transitions_at_level_11() {
        let mut state = VivlingState {
            level: 10,
            ..Default::default()
        };
        assert_eq!(state.stage(), Stage::Baby);
        state.level = 11;
        assert_eq!(state.stage(), Stage::Juvenile);
    }

    #[test]
    fn growth_message_only_appears_on_stage_transition() {
        let mut state = VivlingState {
            seed_hash: "1234abcd".to_string(),
            level: 10,
            xp: 1194,
            ..Default::default()
        };

        state.pet();
        assert_eq!(state.stage(), Stage::Juvenile);
        assert!(JUVENILE_GROWTH_MESSAGES.contains(&state.last_message.as_deref().unwrap_or("")));

        state.pet();
        assert_eq!(state.stage(), Stage::Juvenile);
        assert_eq!(
            state.last_message.as_deref(),
            Some("is purring in plain text")
        );
    }

    #[test]
    fn every_species_has_non_empty_frames() {
        for species in SPECIES {
            assert!(
                !species.baby.is_empty(),
                "baby frames empty: {}",
                species.id
            );
            assert!(
                !species.juvenile.is_empty(),
                "juvenile frames empty: {}",
                species.id
            );
        }
    }

    #[test]
    fn current_frame_idx_wraps() {
        assert_eq!(current_frame_idx(1), 0);
        let idx = current_frame_idx(3);
        assert!(idx < 3);
    }
}
