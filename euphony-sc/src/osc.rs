// http://doc.sccode.org/Reference/Server-Command-Reference.html

use euphony_osc::Message;

pub mod controls {
    use super::*;

    /// Quit program. Exits the synthesis server.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/quit"]
    pub struct Quit;

    /// Register to receive notifications from server
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/notify"]
    pub struct Notify {
        pub enabled: bool,
        pub client_id: Option<i32>,
    }

    /// Query the status.
    ///
    /// Replies to sender with a /status.reply message
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/status"]
    pub struct Status;

    // TODO status.reply

    // TODO cmd

    // TODO dumpOSC

    /// Notify when async commands have completed.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/sync"]
    pub struct Sync;

    /// Clear all scheduled bundles. Removes all bundles from the scheduling queue.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/clearSched"]
    pub struct ClearScheduled;

    /// Enable/disable error message posting.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/error"]
    pub struct Error {
        pub mode: i32, // TODO use ErrorMode
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i32)]
    pub enum ErrorMode {
        Disabled = 0,
        Enabled = 1,
        LocalDisabled = -1,
        LocalEnabled = -2,
    }

    /// Query the SuperCollider version. Replies to sender with a /version.reply message
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/version"]
    pub struct Version;
}

pub mod synth {
    use super::*;

    /// Receive a synth definition file.
    #[derive(Clone, Debug, Message, PartialEq, Eq)]
    #[address = "/d_recv"]
    pub struct Receive {
        pub buffer: Bytes,
        pub completion: Option<Bytes>,
    }

    /// Load synth definition.
    #[derive(Clone, Debug, Message, PartialEq, Eq)]
    #[address = "/d_load"]
    pub struct Load {
        pub path: String,
        pub completion: Option<Bytes>,
    }

    /// Load a directory of synth definitions.
    #[derive(Clone, Debug, Message, PartialEq, Eq)]
    #[address = "/d_loadDir"]
    pub struct LoadDir {
        pub path: String,
        pub completion: Option<Bytes>,
    }

    /// Delete synth definition.
    #[derive(Clone, Debug, Message, PartialEq, Eq)]
    #[address = "/d_free"]
    pub struct Free {
        // TODO support a list
        pub name: String,
    }
}

pub mod node {
    use super::*;

    /// Delete a node.
    ///
    /// Stops a node abruptly, removes it from its group,
    /// and frees its memory. A list of node IDs may be
    /// specified. Using this method can cause a click
    /// if the node is not silent at the time it is freed.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/n_free"]
    pub struct Free {
        // TODO support a list
        pub id: i32,
    }
    
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[address = "/n_run"]
    pub struct Run {
        // TODO support a list
        pub id: i32,
        #[message_type = i32]
        pub enabled: bool,
    }
}
