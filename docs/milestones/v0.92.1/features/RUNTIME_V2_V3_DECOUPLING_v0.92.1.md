# Runtime v2/v3 Decoupling

v0.92.1 separates Runtime v2 and Runtime v3 source, manifests, imports, public exports, ownership, tests, and compatibility contracts. The work must inventory every reverse reference, preserve supported behavior, provide rollback and migration proof, and prevent either runtime from silently owning the other's state or authority.

Runtime v4 is excluded. Any v4 requirement triggers explicit replanning rather than widening this track.
