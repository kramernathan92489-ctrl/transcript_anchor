# transcript_anchor

## Project Title
transcript_anchor

## Project Description
Universities and other academic issuers need a tamper-proof way to prove that a transcript is authentic. `transcript_anchor` is a Soroban smart contract that lets an authorized authority (e.g., a university registrar) anchor the hash of an issued transcript on-chain. Employers, graduate schools, and other verifiers can then recompute the hash of an off-chain transcript and call `verify_transcript` to confirm it has not been altered since issuance. The contract stores only hashes, never the transcript contents, so student data stays private and on the institution's own systems.

## Project Vision
Our long-term goal is to become a foundational trust layer for academic credentials in the Stellar ecosystem. By anchoring only cryptographic fingerprints of transcripts, we let universities keep full control of their data while giving verifiers anywhere in the world a single, low-cost, public way to confirm authenticity. Eventually this primitive can extend to diplomas, certifications, micro-credentials, and other education artifacts — turning paper-based trust into math-based trust, at the cost of a few stroops per record.

## Key Features
- **Hash-only anchoring** — only the transcript hash, timestamp, and issuing authority are stored on-chain; the actual grades and personal data never leave the university's system.
- **Authorized issuers** — anchoring and revocation can only be performed by addresses registered in the contract's authority set at initialization.
- **Public verification** — anyone (no wallet signature required) can call `verify_transcript` to check whether a transcript hash matches the on-chain record and is still active.
- **Auditable revocation** — authorities can revoke an anchor with a human-readable reason (e.g., "issued in error", "fraud detected"). The revocation is permanent and visible on-chain.
- **Timestamp lookup** — `anchored_at` returns the issuance time of a transcript, enabling verifiers to display "issued on …" or to enforce date-based policies.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** education dApp — see `contracts/transcript_anchor/src/lib.rs` for the full transcript_anchor business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CAIRLGMMF5XMMVHH37WGINVUJRWJN6KDOMUAHZFKJJZPZQBF6QRE6NNF`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/b67a540b53acb32a81eac4cf0cd4daeeba59ea9c7d9ad8b1ff35371f31239f9b`

## Future Scope
- **Multi-issuer co-signing** — require N-of-M authority signatures (e.g., registrar + dean) before an anchor is accepted, raising the bar against single-account compromise.
- **Batch anchoring** — accept an array of `(student, hash, issued_at)` tuples in a single transaction to lower per-record costs when graduating a whole cohort.
- **Off-chain hash service + IPFS pointer** — let authorities optionally store a content hash of a signed transcript PDF and a pointer to its IPFS location, so verifiers can pull the artifact directly and recompute the hash themselves.
- **Cross-chain bridge** — emit events that a Stellar ↔ Ethereum bridge can consume, so employers using non-Stellar tooling can verify Stellar-anchored transcripts from their existing workflows.
- **Privacy-preserving selective disclosure** — integrate zero-knowledge proofs so a student can prove "I graduated with GPA ≥ 3.5 in 2024" without revealing the full transcript or even the anchor hash.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `transcript_anchor` (education)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
