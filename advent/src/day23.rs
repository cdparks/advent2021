use itertools::Itertools;
use sscanf::scanf;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::convert::TryInto;

/// Compute the least amount of energy necessary to organize the
/// amphipods in their rooms.
pub fn part1(lines: &[String]) -> usize {
    solve(State::<2>::parse(lines).unwrap())
}

/// Compute the least amount of energy necessary to organize the
/// amphipods in their rooms after adding more of this lil guys.
pub fn part2(lines: &[String]) -> usize {
    let mut lines = lines.iter().cloned().collect_vec();
    lines.splice(3..3, ["  #D#C#B#A#".into(), "  #D#B#A#C#".into()]);
    solve(State::<4>::parse(&lines).unwrap())
}

/// Dijkstra's algorithm to find the smallest cost to move from the
/// initial state to an organized state.
pub fn solve<const N: usize>(state: State<N>) -> usize {
    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), Opaque(state)));

    while let Some((Reverse(cost), Opaque(state))) = queue.pop() {
        if state.organized() {
            return cost;
        }

        for (new_cost, other) in state.step() {
            let total_cost = new_cost + cost;
            let previous_cost = seen.entry(other).or_insert(usize::MAX);
            if total_cost < *previous_cost {
                *previous_cost = total_cost;
                queue.push((Reverse(total_cost), Opaque(other)));
            }
        }
    }

    0
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
/// Some kind of critter, really just node labels.
pub enum Amphipod {
    /// Amber
    A,
    /// Bronze
    B,
    /// Copper
    C,
    /// Desert
    D,
}

impl Amphipod {
    /// Parse an amphipod from a character.
    pub fn parse(c: char) -> Option<Self> {
        match c {
            'A' => Some(Self::A),
            'B' => Some(Self::B),
            'C' => Some(Self::C),
            'D' => Some(Self::D),
            _ => None,
        }
    }

    /// Compute the cost to move this amphipod the number of steps.
    pub fn cost(&self, steps: usize) -> usize {
        steps
            * match self {
                Self::A => 1,
                Self::B => 10,
                Self::C => 100,
                Self::D => 1000,
            }
    }

    /// Return this amphipod's destination room index.
    pub fn room(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }

    /// Sequence of amphipods ordered from left-to-right.
    pub fn each() -> [Amphipod; 4] {
        [Self::A, Self::B, Self::C, Self::D]
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
/// A room containing no more than N amphipods.
pub struct Room<const N: usize> {
    kind: Amphipod,
    slots: [Option<Amphipod>; N],
}

impl<const N: usize> Room<N> {
    /// Initial sequence of empty rooms.
    pub fn rooms() -> [Self; 4] {
        Amphipod::each()
            .iter()
            .copied()
            .map(Self::new)
            .collect_vec()
            .try_into()
            .unwrap()
    }

    /// Create an empty room as a destination for the specified kind
    /// of amphipod.
    pub fn new(kind: Amphipod) -> Self {
        let slots = [None; N];
        Self { kind, slots }
    }

    /// Place an amphipod in the specified slot regardless of its kind.
    pub fn put(&mut self, i: usize, pod: Amphipod) {
        self.slots[i] = Some(pod);
    }

    /// Is this room full of the right kind of amphipod?
    pub fn organized(&self) -> bool {
        self.slots.iter().all(|slot| {
            if let Some(pod) = slot {
                return *pod == self.kind;
            }
            false
        })
    }

    /// Attempt to move an amphipod into this room.
    pub fn push(&mut self, pod: Amphipod) -> Option<usize> {
        if self.kind != pod {
            return None;
        }

        if self.has_others() {
            return None;
        }

        for (i, slot) in self.slots.iter_mut().enumerate().rev() {
            if slot.is_some() {
                continue;
            }
            *slot = Some(pod);
            return Some(i + 1);
        }
        None
    }

    /// Attempt to move an amphipod out of this room, returning the
    /// cost to move out into the hallway.
    pub fn pop(&mut self) -> Option<(usize, Amphipod)> {
        for i in 0..N {
            let has_others = self.has_others_from(i + 1);
            if let Some(pod) = self.slots[i].take() {
                if pod != self.kind || has_others {
                    return Some((i + 1, pod));
                }
                self.slots[i].replace(pod);
                break;
            }
        }
        None
    }

    /// Are there other kinds of amphipods in this room?
    pub fn has_others(&self) -> bool {
        self.has_others_from(0)
    }

    /// Are there other kinds of amphipods in this room starting from
    /// the slot index?
    fn has_others_from(&self, i: usize) -> bool {
        self.slots[i..]
            .iter()
            .filter_map(|slot| *slot)
            .any(|pod| pod != self.kind)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
/// Cave state consists of the hallway and 4 rooms.
pub struct State<const N: usize> {
    hall: [Option<Amphipod>; 11],
    rooms: [Room<N>; 4],
}

impl<const N: usize> State<N> {
    /// Create a new state.
    pub fn new(hall: [Option<Amphipod>; 11], rooms: [Room<N>; 4]) -> Self {
        Self { hall, rooms }
    }

    /// Parse the state from a sequence of lines.
    pub fn parse(lines: &[String]) -> Option<State<N>> {
        let mut rooms = Room::rooms();
        for (i, line) in lines[2..2 + N].iter().enumerate() {
            let (_, a, b, c, d, _) = scanf!(
                line,
                "{/[# ]+/}{}#{}#{}#{}{/[# ]+/}",
                String,
                char,
                char,
                char,
                char,
                String
            )?;
            for (j, &x) in [a, b, c, d].iter().enumerate() {
                let pod = Amphipod::parse(x)?;
                rooms[j].put(i, pod);
            }
        }
        Some(State::new([None; 11], rooms))
    }

    /// Sequence of valid states reachable from this state along with
    /// their associated costs.
    pub fn step(&self) -> Vec<(usize, Self)> {
        let mut states: Vec<(usize, Self)> = Vec::new();

        for (i, _room) in self.rooms.iter().enumerate() {
            // Hall to room
            for (j, &spot) in self.hall.iter().enumerate() {
                if let Some(pod) = spot {
                    if let Some(cost) = self.hall_to_room(j, i) {
                        let mut copy = *self;
                        if let Some(enter) = copy.rooms[i].push(pod) {
                            copy.hall[j] = None;
                            states.push((pod.cost(cost + enter), copy))
                        }
                    }
                }
            }

            let mut copy = *self;
            if let Some((exit, pod)) = copy.rooms[i].pop() {
                // Room to room
                for (j, _room) in copy.rooms.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    if let Some(enter) = copy.room_to_room(i, j) {
                        let mut copy = copy;
                        if let Some(cost) = copy.rooms[j].push(pod) {
                            states.push((pod.cost(exit + cost + enter), copy))
                        }
                    }
                }

                // Room to hall
                for (j, _spot) in copy.hall.iter().enumerate() {
                    if let Some(cost) = copy.room_to_hall(i, j) {
                        let mut copy = copy;
                        copy.hall[j] = Some(pod);
                        states.push((pod.cost(exit + cost), copy))
                    }
                }
            }
        }

        states
    }

    /// Is every room full of the right kind of amphipod?
    pub fn organized(&self) -> bool {
        self.rooms.iter().all(|room| room.organized())
    }

    /// Cost to move from one room to another, if possible.
    fn room_to_room(&self, src: usize, dest: usize) -> Option<usize> {
        self.dist(2 * src + 2, 2 * dest + 2, true)
    }

    /// Cost to move from the hall to a room, if possible.
    fn hall_to_room(&self, src: usize, dest: usize) -> Option<usize> {
        self.dist(src, 2 * dest + 2, true)
    }

    /// Cost to move from a room to the hall, if possible.
    fn room_to_hall(&self, src: usize, dest: usize) -> Option<usize> {
        self.dist(2 * src + 2, dest, false)
    }

    /// Cost to move from the source to the destination, if possible.
    /// The final argument determines whether or not the destination is
    /// allowed to be the threshold of a room.
    ///
    /// Note that a room's index can be converted to its threshold's
    /// tile's index by doubling it and adding 2:
    ///
    /// ```plaintext
    /// room:       0     1     2     3
    ///           x 2   x 2   x 2   x 2
    ///           = 0   = 2   = 4   = 6
    ///           + 2   + 2   + 2   + 2
    ///           = 2   = 4   = 6   = 8
    /// --------------------------------------
    /// hall: 0  1  2  3  4  5  6  7  8  9  10
    /// ```
    fn dist(&self, src: usize, dest: usize, to_room: bool) -> Option<usize> {
        if src == dest {
            return None;
        }

        if !to_room && [2, 4, 6, 8].contains(&dest) {
            return None;
        }

        let from = src.min(dest);
        let to = src.max(dest);
        for i in from..=to {
            if i == src {
                continue;
            }
            if self.hall[i].is_some() {
                return None;
            }
        }
        Some(to - from)
    }
}

#[derive(Debug, Copy, Clone)]
/// A value that does not contribute to a composite value's equality
/// or ordering. It is always equal to other values of the same type.
pub struct Opaque<A>(A);

impl<A> PartialEq for Opaque<A> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<A> Eq for Opaque<A> {}

impl<A> PartialOrd for Opaque<A> {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<A> Ord for Opaque<A> {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

check!(ex 1 = 12521, ex 2 = 44169, part 1 = 18282, part 2 = 50132);
bench!(part 1, part 2);
