mod hand;
mod message;

use hand::Hand;
use message::{Commit, Message, Salt};
use std::{cmp::Ordering, sync::mpsc};
use tracing::{error, info, warn};

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let (atx, brx) = mpsc::channel();
    let (btx, arx) = mpsc::channel();

    std::thread::scope(|s| -> Result<()> {
        // change the player to Trudy to see the effect
        //                                        👇
        let a = s.spawn(|| Player::new(PlayerName::Alice, atx, arx).play(Hand::Paper));
        let b = s.spawn(|| Player::new(PlayerName::Bob, btx, brx).play(Hand::Scissor));
        a.join().unwrap()?;
        b.join().unwrap()?;
        Ok(())
    })
    .unwrap();

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PlayerName {
    Alice,
    Bob,
    Trudy,
}

struct Player {
    name: PlayerName,
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl Player {
    fn new(name: PlayerName, tx: mpsc::Sender<Message>, rx: mpsc::Receiver<Message>) -> Self {
        Self { name, tx, rx }
    }

    fn play(&mut self, mut hand: Hand) -> Result<()> {
        // MATCH START

        // 1. come up with a hand and send the opponent the commit
        let (commit, salt) = self.commit(hand);
        self.send_commit(commit)?;

        // 2a1. wait for the opponent's commit
        let op_commit = self.wait_commit()?;

        if self.name == PlayerName::Trudy {
            // 2a2. wait for opponent's hand
            let (op_hand, _) = self.wait_hand()?;

            // 2a3. change into winning hand
            self.change_hand(&mut hand, op_hand);

            // 2a4. send hand
            self.throw_hand(hand, salt)?;

            return Ok(());
        }

        // 2a2. throw hand to opponent
        self.throw_hand(hand, salt)?;

        // 2b1. wait for opponent hand (parallel to 2a)
        let (op_hand, op_salt) = self.wait_hand()?;

        // 2b2. verify opponent integrity
        if !Message::verify(op_hand, op_salt, op_commit) {
            error!("cheater detected!");
            return Ok(());
        }

        // 3. check match result
        self.check_result(hand, op_hand);

        Ok(())
    }

    #[tracing::instrument]
    fn commit(&mut self, hand: Hand) -> (Commit, Salt) {
        let (commit, salt) = Message::commit(hand);
        info!("done");
        (commit, salt)
    }

    #[tracing::instrument(skip(commit))]
    fn send_commit(&mut self, commit: Commit) -> Result<()> {
        self.tx.send(Message::Commit(commit))?;
        info!("done");
        Ok(())
    }

    #[tracing::instrument]
    fn wait_commit(&mut self) -> Result<Commit> {
        let Message::Commit(op_commit) = self.rx.recv()? else { panic!("expected commit") };
        info!("got commit");
        Ok(op_commit)
    }

    #[tracing::instrument(skip(salt))]
    fn throw_hand(&mut self, hand: Hand, salt: Salt) -> Result<()> {
        self.tx.send(Message::Hand(hand, salt))?;
        info!("done");
        Ok(())
    }

    #[tracing::instrument]
    fn wait_hand(&mut self) -> Result<(Hand, Salt)> {
        let Message::Hand(op_hand, op_salt) = self.rx.recv()? else { panic!("expected hand") };
        info!("got {op_hand}");
        Ok((op_hand, op_salt))
    }

    #[tracing::instrument]
    fn check_result(&mut self, hand: Hand, op_hand: Hand) {
        match hand.cmp(&op_hand) {
            Ordering::Less => info!("i lost..."),
            Ordering::Equal => info!("a draw, i see"),
            Ordering::Greater => info!("i won!"),
        }
    }

    #[tracing::instrument(skip(hand, op_hand))]
    fn change_hand(&mut self, hand: &mut Hand, op_hand: Hand) {
        *hand = op_hand.opposite();
        warn!("change hand to {hand}");
    }
}
