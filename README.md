# Shard

Fragment caching proxy for web servers. Built in Rust with assembly optimizations.

Shard intercepts HTTP requests between your web server and backend, caches responses, and intelligently fragments HTML pages to enable granular cache invalidation. This allows hosting providers to significantly increase server density while maintaining performance. The core differentiator is semantic HTML fragmentation, which enables invalidating only the parts of a page that actually change, rather than entire responses.

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-black?style=for-the-badge&logo=github)](https://github.com/mqteooo317/shard-project)
[![GitHub Profile](https://img.shields.io/badge/GitHub-Profile-181717?style=for-the-badge&logo=github)](https://github.com/mqteooo317)
[![Discord](https://img.shields.io/badge/Discord-Join-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/user/1279870617197482055)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue?style=for-the-badge)](LICENSE)

## Contributors

- @mqteooo317

## License

Apache License 2.0