# Issue 723 design

Repair the C-SDLC v3 proof/shadow/install integration contract without weakening proof parity or cutover authority. Shadow execution must normalize exactly one typed document from the channel used by the invoked command, preserve nonzero exit truth, and retain mismatch detection. Install execution must require typed #505 cutover approval bound to repository, exact head, binary digest, and selector digest. Tests use deterministic lifecycle fixtures rather than mutable terminal issue state.
