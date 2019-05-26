# `weather`

Uses the ESP8285 WiFi chip of the Maix Go to fetch weather data from
[wttr.in](https://wttr.in) and print it to the display using `k210-console`.

As it needs to connect to an access point first, this needs configuration of one
to connect to in `src/config.rs`:

```bash
cp src/config.rs.example src/config.rs
vim src/config.rs # ...
```

Set `<ap name>` and `<ap password>` accordingly. Do not check in `src/config.rs` !
(gitignore settings should prevent this)
