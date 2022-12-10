use super::*;

#[derive(Clone, Debug)]
pub enum Table {
    SelectIndex {
        group: Local,
        index: Local,
        destination: Local,
    },
    Push {
        source: Local,
        groups: Vec<Local>,
    },
    GroupLen {
        group: Local,
        destination: Local,
    },
    RemoveGroupIndex {
        group: Local,
        index: Local,
    },
    RemoveGroup {
        group: Local,
    },
    RemoveGroupMembers {
        group: Local,
    },
    SendAll {
        groups: Vec<Local>,
        output: Local,
    },
}

impl Table {
    pub(super) fn apply(&self, db: &mut Database, locals: &mut [Event], outputs: &mut [Output]) {
        match self {
            Table::SelectIndex {
                group,
                index,
                destination,
            } => {
                let group = &locals[group.id];
                let index = &locals[index.id];
                let event = db.select_index(group, index).unwrap_or_default();
                locals[destination.id] = event;
            }
            Table::Push { source, groups } => {
                let value = locals[source.id].clone();
                let groups = groups.iter().map(|g| &locals[g.id]);
                db.push(groups, value);
            }
            Table::GroupLen { group, destination } => {
                let group = &locals[group.id];
                locals[destination.id] = (db.group_len(group) as i64).into();
            }
            Table::RemoveGroupIndex { group, index } => {
                let group = &locals[group.id];
                let index = &locals[index.id];
                db.remove(group, index);
            }
            Table::RemoveGroup { group } => {
                let group = &locals[group.id];
                db.remove_group(group);
            }
            Table::RemoveGroupMembers { group } => {
                let group = &locals[group.id];
                db.remove_group_members(group);
            }
            Table::SendAll { groups, output } => {
                if outputs.is_empty() {
                    return;
                }

                let groups = groups.iter().map(|g| &locals[g.id]);
                if let Some(n) = locals[output.id].as_number() {
                    let n = n.whole() as usize;
                    let idx = n % outputs.len();

                    let output = &mut outputs[idx];

                    for event in db.select_all(groups) {
                        output.send(event.clone());
                    }
                }
            }
        }
    }
}
