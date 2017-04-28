# pretty-cron

A Rust library to print Cron expressions as human text.

An in-progress port of https://github.com/azza-bazoo/prettycron

Currently missing support for step values e.g. "0/1"

```
cargo add pretty-cron
```

```
extern crate pretty_cron;

use pretty_cron::prettify_cron;

let res = prettify_cron("30 * * * * *");
// res == "Every minute starting on the 30th second"
```

## License

LGPLv3
