# Moving to Victoria notes and todo

- We should set `-retentionPeriod=100y`
- We might want to take a look at: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#downsampling. Actually, it seems to be a entreprise only feature, so it's probably not activated by default.

Euuuh certaines features sont payantes ??

## Pulling in Prometheus style

- See: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#how-to-scrape-prometheus-exporters-such-as-node-exporter

## Pushing

- See: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#how-to-import-data-in-prometheus-exposition-format
- You will need to set `-search.cacheTimestampOffset`: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#backfilling

## Gathering data

- Filter to keep only sockets with destination port `5201`.

```sh
ss -i dport = :5201
```

- We can't see those sockets on the router (it doesn't know about them I guess).

- On the client, we see two sockets: maybe one for data and the other for control. Or maybe both for data. Idk.
