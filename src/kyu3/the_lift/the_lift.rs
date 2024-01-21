use std::cmp::{max, min};

enum Direction {
    Up,
    Down,
}

struct Lift {
    capacity: usize,
    people: Vec<usize>,
    current_floor: usize,
    direction: Direction,
}

impl Lift {
    fn next_floor_to_get_out(&self) -> Option<usize> {
        match self.direction {
            Direction::Up => {return self.people.iter().filter_map(|&p| if p > self.current_floor {Some(p)} else {None}).min()},
            Direction::Down => {return self.people.iter().filter_map(|&p| if p < self.current_floor {Some(p)} else {None}).max()},
        }
    }

    fn next_floor_to_get_in(&self, queues: &[Vec<usize>]) -> Option<usize> {
        match self.direction {
            Direction::Up => {
                //Return the floor where someone is going up
                for idx in self.current_floor + 1 .. queues.len() {
                    if queues[idx]
                        .iter()
                        .filter_map(|&p| if p > idx {Some(p)} else {None})
                        .collect::<Vec<usize>>()
                        .len() > 0 {return Some(idx)};
                };
                //Return the highest floor where there is someone - but only if there is noone in the lift
                if self.people.len() == 0 {
                    for idx in (self.current_floor + 1 .. queues.len()).rev() {
                        if queues[idx].len() > 0 {return Some(idx)}
                    }
                };
            },
            Direction::Down => {
                //Return the floor where someone is going down
                for idx in (0..self.current_floor).rev() {
                    if queues[idx].to_vec()
                        .iter()
                        .filter_map(|&p| if p < idx {Some(p)} else {None})
                        .collect::<Vec<usize>>()
                        .len() > 0 {return Some(idx)};
                };
                //Return the lowest floor where there is someone - but only if there is noone in the lift
                if self.people.len() == 0 {
                    for idx in 0..self.current_floor {
                        if queues[idx].len() > 0 {return Some(idx)}
                    }
                };
            },
        }
        None
    }

    fn next_floor(&mut self, queues: &[Vec<usize>]) -> usize {
        let next_floor_get_out = self.next_floor_to_get_out();
        let next_floor_get_in = self.next_floor_to_get_in(queues);

        match (next_floor_get_out, next_floor_get_in) {
            (None, None) => {
                match self.direction {
                    Direction::Up => {
                        self.change_direction();
                        return self.next_floor(queues);
                    },
                    Direction::Down => {
                        if vec2d_empty(queues) && self.people.len() == 0 {return 0}
                        else {
                            self.change_direction();
                            return self.next_floor(queues);
                        }
                    },
                }
            },
            (None, Some(value)) => value,
            (Some(value), None) => value,
            (Some(value1), Some(value2)) => {
                match self.direction {
                    Direction:: Down => max(value1, value2),
                    Direction:: Up => min(value1, value2),
                }
            }
        }
    }
    
    fn process_floor(&mut self, queue: &mut Vec<usize>) {
        //People get off
        self.people = self.people.iter().filter_map(|&p| if p != self.current_floor {Some(p)} else {None}).collect();
        
        //People get on
        let mut counter = self.capacity - self.people.len();
        let mut idx_to_remove = vec![];
        for (idx, &p) in queue.iter().enumerate() {
            if counter == 0 {break};
            match self.direction {
                Direction::Up => {
                    if p > self.current_floor {
                        counter -= 1;
                        idx_to_remove.push(idx);
                    }
                },
                Direction::Down => {
                    if p < self.current_floor {
                        counter -= 1;
                        idx_to_remove.push(idx);
                    }
                },
            }
        }
        for &idx in idx_to_remove.iter() {
            self.people.push(queue[idx].clone())
        };
        let mut idx = 0;
        queue.retain(|_| {
            idx += 1;
            !idx_to_remove.contains(&(idx - 1))
        })
    }

    fn change_direction(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Up,
        }
    }
    
    fn is_change_direction(&self, queues: &[Vec<usize>]) -> bool {
        //If there is anyone going in the lift's direction, the lift continues
        match self.direction {
            Direction::Up => {
                if self.people
                    .iter()
                    .filter_map(|&p| if p > self.current_floor {Some(p)} else {None})
                    .collect::<Vec<usize>>()
                    .len() > 0 {return false};
            },
            Direction::Down => {
                if self.people
                    .iter()
                    .filter_map(|&p| if p < self.current_floor {Some(p)} else {None})
                    .collect::<Vec<usize>>()
                    .len() > 0 {return false};
            },
        }
        //If there is anyone still queuing in the lift's direction, the lift continues
        match self.next_floor_to_get_in(queues) {
            None => {
                //Do not change if queues is empty
                !vec2d_empty(queues)
            }
            Some(_) => false,
        }
    }
}

fn vec2d_empty(queues: &[Vec<usize>]) -> bool {
    queues.iter().all(|q| q.is_empty())
}

pub fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    let mut lift = Lift{
        capacity: capacity as usize,
        people: Vec::new(),
        current_floor: 0,
        direction: Direction::Up,
    };
    let mut queues2: Vec<Vec<usize>> = queues
        .iter()
        .map(|q| q.iter().map(|&q2| q2 as usize).collect())
        .collect();
    let mut result = vec![0];
    let mut next_floor: usize = 0;

    while lift.current_floor != 0 || lift.people.len() != 0 || !vec2d_empty(&queues2) {
        lift.process_floor(&mut queues2[next_floor]);
        if lift.is_change_direction(&queues2) {
            lift.change_direction();
            lift.process_floor(&mut queues2[next_floor]);
        }
        next_floor = lift.next_floor(&queues2);
        if next_floor == 0 && lift.current_floor == 0 {break};
        lift.current_floor = next_floor;
        result.push(next_floor as u32);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::the_lift;
    
    fn print_queues(queues: &[Vec<u32>], capacity: u32) -> String {
        let mut result = format!("\nLift capacity = {capacity}\n\n Floor    Queue");
        for (i, q) in queues.iter().enumerate().rev() {
            result.push_str(&format!("\n{i:>4} .... {q:?}"));
        }
        result
    }

    fn do_test(queues: &[Vec<u32>], capacity: u32, expected: &[u32]) {
        let actual = the_lift(queues, capacity);
        assert_eq!(actual, expected,
            "\nYour result (left) did not match expected output (right) for the given queues:\n{}\n",
            print_queues(queues, capacity));
    }

    #[test]
    fn test_up() {
        do_test(&[vec![], vec![], vec![5,5,5],vec![],vec![],vec![],vec![]], 5, &[0, 2, 5, 0]);
    }
    #[test]
    fn test_down() {
        do_test(&[vec![],vec![],vec![1],vec![],vec![],vec![],vec![]], 5, &[0, 2, 1, 0]);
    }
    #[test]
    fn test_up_and_up() {
        do_test(&[vec![],vec![3],vec![4],vec![],vec![5],vec![],vec![]], 5, &[0, 1, 2, 3, 4, 5, 0]);
    }
    #[test]
    fn test_down_and_down() {
        do_test(&[vec![],vec![0],vec![],vec![],vec![2],vec![3],vec![]], 5, &[0, 5, 4, 3, 2, 1, 0]);
    }
}

