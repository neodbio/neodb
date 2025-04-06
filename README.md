# neodb
NeoDB is a research project, focusing on the development of a data engine written in Rust,
focused on the intersection of columnar time-series storage, streaming data ingestion, and intelligent query execution.

## What Is This?

**neodb** is an experimental database engine written in Rust, exploring:
- Efficient, columnar storage structures
- A custom query language (`nql`) for time-aware analytics
- Event-aligned, streaming-friendly processing models

## Goals (In Progress)
- Arrow-native, memory-mapped query execution
- Time-aligned filtering for real-time and historical data

## Status
This project is in **early-stage research and prototyping**.
It is not stable, nor production-ready.

We're actively validating architectural models, benchmarking performance, and experimenting with query abstractions.

## License
**neodb** is licensed under the **Business Source License 1.1 (BSL)**.

- Free for research, development, and internal non-production use
- Not permitted for commercial or hosted use (e.g., DBaaS/SaaS) without a commercial license

On **April 6, 2028**, the license will automatically convert to **Mozilla Public License 2.0 (MPL 2.0)**.

See the [`LICENSE`](./LICENSE) file for full terms.

## Contact
For licensing, research collaboration, or commercial use inquiries:
**Ervin Bosenbacher** – [ervin.bosenbacher@neodb.io]

## Coming Soon
- The initial `nql` grammar and parser
- High-throughput ingestion benchmarks
- Early versions of the neodb query engine
