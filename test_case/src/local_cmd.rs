use clap::ArgMatches;

use crate::*;
use nrgsc::error::Result;

pub fn local_cmd(m: &ArgMatches) -> Result<()> {
    // init command
    if let Some(init_arg) = m.subcommand_matches("init") {
        if let Some(mnemonic) = init_arg.value_of("MNEMONIC") {
            nrgsc::config::update_mnemonic(mnemonic)?;
        }
        // create settings
        let settings = nrgsc::config::get_settings();
        settings.show_config();

        return Ok(());
    }

    if let Some(n) = m.subcommand_matches("genesis") {
        match value_t!(n.value_of("n"), u32) {
            Ok(num) => genesis_init(num)?,

            Err(e) => {
                error!("{}", e);
                e.exit()
            }
        }

        return Ok(());
    }

    if let Some(n) = m.subcommand_matches("wallets") {
        match value_t!(n.value_of("n"), usize) {
            Ok(num) => wallet::gen_wallets(num)?,

            Err(e) => e.exit(),
        };
    }
    Ok(())
}

fn genesis_init(witness_counts: u32) -> Result<()> {
    // TODO: get total amount and msg from args
    let total = 500_000_000_000_000;
    let msg = "hello nrgsc";
    let wallets = genesis::gen_all_wallets(witness_counts)?;

    let (genesis_joint, balance) = genesis::gen_genesis_joint(&wallets, total, msg)?;
    let first_joint = genesis::gen_first_payment(&wallets.nrgsc_org, 20, &genesis_joint, balance)?;

    use nrgsc::joint::Joint;
    #[derive(Serialize)]
    struct GENESIS<'a> {
        wallets: Vec<&'a String>,
        nrgsc_org: &'a String,
        first_payment: Joint,
        genesis_joint: Joint,
    }
    let result = GENESIS {
        wallets: wallets
            .witnesses
            .iter()
            .map(|v| &v.mnemonic)
            .collect::<Vec<_>>(),
        nrgsc_org: &wallets.nrgsc_org.mnemonic,
        first_payment: first_joint,
        genesis_joint,
    };

    save_results(&result, "result.json")
}
