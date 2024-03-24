use std::process::exit;

use clap::{Parser, Subcommand};

use self::network::LoginWithAccount;

pub mod network;

trait Display<T> {
    fn display(v: T);
}

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub sub_action: Option<SubAction>,
}

#[derive(Subcommand)]
pub enum SubAction {
    /// 校园网相关操作
    Network {
        #[command(subcommand)]
        network_action: network::Command,
    },
}

impl Cli {
    pub fn process() {
        let cli = Self::parse();

        if let Some(action) = cli.sub_action {
            match action {
                SubAction::Network { network_action } => {
                    // if !ctbox::network::util::is_cnu() {
                    //     eprintln!("无法访问校园网入口，请检查您及校园网的网络状态。");
                    //     exit(0);
                    // };
                    match network_action {
                        network::Command::Login {
                            login_with_account,
                            login_with_label: _,
                        } => {
                            if let LoginWithAccount {
                                account: Some(a),
                                password: Some(p),
                                save: None,
                                default: false,
                            } = login_with_account
                            {
                                network::login(&a, &p, true);
                            } else {
                                println!("功能未完成! 敬请期待.");
                            }
                        }
                        network::Command::Logout {} => network::logout(),
                        network::Command::Query { account } => {
                            network::query_user(account.as_deref());
                            println!();
                            network::query_device(account.as_deref());
                        }
                        network::Command::Encrypt { decrypt, source } => {
                            network::encrypt(decrypt, &source);
                        }
                    }
                }
            }
        }
    }
}
