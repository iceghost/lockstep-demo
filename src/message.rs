use rand::RngCore;
use sha2::Digest;

use crate::hand::Hand;

pub type Salt = [u8; 16];
pub type Commit = [u8; 32];

#[derive(PartialEq, Eq, Debug)]
pub enum Message {
    Commit(Commit),
    Hand(Hand, Salt),
}

impl Message {
    pub fn commit(hand: Hand) -> (Commit, Salt) {
        let mut salt = Salt::default();
        rand::thread_rng().fill_bytes(&mut salt);
        (Self::commit_with_salt(hand, salt), salt)
    }

    fn commit_with_salt(hand: Hand, salt: Salt) -> Commit {
        let mut hasher = sha2::Sha256::new();
        hasher.update(&[hand as u8]);
        hasher.update(&salt);
        hasher.finalize().into()
    }

    pub fn verify(hand: Hand, salt: Salt, commit: Commit) -> bool {
        commit == Self::commit_with_salt(hand, salt)
    }
}
