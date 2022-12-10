use super::*;
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use std::collections::HashMap;

new_key_type! {
    struct EventKey;

    struct GroupKey;
}

#[derive(Clone, Debug, Default)]
struct Group {
    members: Vec<EventKey>,
    data: Event,
}

#[derive(Clone, Debug, Default)]
pub struct Database {
    events: SlotMap<EventKey, Event>,
    event_groups: SecondaryMap<EventKey, Vec<GroupKey>>,
    group_values: HashMap<Event, GroupKey>,
    groups: SlotMap<GroupKey, Group>,
}

impl Database {
    pub fn select_index(&self, group: &Event, index: &Event) -> Option<Event> {
        let index = index.as_number()?.whole();
        let group_key = *self.group_values.get(group)?;
        let group = &self.groups[group_key];
        if group.members.is_empty() {
            return None;
        }
        let idx = (index % group.members.len() as i64) as usize;
        let key = group.members[idx];
        let event = self.events.get(key)?.clone();
        Some(event)
    }

    pub fn select_group<'a>(&'a self, group: &'a Event) -> impl Iterator<Item = &'a Event> + 'a {
        self.group_members(group)
            .into_iter()
            .flat_map(|members| members.iter().map(|key| &self.events[*key]))
    }

    fn group_members(&self, group: &Event) -> Option<&[EventKey]> {
        let group_key = self.group_values.get(group)?;
        let group = &self.groups[*group_key];
        Some(&group.members)
    }

    pub fn select_all<'a, G: Iterator<Item = &'a Event> + 'a>(
        &'a self,
        groups: G,
    ) -> impl Iterator<Item = &'a Event> + 'a {
        groups.flat_map(|group| self.select_group(group))
    }

    pub fn push<'a, G: Iterator<Item = &'a Event>>(&mut self, groups: G, value: Event) {
        let key = self.events.insert(value);
        let mut event_groups = vec![];

        for group in groups {
            if let Some(group_key) = self.group_values.get(group) {
                self.groups[*group_key].members.push(key);
                event_groups.push(*group_key);
                continue;
            }

            let group_key = self.groups.insert(Group {
                data: group.clone(),
                members: vec![key],
            });
            self.group_values.insert(group.clone(), group_key);
            event_groups.push(group_key);
        }

        self.event_groups.insert(key, event_groups);
    }

    pub fn group_len(&self, group: &Event) -> usize {
        self.group_members(group).map_or(0, |m| m.len())
    }

    pub fn remove(&mut self, group: &Event, index: &Event) -> Option<()> {
        let index = index.as_number()?.whole();
        let group_key = self.group_values.get(group)?;
        let group = &mut self.groups[*group_key];

        if group.members.is_empty() {
            return None;
        }

        let idx = (index % group.members.len() as i64) as usize;

        let key = group.members.remove(idx);

        let mut group_list = vec![];
        self.remove_event(key, &mut group_list);

        self.remove_groups(&mut group_list);

        Some(())
    }

    pub fn remove_group(&mut self, group: &Event) -> Option<()> {
        let group_key = self.group_values.remove(group)?;
        let mut group_list = vec![];
        self.remove_group_by_key(group_key, &mut group_list);
        self.remove_groups(&mut group_list);
        Some(())
    }

    fn remove_group_by_key(
        &mut self,
        group_key: GroupKey,
        group_list: &mut Vec<GroupKey>,
    ) -> Option<()> {
        let group = self.groups.remove(group_key)?;

        for event in group.members {
            self.remove_event_group(event, group_key, group_list);
        }

        self.remove_groups(group_list);

        Some(())
    }

    pub fn remove_group_members(&mut self, group: &Event) -> Option<()> {
        let group_key = self.group_values.remove(group)?;

        let mut group_list = vec![];
        self.remove_group_events(group_key, &mut group_list);

        self.remove_groups(&mut group_list);

        Some(())
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.event_groups.clear();
        self.group_values.clear();
        self.groups.clear();
    }

    fn remove_event(&mut self, key: EventKey, group_list: &mut Vec<GroupKey>) {
        let _ = self.events.remove(key);
        if let Some(keys) = self.event_groups.remove(key) {
            for group_key in keys {
                if let Some(group) = self.groups.get_mut(group_key) {
                    group.members.retain(|member| *member != key);

                    if group.members.is_empty() {
                        group_list.push(group_key);
                    }
                }
            }
        }
    }

    fn remove_event_group(
        &mut self,
        key: EventKey,
        group: GroupKey,
        group_list: &mut Vec<GroupKey>,
    ) {
        if let Some(groups) = self.event_groups.get_mut(key) {
            groups.retain(|g| *g != group);

            if groups.is_empty() {
                self.remove_event(key, group_list);
            }
        }
    }

    fn remove_group_events(&mut self, key: GroupKey, group_list: &mut Vec<GroupKey>) {
        if let Some(group) = self.groups.remove(key) {
            self.group_values.remove(&group.data);

            for member in group.members {
                self.remove_event(member, group_list);
            }
        }
    }

    fn remove_groups(&mut self, groups: &mut Vec<GroupKey>) {
        while let Some(group_key) = groups.pop() {
            self.remove_group_by_key(group_key, groups);
        }
    }
}
