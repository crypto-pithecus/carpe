use libra_types::transaction::authenticator::AuthenticationKey;
use libra_types::waypoint::Waypoint;
use ol::commands::init_cmd::InitCmd;
use ol::prelude::{app_config, Runnable};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

use libra_config::config::NodeConfig;
use libra_types::account_address::AccountAddress;
use libra_wallet::{Mnemonic, WalletLibrary};
use miner::commands::start_cmd::StartCmd;
use miner::commands::MinerCmd;
use ol::config::AppCfg;
use ol::entrypoint::EntryPoint;
use ol::mgmt::management::NodeMode;
use ol::node::client;
use ol::node::node::Node;
use ol_keys::wallet;
use onboard::commands::wizard_user_cmd::{check, wizard};
use std::thread;
use std::time::Duration;
use tauri::Error;

#[tauri::command]
pub fn hello(hello: String) -> String {
  return format!("Hello: {}", hello);
}

/// Keygen output
#[derive(Serialize, Deserialize)]
struct Output {
  mnemonic: String,
  account: AccountAddress,
  authkey: AuthenticationKey,
}

/// Keygen handler
#[tauri::command]
pub fn keygen() -> Result<String, String> {
  let wallet = WalletLibrary::new();
  let mnemonic_string = wallet.mnemonic();

  let (authkey, account, _) = wallet::get_account_from_mnem(mnemonic_string.clone());

  let output = Output {
    mnemonic: mnemonic_string,
    account,
    authkey,
  };
  return match serde_json::to_string(&output) {
    Ok(t) => Ok(t),
    Err(e) => Err(e.to_string()),
  };
}

/// Wizard User handler
#[tauri::command]
pub async fn wizard_user(
  home_path: Option<PathBuf>,
  check: bool,
  fix: bool,
  validator: bool,
  block_zero: Option<PathBuf>,
) -> bool {
  if let Some(path) = home_path {
    let (acc, add) = onboard::commands::wizard_user_cmd::wizard(path, false, &None);
    true
  } else {
    false
  }
}

/// Wizard init handler
#[tauri::command]
pub async fn init(authkey: AuthenticationKey, account: AccountAddress) -> bool {
  ol_types::config::AppCfg::init_app_configs(authkey, account, &None, &None, None, None, &None);

  true
}

/// Wizard User Check Handler
#[tauri::command]
pub fn wizard_user_check(home: String) -> bool {
  let home_path = if home.is_empty() {
    PathBuf::from(".")
  } else {
    PathBuf::from(home)
  };
  check(home_path)
}

#[tauri::command]
pub async fn start_swarm(swarm_path: Option<PathBuf>) -> bool {
  true
}

/// Start Mining handler
#[tauri::command]
pub async fn start_mining(
  home: Option<PathBuf>,
  swarm_path: Option<PathBuf>,
  swarm_persona: Option<String>,
  is_operator: bool,
) -> bool {
  let s = StartCmd {
    backlog_only: false,
    skip_backlog: false,
    upstream_url: false,
    url: None,
  };
  miner::entrypoint::EntryPoint {
    config: home,
    help: false,
    verbose: false,
    command: Some(miner::commands::MinerCmd::Start(s)),
    account: None,
    url: None,
    use_upstream_url: false,
    waypoint: None,
    save_path: None,
    no_send: false,
    swarm_path,
    swarm_persona,
    is_operator,
  };

  true
}

/// Start Node handler
#[tauri::command]
pub async fn start_node(home: PathBuf) -> Result<bool, String> {
  match NodeConfig::load(home) {
    Ok(config) => {
      libra_node::start(&config, None);
      Ok(true)
    }
    Err(e) => Err(format!("Config was not loaded from: {:?}", e)),
  }
}

/// Stop Mining handler
#[tauri::command]
pub async fn stop_mining() -> bool {
  true
}

/// Stop Mining handler
#[tauri::command]
pub async fn stop_node() -> bool {
  true
}
