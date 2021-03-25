// http://doc.sccode.org/Reference/Server-Command-Reference.html

use core::fmt;
use euphony_osc::{
    codec::{
        self,
        encode::{EncoderBuffer, TypeEncoder},
    },
    types::{Blob, Str, Tagged},
    Message,
};

macro_rules! impl_id {
    () => {
        impl_id!(Id);
    };
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
        pub struct $name(pub i32);
    };
}

macro_rules! enum_traits {
    ($name:ident) => {
        impl<Buf: EncoderBuffer> TypeEncoder<Buf> for $name {
            fn encode_type(self, buffer: Buf) -> codec::buffer::Result<(), Buf> {
                (self as i32).encode_type(buffer)
            }
        }

        impl<B: EncoderBuffer> Tagged<B> for $name {
            fn encode_tag(&self, buffer: B) -> codec::buffer::Result<(), B> {
                (*self as i32).encode_tag(buffer)
            }
        }
    };
}

#[cfg(test)]
macro_rules! snapshot {
    ($value:expr) => {{
        let value = $value;
        let mut buf = [0u8; 1024];
        let (len, _) = buf.encode(value).unwrap();
        let msg = rosc::decoder::decode(&buf[..len]);
        ::insta::assert_debug_snapshot!(msg);
    }};
}

pub mod server {
    use super::*;

    /// Quit program. Exits the synthesis server.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/quit")]
    pub struct Quit;

    #[test]
    fn quit_test() {
        snapshot!(Quit);
    }

    /// Register to receive notifications from server
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/notify")]
    pub struct Notify {
        #[osc(encoder = "bool_encoder")]
        pub enabled: bool,
        pub client_id: Option<i32>,
    }

    /// Query the status.
    ///
    /// Replies to sender with a /status.reply message
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/status")]
    pub struct Status;

    #[test]
    fn status_test() {
        snapshot!(Status);
    }

    // TODO status.reply

    // TODO cmd

    // TODO dumpOSC

    /// Notify when async commands have completed.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/sync")]
    pub struct Sync;

    #[test]
    fn sync_test() {
        snapshot!(Sync);
    }

    /// Clear all scheduled bundles. Removes all bundles from the scheduling queue.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/clearSched")]
    pub struct ClearScheduled;

    #[test]
    fn clear_sched_test() {
        snapshot!(ClearScheduled);
    }

    /// Enable/disable error message posting.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/error")]
    pub struct Error {
        pub mode: ErrorMode,
    }

    #[test]
    fn error_test() {
        snapshot!(Error {
            mode: ErrorMode::Disabled
        });
        snapshot!(Error {
            mode: ErrorMode::Enabled
        });
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i32)]
    pub enum ErrorMode {
        Disabled = 0,
        Enabled = 1,
        LocalDisabled = -1,
        LocalEnabled = -2,
    }

    enum_traits!(ErrorMode);

    /// Query the SuperCollider version. Replies to sender with a /version.reply message
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/version")]
    pub struct Version;

    #[test]
    fn version_test() {
        snapshot!(Version);
    }
}

pub mod control {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Id<'a> {
        Index(i32),
        Name(&'a str),
    }

    impl<'a, B: EncoderBuffer> Tagged<B> for Id<'a> {
        fn encode_tag(&self, buffer: B) -> codec::buffer::Result<(), B> {
            match self {
                Self::Index(v) => v.encode_tag(buffer),
                Self::Name(v) => v.encode_tag(buffer),
            }
        }
    }

    impl<'a, B: EncoderBuffer> TypeEncoder<B> for Id<'a> {
        fn encode_type(self, buffer: B) -> codec::buffer::Result<(), B> {
            match self {
                Self::Index(value) => value.encode_type(buffer),
                Self::Name(value) => Str(value).encode_type(buffer),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum Value {
        Float(f32),
        Int(i32),
    }

    impl fmt::Debug for Value {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Float(v) => v.fmt(f),
                Self::Int(v) => v.fmt(f),
            }
        }
    }

    impl<B: EncoderBuffer> Tagged<B> for Value {
        fn encode_tag(&self, buffer: B) -> codec::buffer::Result<(), B> {
            match self {
                Self::Float(v) => v.encode_tag(buffer),
                Self::Int(v) => v.encode_tag(buffer),
            }
        }
    }

    impl<B: EncoderBuffer> TypeEncoder<B> for Value {
        fn encode_type(self, buffer: B) -> codec::buffer::Result<(), B> {
            match self {
                Self::Float(value) => value.encode_type(buffer),
                Self::Int(value) => value.encode_type(buffer),
            }
        }
    }

    impl From<f32> for Value {
        fn from(value: f32) -> Value {
            Value::Float(value)
        }
    }

    impl From<i32> for Value {
        fn from(value: i32) -> Value {
            Value::Int(value)
        }
    }

    impl_id!(Bus);
}

pub mod synthdef {
    use super::*;

    /// Receive a synth definition file.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/d_recv")]
    pub struct Receive<'a> {
        #[osc(encoder = "Blob")]
        pub buffer: &'a [u8],
        // pub completion: Option<&'a [u8]>,
    }

    #[test]
    fn receive_test() {
        snapshot!(Receive { buffer: &[] });
        snapshot!(Receive { buffer: &[1] });
        snapshot!(Receive { buffer: &[1, 2] });
        snapshot!(Receive { buffer: &[1, 2, 3] });
        snapshot!(Receive {
            buffer: &[1, 2, 3, 4]
        });
        snapshot!(Receive {
            buffer: &[1, 2, 3, 4, 5]
        });
    }

    /// Load synth definition.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/d_load")]
    pub struct Load<'a> {
        #[osc(encoder = "Str")]
        pub path: &'a str,
        // pub completion: Option<&'a [u8]>,
    }

    #[test]
    fn load_test() {
        snapshot!(Load {
            path: "path/to/def"
        });
    }

    /// Load a directory of synth definitions.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/d_loadDir")]
    pub struct LoadDir<'a> {
        #[osc(encoder = "Str")]
        pub path: &'a str,
        // pub completion: Option<&'a [u8]>,
    }

    #[test]
    fn load_dir_test() {
        snapshot!(LoadDir {
            path: "path/to/dir"
        });
    }

    /// Delete synth definition.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/d_free")]
    pub struct Free<'a> {
        #[osc(encoder = "Str")]
        pub name: &'a str,
    }

    #[test]
    fn free_test() {
        snapshot!(Free {
            name: "my synthdef"
        });
    }
}

pub mod node {
    use super::*;

    impl_id!();

    /// Delete a node.
    ///
    /// Stops a node abruptly, removes it from its group,
    /// and frees its memory. A list of node IDs may be
    /// specified. Using this method can cause a click
    /// if the node is not silent at the time it is freed.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_free")]
    pub struct Free {
        pub id: Id,
    }

    #[test]
    fn free_test() {
        snapshot!(Free { id: Id(2) });
    }

    /// Turn node on or off.
    ///
    /// Using this method to start and stop nodes can cause a click if the node is not silent at the time run flag is toggled.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_run")]
    pub struct Run {
        pub id: Id,
        #[osc(encoder = "bool_encoder")]
        pub enabled: bool,
    }

    #[test]
    fn run_test() {
        snapshot!(Run {
            id: Id(2),
            enabled: true
        });
        snapshot!(Run {
            id: Id(2),
            enabled: false
        });
    }

    /// Set a node's control value.
    ///
    /// Takes a pair of control indices and values and sets the controls to those values. If the node is a group, then it sets the controls of every node in the group.
    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/n_set")]
    pub struct Set<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub controls: &'a [(control::Id<'a>, control::Value)],
    }

    #[test]
    fn set_test() {
        snapshot!(Set {
            id: Id(2),
            controls: &[
                (control::Id::Index(1), control::Value::Float(3.14)),
                (control::Id::Name("controla"), control::Value::Int(123)),
            ],
        });
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/n_set")]
    pub struct SetOptional<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub controls: &'a [Option<(control::Id<'a>, control::Value)>],
    }

    /// Set a range of a node's control values.
    ///
    /// Set contiguous ranges of control indices to sets of values. For each range, the starting control index is given followed by the number of controls to change, followed by the values. If the node is a group, then it sets the controls of every node in the group.
    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/n_setn")]
    pub struct SetN<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub ranges: &'a [SetNRange<'a>],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    pub struct SetNRange<'a> {
        pub id: control::Id<'a>,
        #[osc(flatten, len_prefix = "i32")]
        pub values: &'a [control::Value],
    }

    #[test]
    fn setn_test() {
        snapshot!(SetN {
            id: Id(2),
            ranges: &[SetNRange {
                id: control::Id::Index(1),
                values: &[control::Value::Float(3.14), control::Value::Int(123),]
            }],
        });
    }

    /// Fill ranges of a node's control values.
    ///
    /// Set contiguous ranges of control indices to single values. For each range, the starting control index is given followed by the number of controls to change, followed by the value to fill. If the node is a group, then it sets the controls of every node in the group.
    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/n_fill")]
    pub struct Fill<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub ranges: &'a [FillRange<'a>],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    pub struct FillRange<'a> {
        pub id: control::Id<'a>,
        pub count: i32,
        pub value: control::Value,
    }

    #[test]
    fn fill_test() {
        snapshot!(Fill {
            id: Id(2),
            ranges: &[FillRange {
                id: control::Id::Index(1),
                count: 3,
                value: control::Value::Float(3.24),
            }],
        });
    }

    /// Map a node's controls to read from a control bus.
    ///
    /// Takes a list of pairs of control names or indices and bus indices and causes those controls to be read continuously from a global control bus. If the node is a group, then it maps the controls of every node in the group. If the control bus index is -1 then any current mapping is undone. Any n_set, n_setn and n_fill command will also unmap the control.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_map")]
    pub struct Map<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub controls: &'a [(control::Id<'a>, control::Bus)],
    }

    #[test]
    fn map_test() {
        snapshot!(Map {
            id: Id(2),
            controls: &[(control::Id::Index(1), control::Bus(2),)],
        });
    }

    /// Map a node's controls to read from buses.
    ///
    /// Takes a list of triplets of control names or indices, bus indices, and number of controls to map and causes those controls to be mapped sequentially to buses. If the node is a group, then it maps the controls of every node in the group. If the control bus index is -1 then any current mapping is undone. Any n_set, n_setn and n_fill command will also unmap the control.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_mapn")]
    pub struct MapN<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub controls: &'a [MapControl<'a>],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    pub struct MapControl<'a> {
        pub id: control::Id<'a>,
        pub bus: control::Bus,
        pub count: i32,
    }

    /// Map a node's controls to read from an audio bus.
    ///
    /// Takes a list of pairs of control names or indices and audio bus indices and causes those controls to be read continuously from a global audio bus. If the node is a group, then it maps the controls of every node in the group. If the audio bus index is -1 then any current mapping is undone. Any n_set, n_setn and n_fill command will also unmap the control. For the full audio rate signal, the argument must have its rate set to \ar.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_mapa")]
    pub struct MapA<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub controls: &'a [(control::Id<'a>, control::Bus)],
    }

    /// Map a node's controls to read from audio buses.
    ///
    /// Takes a list of triplets of control names or indices, audio bus indices, and number of controls to map and causes those controls to be mapped sequentially to buses. If the node is a group, then it maps the controls of every node in the group. If the audio bus index is -1 then any current mapping is undone. Any n_set, n_setn and n_fill command will also unmap the control. For the full audio rate signal, the argument must have its rate set to \ar.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_mapan")]
    pub struct MapAN<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub controls: &'a [MapControl<'a>],
    }

    /// Place a node before another.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_before")]
    pub struct Before {
        /// the ID of the node to place
        pub subject: Id,
        /// the ID of the node before which the above is placed
        pub target: Id,
    }

    /// Place a node after another.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_after")]
    pub struct After {
        /// the ID of the node to place
        pub subject: Id,
        /// the ID of the node after which the above is placed
        pub target: Id,
    }

    /// Get info about a node.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_query")]
    pub struct Query {
        pub id: Id,
    }

    /// Trace a node.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_trace")]
    pub struct Trace {
        pub id: Id,
    }

    /// Move and order a list of nodes
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/n_order")]
    pub struct Order<'a> {
        pub action: Action,
        pub target: Id,
        #[osc(flatten)]
        pub ids: &'a [Id],
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i32)]
    pub enum Action {
        /// construct the node order at the head of the group specified by the add target ID.
        Head = 0,
        /// construct the node order at the tail of the group specified by the add target ID.
        Tail,
        /// construct the node order just before the node specified by the add target ID.
        Before,
        /// construct the node order just after the node specified by the add target ID.
        After,
    }

    enum_traits!(Action);
}

pub mod synth {
    use super::*;

    /// Create a new synth.
    ///
    /// Create a new synth from a synth definition, give it an ID, and add it to the tree of nodes.
    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/s_new")]
    pub struct New<'a> {
        #[osc(encoder = "Str")]
        pub name: &'a str,
        pub id: node::Id,
        pub action: group::Action,
        pub target: node::Id,
        #[osc(flatten)]
        pub values: &'a [(control::Id<'a>, control::Value)],
        // TODO a symbol argument consisting of the letter 'c' or 'a' (for control or audio) followed by the bus's index.
        #[osc(flatten)]
        pub busses: &'a [(control::Id<'a>, control::Bus)],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/s_new")]
    pub struct NewOptional<'a> {
        #[osc(encoder = "Str")]
        pub name: &'a str,
        pub id: node::Id,
        pub action: group::Action,
        pub target: node::Id,
        #[osc(flatten)]
        pub values: &'a [Option<(control::Id<'a>, control::Value)>],
    }

    // TODO s_get
    // TODO s_getn
    // TODO s_noid
}

pub mod group {
    use super::*;

    impl_id!();

    /// Create a new group.
    ///
    /// Create a new group and add it to the tree of nodes.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/g_new")]
    pub struct New {
        pub id: Id,
        pub action: Action,
        pub target: node::Id,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i32)]
    pub enum Action {
        /// construct the node order at the head of the group specified by the add target ID.
        Head = 0,
        /// construct the node order at the tail of the group specified by the add target ID.
        Tail,
        /// construct the node order just before the node specified by the add target ID.
        Before,
        /// construct the node order just after the node specified by the add target ID.
        After,
        /// the new node replaces the node specified by the add target ID. The target node is freed.
        Replace,
    }

    impl Default for Action {
        fn default() -> Self {
            Action::Tail
        }
    }

    enum_traits!(Action);

    /// Create a new parallel group.
    ///
    /// Create a new parallel group and add it to the tree of nodes. Parallel groups are relaxed groups, their child nodes are evaluated in unspecified order.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/p_new")]
    pub struct PNew {
        pub id: Id,
        pub action: Action,
        pub target: node::Id,
    }

    /// Add node to head of group.
    ///
    /// Adds the node to the head (first to be executed) of the group.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/g_head")]
    pub struct Head {
        pub id: Id,
        pub node: node::Id,
    }

    /// Add node to tail of group.
    ///
    /// Adds the node to the tail (last to be executed) of the group.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/g_tail")]
    pub struct Tail {
        pub id: Id,
        pub node: node::Id,
    }

    /// Delete all nodes in a group.
    ///
    /// Frees all nodes in the group.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/g_freeAll")]
    pub struct FreeAll {
        pub id: Id,
    }

    /// Free all synths in this group and all its sub-groups.
    ///
    /// Traverses all groups below this group and frees all the synths. Sub-groups are not freed.
    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/g_deepFree")]
    pub struct DeepFree {
        pub id: Id,
    }

    // TODO dumpTree
    // TODO queryTree
}

pub mod buffer {
    use super::*;

    impl_id!();

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_alloc")]
    pub struct Alloc {
        pub id: Id,
        pub len: i32,
        pub channels: i32,
        // pub completion: Option<&'a [u8]>,
    }

    #[test]
    fn alloc_test() {
        snapshot!(Alloc {
            id: Id(1),
            len: 123,
            channels: 456
        });
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_allocRead")]
    pub struct AllocRead<'a> {
        pub id: Id,
        #[osc(encoder = "Str")]
        pub path: &'a str,
        pub offset: i32,
        pub len: i32,
        // pub completion: Option<&'a [u8]>,
    }

    #[test]
    fn alloc_read_test() {
        snapshot!(AllocRead {
            id: Id(1),
            path: "path/to/buffer",
            offset: 123,
            len: 456,
        });
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_allocReadChannel")]
    pub struct AllocReadChannel<'a> {
        pub id: Id,
        #[osc(encoder = "Str")]
        pub path: &'a str,
        pub offset: i32,
        pub len: i32,
        #[osc(flatten)]
        pub channels: &'a [i32],
        // pub completion: Option<&'a [u8]>,
    }

    #[test]
    fn alloc_read_channel_test() {
        snapshot!(AllocReadChannel {
            id: Id(1),
            path: "path/to/buffer",
            offset: 123,
            len: 456,
            channels: &[1, 2, 3, 4, 5]
        });
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_read")]
    pub struct Read<'a> {
        pub id: Id,
        #[osc(encoder = "Str")]
        pub path: &'a str,
        pub offset: i32,
        pub len: i32,
        pub buffer_offset: i32,
        #[osc(encoder = "bool_encoder")]
        pub leave_file_open: bool,
        // pub completion: Option<&'a [u8]>,
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_readChannel")]
    pub struct ReadChannel<'a> {
        pub id: Id,
        #[osc(encoder = "Str")]
        pub path: &'a str,
        pub offset: i32,
        pub len: i32,
        pub buffer_offset: i32,
        #[osc(encoder = "bool_encoder")]
        pub leave_file_open: bool,
        #[osc(flatten)]
        pub channels: &'a [i32],
        // pub completion: Option<&'a [u8]>,
    }

    // TODO b_write

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_free")]
    pub struct Free {
        pub id: Id,
        // pub completion: Option<&'a [u8]>,
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_zero")]
    pub struct Zero {
        pub id: Id,
        // pub completion: Option<&'a [u8]>,
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/b_set")]
    pub struct Set<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub values: &'a [(i32, f32)],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/b_setm")]
    pub struct SetN<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub ranges: &'a [SetNRange<'a>],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    pub struct SetNRange<'a> {
        pub offset: i32,
        #[osc(flatten, len_prefix = "i32")]
        pub values: &'a [f32],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    #[osc(address = "/b_fill")]
    pub struct Fill<'a> {
        pub id: Id,
        #[osc(flatten)]
        pub ranges: &'a [FillRange],
    }

    #[derive(Clone, Copy, Debug, Message, PartialEq)]
    pub struct FillRange {
        pub offset: i32,
        pub len: i32,
        pub value: f32,
    }

    // TODO b_gen

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/b_close")]
    pub struct Close {
        pub id: Id,
        // pub completion: Option<&'a [u8]>,
    }

    // TODO query
    // TODO get
    // TODO getn
}

pub mod control_bus {
    // TODO
}

pub mod nrt {
    use super::*;

    #[derive(Clone, Copy, Debug, Message, PartialEq, Eq)]
    #[osc(address = "/nrt_end")]
    pub struct End;
}

fn bool_encoder(value: bool) -> i32 {
    if value {
        1
    } else {
        0
    }
}
