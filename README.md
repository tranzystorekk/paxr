# Paxr

## About

Paxr is a Rust wrapper for the [XBPS](https://github.com/void-linux/xbps) package mangager,
similar to [Paker](https://github.com/tranzystorek-io/pakr).

It compresses the most common package maintenance operations into intuitive commands, e.g.:

- Install packages: `paxr install my-pkg`
- Uninstall packages (recursive): `paxr uninstall my-pkg`
- Upgrade packages: `paxr upgrade`
