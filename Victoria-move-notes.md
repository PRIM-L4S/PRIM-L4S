# Moving to Victoria notes and todo

- We should set `-retentionPeriod=100y`
- We might want to take a look at: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#downsampling. Actually, it seems to be a entreprise only feature, so it's probably not activated by default.

Euuuh certaines features sont payantes ??

## Pulling in Prometheus style

- See: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#how-to-scrape-prometheus-exporters-such-as-node-exporter

## Pushing

- See: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#how-to-import-data-in-prometheus-exposition-format
- You will need to set `-search.cacheTimestampOffset`: https://docs.victoriametrics.com/victoriametrics/single-server-victoriametrics/#backfilling
