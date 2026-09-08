# System clock and cache proof

- Product revision: `64c40f0fad86aa5f73ceb5331ca88bc2ebadb69e`
- Served URL: `http://localhost:8000/demos/html-observatory/`
- Asset generation: `v0921-wuji-56`
- Tracked `index.html` SHA-256: `738a1ce3ef45616021da7dcd9662d311bb03b197752f82eb5f6d86130d690cad`
- Served `index.html` SHA-256: `738a1ce3ef45616021da7dcd9662d311bb03b197752f82eb5f6d86130d690cad`
- Tracked `app.js` SHA-256: `1cc89b0bd43ca98bfc2441e2fb045dfd4a4a1cb211cef76c38d42636dc586523`
- Served `app.js?v=v0921-wuji-56` SHA-256: `1cc89b0bd43ca98bfc2441e2fb045dfd4a4a1cb211cef76c38d42636dc586523`

`node --test demos/html-observatory/tests/system_clock_cache.test.mjs` executes the production clock functions with a controlled clock. It proves that both displayed time fields start equal, advance by two seconds after the registered one-second callback, remain synchronized, and use only one timer. It also proves that the CSS and JavaScript generation keys match `v0921-wuji-56`.

The operator's six-key hardware interaction matrix remains applicable to this revision: the only product-code delta after the tested `b893294df249b2db09eb90076627eed46c442a42` revision adds seconds to `formatTimestampLabel`; the Inspector tab handler is unchanged.
