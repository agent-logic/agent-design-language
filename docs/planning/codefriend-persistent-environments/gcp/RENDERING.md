# Reproduce the diagram

Download the two official icon ZIP URLs in ICON_PROVENANCE.json and verify their SHA256 values. Run:

```sh
python3 draw_architecture.py --core-icons /path/to/core-products-icons.zip --legacy-icons /path/to/google-cloud-legacy-icons.zip
rsvg-convert GCP_ARCHITECTURE.svg -o GCP_ARCHITECTURE.png
```

Python standard library and librsvg rsvg-convert are required. Icons are embedded unchanged. Region/zones/project/compute and database status are proposals described in PLAN.md. Visual QA checks three environments, two zones, regional NAT/subnet semantics and readable labels. Worker egress and API service connections are summarized by labels rather than separately arrowed.
