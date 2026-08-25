# Internal Review Packet Manifest

- Exact product target: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Manifest schema: `adl.internal_review.packet_manifest.v1`
- Digested objects: 42
- Packet SHA-256: `fd6f662984645f13d28842619a9a6ef533de6e9ab138eb203172e1b7346d18ec`

`packet-manifest.json` lists every digested object. The manifest files exclude
themselves to avoid circular self-digests. Validation recomputes every listed
object digest before accepting the packet.
