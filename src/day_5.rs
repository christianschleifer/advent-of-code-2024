use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Page {
    id: PageNumber,
    dependent_pages: HashSet<PageNumber>,
}

impl Page {
    pub fn new(id: PageNumber, dependent_pages: HashSet<PageNumber>) -> Self {
        Self {
            id,
            dependent_pages,
        }
    }
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: HashMap<PageNumber, Page>,
}

impl Graph {}

impl Graph {
    fn new(orderings: Vec<Ordering>) -> Graph {
        let mut nodes = HashMap::with_capacity(orderings.len());

        for ordering in orderings {
            nodes
                .entry(ordering.left)
                .or_insert(Page::new(ordering.left, HashSet::new()))
                .dependent_pages
                .insert(ordering.right);

            nodes
                .entry(ordering.right)
                .or_insert(Page::new(ordering.right, HashSet::new()));
        }

        Graph { nodes }
    }

    fn path_exists(&self, source: PageNumber, target: PageNumber, path: &[PageNumber]) -> bool {
        let page = if let Some(page) = self.nodes.get(&source) {
            page
        } else {
            return false;
        };

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(page.clone());

        while let Some(page) = queue.pop_front() {
            if page.id == target {
                return true;
            }

            visited.insert(page.id);

            for prerequisite_page in page.dependent_pages {
                if !visited.contains(&prerequisite_page) && path.contains(&prerequisite_page) {
                    if let Some(page) = self.nodes.get(&prerequisite_page).cloned() {
                        queue.push_back(page.clone())
                    }
                }
            }
        }

        false
    }

    fn is_valid_ordering(&self, page_numbers: &[PageNumber]) -> bool {
        for i in (1..page_numbers.len()).rev() {
            let page_number = page_numbers[i];
            for target_page_number in page_numbers[..i].iter().rev() {
                if self.path_exists(page_number, *target_page_number, &page_numbers[..i]) {
                    return false;
                }
            }
        }

        true
    }

    fn order(&self, mut page_numbers: Vec<PageNumber>) -> Vec<PageNumber> {
        'check_loop: loop {
            for i in (1..page_numbers.len()).rev() {
                let page_number = page_numbers[i];
                for target_i in (0..i).rev() {
                    let target_page_number = page_numbers[target_i];
                    if self.path_exists(page_number, target_page_number, &page_numbers[..i]) {
                        page_numbers.swap(i, target_i);
                        continue 'check_loop;
                    }
                }
            }

            break 'check_loop;
        }

        page_numbers
    }
}

pub fn solve() {
    let input = get_input();
    solve_puzzle_1(input.clone());
    solve_puzzle_2(input);
}

fn solve_puzzle_1(input: Input) {
    let dependency_graph = Graph::new(input.orderings.clone());

    let mut sum_of_valid_middle = 0;

    for update in &input.updates {
        if dependency_graph.is_valid_ordering(update) {
            let middle_page_number = update
                .get(update.len() / 2)
                .expect("could not get middle page number");

            sum_of_valid_middle += middle_page_number;
        }
    }

    println!(
        "Puzzle 1: Sum of valid ordering middle page numbers: {}",
        sum_of_valid_middle
    );
}

fn solve_puzzle_2(input: Input) {
    let dependency_graph = Graph::new(input.orderings.clone());

    let mut sum_of_reordered_middle = 0;

    for update in &input.updates {
        if !dependency_graph.is_valid_ordering(update) {
            let ordered_page_numbers = dependency_graph.order(update.clone());
            let middle_page_number = ordered_page_numbers
                .get(update.len() / 2)
                .expect("could not get middle page number");

            sum_of_reordered_middle += middle_page_number;
        }
    }

    println!(
        "Puzzle 2: Sum of reordered middle page numbers: {}",
        sum_of_reordered_middle
    );
}

type PageNumber = u32;

enum InputSections {
    OrderingRules,
    Updates,
}

#[derive(Debug, Clone)]
struct Ordering {
    left: PageNumber,
    right: PageNumber,
}

#[derive(Debug, Clone)]
struct Input {
    orderings: Vec<Ordering>,
    updates: Vec<Vec<PageNumber>>,
}

fn get_input() -> Input {
    let file = File::open("./src/resources/day_5.txt").expect("could not open file for day 5");
    let buf_reader = BufReader::new(file);

    let mut state = InputSections::OrderingRules;
    let mut orderings = Vec::new();
    let mut updates = Vec::new();

    for line in buf_reader.lines() {
        let line = line.expect("could not read line");
        if line.is_empty() {
            state = InputSections::Updates;
            continue;
        }

        match state {
            InputSections::OrderingRules => {
                let mut split = line.split('|');
                let left = split
                    .next()
                    .expect("invalid input format")
                    .parse::<u32>()
                    .expect("could not parse number");
                let right = split
                    .next()
                    .expect("invalid input format")
                    .parse::<u32>()
                    .expect("could not parse number");

                orderings.push(Ordering { left, right })
            }
            InputSections::Updates => {
                let page_numbers = line
                    .split(',')
                    .map(|str| str.parse::<u32>())
                    .map(|result| result.expect("could not parse number"))
                    .collect::<Vec<PageNumber>>();

                updates.push(page_numbers);
            }
        }
    }

    Input { orderings, updates }
}
