#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol, Vec};

/// On-chain record of a transcript that has been anchored by an authority.
/// Stores only the hash (not the transcript body) plus metadata about who
/// anchored it, when, and whether it has been revoked.
#[contracttype]
#[derive(Clone, Debug)]
pub struct AnchorRecord {
    pub transcript_hash: Symbol,
    pub issued_at: u64,
    pub authority: Address,
    pub revoked: bool,
    pub revoke_reason: Symbol,
}

#[contract]
pub struct TranscriptAnchor;

#[contractimpl]
impl TranscriptAnchor {
    /// Initialize the contract with the set of authorized university authorities
    /// (e.g., the registrar's address). Must be called exactly once before any
    /// other function. Panics if the contract is already initialized.
    pub fn init(env: Env, authorities: Vec<Address>) {
        if env.storage().instance().has(&"initialized") {
            panic!("Contract already initialized");
        }
        let mut auth_map: Map<Address, bool> = Map::new(&env);
        for auth in authorities.iter() {
            auth_map.set(auth, true);
        }
        env.storage().instance().set(&"authorities", &auth_map);
        env.storage().instance().set(&"initialized", &true);
    }

    /// Authority anchors a transcript hash for a given student. The caller must
    /// be an authorized authority (the university's registrar account). The
    /// transcript hash and the issuance timestamp are stored on-chain. Only the
    /// hash is stored — never the underlying transcript data — so this contract
    /// is safe for academic records under data-minimization principles.
    /// Panics if an active (non-revoked) anchor already exists for the student.
    pub fn anchor_transcript(
        env: Env,
        authority: Address,
        student: Address,
        transcript_hash: Symbol,
        issued_at: u64,
    ) {
        authority.require_auth();
        Self::assert_authority(&env, &authority);

        let mut anchors: Map<Address, AnchorRecord> = env
            .storage()
            .instance()
            .get(&"anchors")
            .unwrap_or_else(|| Map::new(&env));

        if let Some(existing) = anchors.get(student.clone()) {
            if !existing.revoked {
                panic!("Active transcript already anchored for this student");
            }
        }

        let record = AnchorRecord {
            transcript_hash,
            issued_at,
            authority: authority.clone(),
            revoked: false,
            revoke_reason: Symbol::new(&env, ""),
        };

        anchors.set(student, record);
        env.storage().instance().set(&"anchors", &anchors);
    }

    /// Verifier (employer, graduate school, etc.) calls this to confirm a
    /// transcript has been anchored and that the hash they computed from the
    /// off-chain PDF matches the on-chain record. Returns `true` only when an
    /// active (non-revoked) anchor exists for the student AND the supplied
    /// `transcript_hash` matches the anchored hash exactly. Does not require
    /// authorization — anyone can verify.
    pub fn verify_transcript(env: Env, student: Address, transcript_hash: Symbol) -> bool {
        let anchors: Map<Address, AnchorRecord> = env
            .storage()
            .instance()
            .get(&"anchors")
            .unwrap_or_else(|| Map::new(&env));

        match anchors.get(student) {
            Some(record) => !record.revoked && record.transcript_hash == transcript_hash,
            None => false,
        }
    }

    /// Authority revokes a previously anchored transcript (e.g., issued in
    /// error, contains incorrect grades, or is found to be fraudulent). The
    /// caller must be an authorized authority. After revocation the anchor
    /// remains on-chain for auditability, but `verify_transcript` will return
    /// `false` for it. A revoked anchor may later be overwritten by a new
    /// `anchor_transcript` call (e.g., to issue a corrected transcript).
    pub fn revoke_transcript(
        env: Env,
        authority: Address,
        student: Address,
        reason: Symbol,
    ) {
        authority.require_auth();
        Self::assert_authority(&env, &authority);

        let mut anchors: Map<Address, AnchorRecord> = env
            .storage()
            .instance()
            .get(&"anchors")
            .unwrap_or_else(|| Map::new(&env));

        let mut record = anchors
            .get(student.clone())
            .unwrap_or_else(|| panic!("No transcript anchored for this student"));

        record.revoked = true;
        record.revoke_reason = reason;
        anchors.set(student, record);
        env.storage().instance().set(&"anchors", &anchors);
    }

    /// Returns the issuance timestamp of the (current) anchor for a student.
    /// Returns `0` if no anchor exists. Useful for verifiers that want to
    /// display "this transcript was issued on …" alongside the verification
    /// result, or for time-bound checks ("only accept transcripts issued
    /// after date X").
    pub fn anchored_at(env: Env, student: Address) -> u64 {
        let anchors: Map<Address, AnchorRecord> = env
            .storage()
            .instance()
            .get(&"anchors")
            .unwrap_or_else(|| Map::new(&env));

        match anchors.get(student) {
            Some(record) => record.issued_at,
            None => 0,
        }
    }

    /// Returns the revocation reason for a student's anchored transcript, or an
    /// empty symbol if the transcript is active or no anchor exists. Lets
    /// verifiers surface "why was this revoked?" to the end user.
    pub fn revoke_reason(env: Env, student: Address) -> Symbol {
        let anchors: Map<Address, AnchorRecord> = env
            .storage()
            .instance()
            .get(&"anchors")
            .unwrap_or_else(|| Map::new(&env));

        match anchors.get(student) {
            Some(record) if record.revoked => record.revoke_reason,
            _ => Symbol::new(&env, ""),
        }
    }

    /// Internal helper: panic if the calling address is not in the
    /// `authorities` set configured at initialization.
    fn assert_authority(env: &Env, authority: &Address) {
        let auth_map: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&"authorities")
            .unwrap_or_else(|| Map::new(env));
        if !auth_map.get(authority.clone()).unwrap_or(false) {
            panic!("Caller is not an authorized authority");
        }
    }
}
