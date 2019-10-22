mod container;

use container::ExpandingVec;

type Token = char;
type State = char;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MoveHead {
    LEFT, NOT, RIGHT
}

impl MoveHead {
    fn head_offset(&self) -> i64 {
        return match self {
            MoveHead::LEFT => -1,
            MoveHead::NOT => 0,
            MoveHead::RIGHT => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Transition {
    curr_token: Token,
    curr_state: State,
    next_token: Token,
    next_state: State,
    move_head: MoveHead,
}

impl Transition {
    fn new(curr_token: Token, curr_state: State, next_token: Token, next_state: State, move_head: MoveHead) -> Self {
        Transition {
            curr_token,
            curr_state,
            next_token,
            next_state,
            move_head
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TouringMachine {
    input_alphabet: Vec<Token>,
    states: Vec<State>,
    work_alphabet: Vec<Token>,
    transitions: Vec<Transition>,
    start_state: State,
    empty_symbol: Token,
}


#[derive(Debug)]
struct TouringState<'a> {
    machine: &'a TouringMachine,
    input: ExpandingVec,
    head_position: i64,
    current_state: State,
}

impl TouringState<'_> {
    fn new(machine: &TouringMachine, input: Vec<Token>, head_position: i64, current_state: State) -> TouringState {
        TouringState {
            machine,
            input: ExpandingVec::new(input, machine.empty_symbol),
            head_position,
            current_state
        }
    }

    fn current_token(&mut self) -> &Token {
        self.input.get(self.head_position)
    }

    fn apply_transition(&mut self, transition: &Transition) {
        *self.input.get(self.head_position) = transition.next_token;
        self.head_position += transition.move_head.head_offset();
        self.input.ensure_available(self.head_position); // ensure that new head position has actual value
        self.current_state = transition.next_state;
    }

    fn exec_step(&mut self) -> bool {
        let last_head_pos = self.head_position;
        let last_token = self.current_token().clone();
        let last_state = self.current_state;
        let matching_transition = self.machine.transitions.iter()
            .filter(|t| t.curr_token == last_token && t.curr_state == self.current_state)
            .next();
        match matching_transition {
            Some(transition) => {
                println!("rule matched: {:?}", transition);
                self.apply_transition(transition);
                last_state != self.current_state || last_head_pos != self.head_position
            },
            None => {
                println!("No matching transition found");
                false
            },
        }
    }

    fn pretty_print(&self) {
        println!("Current State: {}", self.current_state);

        //println!("head_pos = {}", self.head_position);
        //println!("first_index = {}", self.input.first_index());


        for c in self.input.iter() {
            print!("{}", c);
        }
        println!();

        print!("{}", " ".repeat((self.head_position - self.input.first_index()) as usize));
        println!("^")
    }

    fn run(&mut self) {
        self.pretty_print();
        loop {
            let did_changed = self.exec_step();
            self.pretty_print();
            if !did_changed {
                println!("No change occurred stop processing!");
                break;
            }
        }
    }
}

#[test]
fn test_simple_machine() {
    let machine = TouringMachine {
        input_alphabet: vec!['0', '1'],
        states: vec!['a', 'b'],
        work_alphabet: vec!['0', '1'],
        transitions: vec![
            Transition {
                curr_token: '0',
                curr_state: 'a',
                next_token: '0',
                next_state: 'a',
                move_head: MoveHead::RIGHT
            },
            Transition {
                curr_token: '1',
                curr_state: 'a',
                next_token: '1',
                next_state: 'b',
                move_head: MoveHead::NOT
            }
        ],
        start_state: 'a',
        empty_symbol: 'x'
    };
    let mut state = TouringState::new(
        &machine,
        vec!['0', '0', '0', '1', '0'],
        0,
        machine.start_state
    );
    state.run();
    assert_eq!('b', state.current_state);
    assert_eq!('1', *state.current_token());
}

fn main() {
    /*let machine = TouringMachine {
        input_alphabet: vec![],
        states: vec![],
        work_alphabet: vec![],
        transitions: vec![
            Transition::new( '1', '1', 'x', '2', MoveHead::RIGHT)
            , Transition::new( 'x', '1', 'x', '1', MoveHead::RIGHT)
            , Transition::new( '1', '2', '1', '3', MoveHead::RIGHT)
            , Transition::new( '1', '3', '1', '3', MoveHead::RIGHT)
            , Transition::new( '.', '3', '.', '4', MoveHead::RIGHT)
            , Transition::new( '1', '4', 'x', '5', MoveHead::RIGHT)
            , Transition::new( 'x', '4', 'x', '4', MoveHead::RIGHT)
            , Transition::new( '1', '5', '1', '6', MoveHead::RIGHT)
            , Transition::new( '1', '6', '1', '6', MoveHead::RIGHT)
            , Transition::new( '.', '6', '.', '7', MoveHead::RIGHT)
            , Transition::new( '1', '7', '1', '7', MoveHead::RIGHT)
            , Transition::new( '#', '7', '1', '8', MoveHead::NOT)
            , Transition::new( '1', '8', '1', '8', MoveHead::LEFT)
            , Transition::new( 'x', '8', 'x', '8', MoveHead::LEFT)
            , Transition::new( '.', '8', '.', '8', MoveHead::LEFT)
            , Transition::new( '#', '8', '#', '1', MoveHead::RIGHT)
            , Transition::new( '.', '2', '.', 'A', MoveHead::NOT)
            , Transition::new( '.', '5', '.', 'A', MoveHead::NOT)
            , Transition::new( '.', 'A', '.', 'A', MoveHead::LEFT)
            , Transition::new( 'x', 'A', 'x', 'A', MoveHead::LEFT)
            , Transition::new( '1', 'A', '1', 'A', MoveHead::LEFT)
            , Transition::new( '#', 'A', '#', 'B', MoveHead::RIGHT)
            , Transition::new( '1', 'B', '#', 'B', MoveHead::RIGHT)
            , Transition::new( 'x', 'B', '#', 'B', MoveHead::RIGHT)
            , Transition::new( '.', 'B', '#', 'C', MoveHead::RIGHT)
            , Transition::new( '1', 'C', '#', 'C', MoveHead::RIGHT)
            , Transition::new( 'x', 'C', '#', 'C', MoveHead::RIGHT)
            , Transition::new( '.', 'C', '#', 'D', MoveHead::RIGHT)
            , Transition::new( '1', 'D', '1', 'D', MoveHead::RIGHT)
            , Transition::new( '#', 'D', '1', 'E', MoveHead::RIGHT)
            , Transition::new( '#', 'E', '1', 'F', MoveHead::RIGHT)
            , Transition::new( '#', 'F', '1', 'G', MoveHead::RIGHT)
        ],
        start_state: '1',
        empty_symbol: '#'
    };*/
    let L = MoveHead::LEFT;
    let R = MoveHead::RIGHT;
    let N = MoveHead::NOT;
    let machine = TouringMachine {
        input_alphabet: vec![],
        states: vec![],
        work_alphabet: vec![],
        transitions: vec![
            Transition::new( '0', 'a', '0', 'c',L),
            Transition::new( '1', 'a', '1', 'c',L),
            Transition::new( '#', 'a', '1', 'd',R),
            Transition::new( '#', 'c', '1', 'd',R),
            Transition::new( '0', 'd', '0', 'd',R),
            Transition::new( '1', 'd', '1', 'd',R),
            Transition::new( '#', 'd', '#', 'e',L),
            Transition::new( '0', 'e', '0', 'f',L),
            Transition::new( '1', 'e', '1', 'f',L),
            Transition::new( '#', 'e', '#', 'b',N ),
            Transition::new( '0', 'f', '#', 'f',L),
            Transition::new( '1', 'f', '#', 'f',L),
            Transition::new( '#', 'f', '#', 'b',N ),
            Transition::new( '#', 'b', '#', 'g',R),
            Transition::new( '#', 'g', '#', 'g',R),
            Transition::new( '1', 'g', '1', 'h',N )
        ],
        start_state: 'a',
        empty_symbol: '#'
    };
    let mut state = TouringState::new(&machine,
                                      "".chars().collect(),
                                      0, machine.start_state
    );
    state.run();
}