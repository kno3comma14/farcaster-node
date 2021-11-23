// LNP Node: node running lightning network protocol and generalized lightning
// channels.
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! Command-line interface to LNP node

#[macro_use]
extern crate log;

use clap::Clap;

use farcaster_node::cli::Opts;
use farcaster_node::rpc::Client;
use farcaster_node::{Config, LogStyle};
use microservices::shell::Exec;

fn main() {
    println!("swap-cli: command-line tool for working with Farcaster node");

    let mut opts = Opts::parse();
    trace!("Command-line arguments: {:?}", &opts);
    opts.process();
    trace!("Processed arguments: {:?}", &opts);

    let config: Config = opts.shared.clone().into();
    trace!("Tool configuration: {:?}", &config);
    debug!("MSG RPC socket {}", &config.msg_endpoint);
    debug!("CTL RPC socket {}", &config.ctl_endpoint);

    let mut client = Client::with(config).expect("Error initializing client");

    trace!("Executing command: {:?}", opts.command);
    opts.command
        .exec(&mut client)
        .unwrap_or_else(|err| eprintln!("{} {}", "error:".err(), err.err()));
}
