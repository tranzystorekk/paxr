use std::process::Command;

const SUDO: &str = "/usr/bin/sudo";

const XBPS_INSTALL: &str = "xbps-install";
const XBPS_QUERY: &str = "xbps-query";
const XBPS_REMOVE: &str = "xbps-remove";

const XBPS_INSTALL_UPGRADE_FLAG: &str = "-Su";
const XBPS_QUERY_LIST_ORPHANS_FLAG: &str = "-O";
const XBPS_REMOVE_CLEAN_ORPHANS_FLAG: &str = "-o";
const XBPS_REMOVE_RECURSIVE_FLAG: &str = "-R";

pub fn install(pkgs: &[String]) -> Command {
    let mut cmd = Command::new(SUDO);

    cmd.arg(XBPS_INSTALL);
    cmd.args(pkgs);

    cmd
}

pub fn uninstall(pkgs: &[String]) -> Command {
    let mut cmd = Command::new(SUDO);

    cmd.args([XBPS_REMOVE, XBPS_REMOVE_RECURSIVE_FLAG]);
    cmd.args(pkgs);

    cmd
}

pub fn upgrade() -> Command {
    let mut cmd = Command::new(SUDO);

    cmd.args([XBPS_INSTALL, XBPS_INSTALL_UPGRADE_FLAG]);

    cmd
}

pub fn info(pkg: &str) -> Command {
    let mut cmd = Command::new(XBPS_QUERY);

    cmd.arg(pkg);

    cmd
}

pub fn list_orphans() -> Command {
    let mut cmd = Command::new(XBPS_QUERY);

    cmd.arg(XBPS_QUERY_LIST_ORPHANS_FLAG);

    cmd
}

pub fn clean_orphans() -> Command {
    let mut cmd = Command::new(SUDO);

    cmd.args([XBPS_REMOVE, XBPS_REMOVE_CLEAN_ORPHANS_FLAG]);

    cmd
}
