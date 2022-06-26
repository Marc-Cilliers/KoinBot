<div align="center">
  <h1>🪙 KoinBot 🪙</h1>
  <h3>Cryptocurrency market data at the speed of light ⚡️</h3>
</div>

## About

KoinBot is a Discord bot used to fetch and display cryptocurrency data with a focus on speed. It currently makes use of the [serenity](https://github.com/serenity-rs/serenity) rust wrapper for the Discord API and the [CoinGecko API](https://www.coingecko.com/en/api/documentation) for fetching cryptocoin market data.

## Adding KoinBot

Want to give KoinBot a try? [Click Here](https://discord.com/api/oauth2/authorize?client_id=817408196053696542&permissions=2147483648&scope=bot+applications.commands)

## Commands

Koinbot has 100 slash commands (currently the Discord limit). There are 2 basic categories;

```
/{coin-name} (eg. /bitcoin)                         | Exists for the 99 most popular coins
/niche {coin-name} (eg. /niche arb protocol)        | For every other niche coin
```

### Options

Options are extra, optional arguments that can be passed to each command.

```
currency                                            | Allows the user to select their preferred currency. 25 of the most popular currencies are available (currently the Discord limit of enum-type options)
```

## Roadmap

| Goal                 |   Status    |     |
| -------------------- | :---------: | :-: |
| Various currencies   |  Complete   | ✅  |
| Localization support | Not Started | 🟠  |
| UI Tweaks            | Not Started | 🟠  |
| NFT Support          | Not Started | 🟠  |
