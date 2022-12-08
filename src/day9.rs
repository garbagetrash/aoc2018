use scan_fmt::scan_fmt;
use std::fmt::Debug;

#[derive(Clone, Debug)]
struct LinkedListNode<T> {
    head: Option<usize>,
    tail: Option<usize>,
    value: T,
}

impl<T> LinkedListNode<T> {
    fn new(value: T) -> Self {
        Self {
            head: None,
            tail: None,
            value,
        }
    }

    fn next(&self) -> Option<usize> {
        // Tail is what is to come
        self.tail
    }

    fn prev(&self) -> Option<usize> {
        // Head is what came before
        self.head
    }
}

#[derive(Clone, Debug)]
struct LinkedList<T> {
    data: Vec<LinkedListNode<T>>,
    head: Option<usize>,
    tail: Option<usize>,
    size: usize,
}

impl<T: Copy + Debug> LinkedList<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_tail(&mut self, value: T) -> usize {
        let mut new_node = LinkedListNode::new(value);
        new_node.head = self.tail;
        let new_id = self.data.len();
        if let Some(tail_id) = self.tail {
            self.data[tail_id].tail = Some(new_id);
            self.tail = Some(new_id);
        } else {
            self.head = Some(new_id);
            self.tail = Some(new_id);
        }

        self.data.push(new_node);
        self.size += 1;

        new_id
    }

    /// Returns the new node ID.
    fn insert_after(&mut self, value: T, node_id: usize) -> usize {
        let mut new_node = LinkedListNode::new(value);
        let new_id = self.data.len();
        new_node.head = Some(node_id);
        new_node.tail = self.data[node_id].tail;
        self.data[node_id].tail = Some(new_id);
        if let Some(next_id) = new_node.tail {
            self.data[next_id].head = Some(new_id);
        } else {
            // If no tail to node_id, then it was the list tail
            self.tail = Some(new_id);
        }
        self.data.push(new_node);
        self.size += 1;
        new_id
    }

    /// NOTE: This removes from the list, but not the underlying data vector.
    fn pop_id(&mut self, node_id: usize) -> T {
        if node_id > self.data.len() {
            panic!("Tried to remove id not in list");
        }
        let head_id = self.data[node_id].head;
        let tail_id = self.data[node_id].tail;

        if let Some(hid) = head_id {
            self.data[hid].tail = tail_id;
        } else {
            // If no head to node_id, then it was the list head
            self.head = tail_id;
        }
        if let Some(tid) = tail_id {
            self.data[tid].head = head_id;
        } else {
            // If no tail to node_id, then it was the list tail
            self.tail = head_id;
        }
        self.size -= 1;
        self.data[node_id].value
    }
}

#[aoc_generator(day9)]
fn load_input(input: &str) -> (usize, usize) {
    scan_fmt!(
        input,
        "{} players; last marble is worth {} points",
        usize,
        usize
    )
    .unwrap()
}

fn solve(n_players: usize, n_marbles: usize) -> usize {
    let mut players: Vec<usize> = vec![0; n_players];
    let mut marbles: LinkedList<usize> = LinkedList::with_capacity(n_marbles);
    let mut id = marbles.push_tail(0);
    // Make it a circularly doubly linked list
    marbles.data[id].head = Some(id);
    marbles.data[id].tail = Some(id);

    for i in 1..n_marbles {
        if i % 23 == 0 {
            let player_id = i % n_players;
            players[player_id] += i;

            for _ in 0..6 {
                id = marbles.data[id].prev().unwrap();
            }
            players[player_id] += marbles.pop_id(marbles.data[id].prev().unwrap());
        } else {
            id = marbles.data[id].next().unwrap();
            id = marbles.insert_after(i, id);
        }
    }
    *players.iter().max().unwrap()
}

#[aoc(day9, part1)]
pub fn part1(input: &(usize, usize)) -> usize {
    solve(input.0, input.1)
}

#[aoc(day9, part2)]
pub fn part2(input: &(usize, usize)) -> usize {
    let &(n_players, mut n_marbles) = input;
    n_marbles *= 100;
    solve(n_players, n_marbles)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&load_input("10 players; last marble is worth 1618 points")),
            8317
        );
        assert_eq!(
            part1(&load_input("13 players; last marble is worth 7999 points")),
            146373
        );
        //assert_eq!(part1(&load_input("17 players; last marble is worth 1104 points")), 2764);
        assert_eq!(
            part1(&load_input("21 players; last marble is worth 6111 points")),
            54718
        );
        assert_eq!(
            part1(&load_input("30 players; last marble is worth 5807 points")),
            37305
        );
    }
}
