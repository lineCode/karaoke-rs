use karaoke::collection::Kfile;

use crossbeam_channel::{bounded, Sender, Receiver};

lazy_static! {
    pub static ref PLAYER_CHANNEL: (Sender<PlayerCommand>, Receiver<PlayerCommand>) = { 
        let (player_send, player_receive) = bounded(1);
        (player_send, player_receive)
    };

    pub static ref LIVE_CHANNEL: (Sender<LiveCommand>, Receiver<LiveCommand>) = { 
        let (live_send, live_receive) = bounded(1);
        (live_send, live_receive)
    };
}

#[derive(Eq, PartialEq, Debug)]
pub enum PlayerCommand {
    Stop,
    Next,
    PlayNow { kfile: Kfile },
    ClearQueue,
}

#[derive(Eq, PartialEq, Debug)]
pub enum LiveCommand {
    Stop,
}