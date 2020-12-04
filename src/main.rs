mod common {
    tonic::include_proto!("common");
}

mod msp {
    tonic::include_proto!("msp");
}

mod protos {
    tonic::include_proto!("protos");
}

mod kvrwset {
    tonic::include_proto!("kvrwset");
}

mod queryresult {
    tonic::include_proto!("queryresult");
}

mod chaincode_tx;
mod proposer;
mod cli;

use shrust::{Shell, ShellIO};
use std::io::prelude::*;
use cli::Cli;

fn main() {
    let kms_addr = "localhost:50005";
    let controller_addr = "localhost:50004";
    let mut shell = Shell::new(Cli::new(kms_addr, controller_addr));
    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
