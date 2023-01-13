use rand::RngCore;
use sha2::Digest;
use std::{cmp::Ordering, fmt::Display, hash::Hash, sync::mpsc};
use tracing::{error, info, warn};
use Message::*;

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let (atx, brx) = mpsc::channel();
    let (btx, arx) = mpsc::channel();

    std::thread::scope(|s| {
        let a = s.spawn(|| alice(atx, arx).unwrap());
        let b = s.spawn(|| bob(btx, brx).unwrap());
        a.join().unwrap();
        b.join().unwrap();
    });

    Ok(())
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}

impl Hand {
    fn opposite(self) -> Self {
        [Hand::Rock, Hand::Paper, Hand::Scissor][(self as usize + 1) % 3]
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hand::Rock => write!(f, "rock"),
            Hand::Paper => write!(f, "paper"),
            Hand::Scissor => write!(f, "scissor"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if *self == other.opposite() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

type Salt = [u8; 16];
type Commit = [u8; 32];

#[derive(PartialEq, Eq, Debug)]
enum Message {
    Commit(Commit),
    Hand(Hand, Salt),
}

impl Message {
    fn commit(hand: Hand) -> (Commit, Salt) {
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

    fn verify(hand: Hand, salt: Salt, commit: Commit) -> bool {
        commit == Self::commit_with_salt(hand, salt)
    }
}

#[tracing::instrument(skip(tx, rx))]
fn alice(tx: mpsc::Sender<Message>, rx: mpsc::Receiver<Message>) -> Result<()> {
    // MATCH START

    let hand = Hand::Paper;

    let (commit, salt) = Message::commit(hand);
    tx.send(Message::Commit(commit))?;
    info!("sent commit ({hand})");

    let Commit(op_commit) = rx.recv()? else { panic!("expected commit") };
    info!("receive commit");

    let Hand(op_hand, op_salt) = rx.recv()? else { panic!("expected hand") };
    info!("got {op_hand}");

    tx.send(Hand(hand, salt))?;
    info!("throw {hand}");

    if !Message::verify(op_hand, op_salt, op_commit) {
        error!("cheater detected!");
    } else {
        match hand.cmp(&op_hand) {
            Ordering::Less => info!("i lose..."),
            Ordering::Equal => info!("a draw, i see"),
            Ordering::Greater => info!("i win!"),
        }
    }

    // channel is closed
    drop(tx);
    assert!(rx.recv().is_err());
    Ok(())
}

#[tracing::instrument(skip(tx, rx))]
fn bob(tx: mpsc::Sender<Message>, rx: mpsc::Receiver<Message>) -> Result<()> {
    // MATCH START

    let hand = Hand::Scissor;

    let (commit, salt) = Message::commit(hand);
    tx.send(Message::Commit(commit))?;
    info!("sent commit ({hand})");

    let Commit(op_commit) = rx.recv()? else { panic!("expected commit") };
    info!("receive commit");

    tx.send(Hand(hand, salt))?;
    info!("throw {hand}");

    let Hand(op_hand, op_salt) = rx.recv()? else { panic!("expected hand") };
    info!("got {op_hand}");

    if !Message::verify(op_hand, op_salt, op_commit) {
        error!("cheater detected!");
    } else {
        match hand.cmp(&op_hand) {
            Ordering::Less => info!("i lose..."),
            Ordering::Equal => info!("a draw, i see"),
            Ordering::Greater => info!("i win!"),
        }
    }

    // channel is closed
    drop(tx);
    assert!(rx.recv().is_err());
    Ok(())
}

#[tracing::instrument(skip(tx, rx))]
fn trudy(tx: mpsc::Sender<Message>, rx: mpsc::Receiver<Message>) -> Result<()> {
    // MATCH START

    let mut hand = Hand::Paper;

    let (commit, salt) = Message::commit(hand);
    tx.send(Message::Commit(commit))?;
    info!("sent commit ({hand})");

    let Commit(op_commit) = rx.recv()? else { panic!("expected commit") };
    info!("receive commit");

    let Hand(op_hand, op_salt) = rx.recv()? else { panic!("expected hand") };
    info!("got {op_hand}");

    hand = op_hand.opposite();
    warn!("change hand to {hand}");

    tx.send(Hand(hand, salt))?;
    info!("throw {hand}");

    if !Message::verify(op_hand, op_salt, op_commit) {
        error!("cheater detected!");
    }

    // channel is closed
    drop(tx);
    assert!(rx.recv().is_err());
    Ok(())
}
