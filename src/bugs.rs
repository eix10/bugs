
const PREY_BREED:u32 = 3;
const PREDATOR_BREED:u32 = 8;
const PREDATOR_STARVE:u32 = 3;

#[derive(Clone, Copy, PartialEq)]
pub enum BugType
{
    Empty,
    Prey,
    Predator
}

pub struct Neighbor
{
    pub index: usize,
    pub bug_type: BugType
}

pub struct Update
{
    pub move_to: i32,
    pub new_bug_idx: i32,
    pub bug_type: BugType
}

impl Update
{
    pub fn new(move_to: i32) -> Self
    {
        return Update { move_to: move_to, new_bug_idx: -1, bug_type: BugType::Empty };
    }
}

impl Neighbor
{
    pub fn new(index: usize, bug_type: BugType) -> Self
    {
        return Neighbor { index: index, bug_type: bug_type };
    }
}

impl Copy for Neighbor { }
impl Clone for Neighbor
{
    fn clone(&self) -> Neighbor
    {
        *self
    }
}

pub struct Bug
{
    age: u32, 
    bug_type: BugType,
    breed: u32,
    starve: u32,
}

impl Bug
{
    pub fn new(bug_type: BugType) -> Self
    {
        Bug { age: 0, bug_type: bug_type, breed: 0, starve: 0 }
    }

    pub fn get_bug(self) -> char
    {
        match self.bug_type
        {
            BugType::Empty => ' ',
            BugType::Prey => '*',
            BugType::Predator => '#'
        }
    }

    pub fn get_bug_type(self) -> BugType
    {
        return self.bug_type;
    }

    pub fn live(&mut self, index: usize, neighbors: &mut Vec<Neighbor>) -> Update
    {
        match self.bug_type
        {
            BugType::Empty => Update::new(index as i32),
            BugType::Prey => self.live_prey(index, neighbors),
            BugType::Predator => self.live_predator(index, neighbors),
        }
    }

    fn attempt_breed(&mut self, update: &mut Update, neighbors: &mut Vec<Neighbor>, breed_time: u32)
    {
        if self.breed == breed_time 
        {
            while neighbors.len() > 0
            {
                let neighbor = neighbors.pop().unwrap();
                if neighbor.bug_type == BugType::Empty
                {
                    update.new_bug_idx = neighbor.index as i32;
                    update.bug_type = self.bug_type;
                    self.breed = 0;
                    break;
                }
            }
        } else {
            self.breed += 1;
        }
    }

    fn attempt_move(self,update: &mut Update, neighbors: &mut Vec<Neighbor>)
    {
        while neighbors.len() > 0
        {
            let neighbor = neighbors.pop().unwrap();
            if neighbor.bug_type == BugType::Empty
            {
                update.move_to = neighbor.index as i32;
                    break;
            }
        }
    }

    fn live_predator(&mut self, index: usize, neighbors: &mut Vec<Neighbor>) -> Update
    {
        let mut update = Update::new(index as i32);

        let mut n = neighbors.clone();
        
        self.starve += 1;

        while n.len() > 0
        {
            let neighbor = n.pop().unwrap();
            if neighbor.bug_type == BugType::Prey
            {
                // Feed and move 
                self.starve = 0;
                update.move_to = neighbor.index as i32;

                if self.breed == PREDATOR_BREED
                {
                    update.new_bug_idx = index as i32;
                    update.bug_type = BugType::Predator;
                } else {
                    self.breed += 1;
                }

                break;
            } 
        }

        if self.starve == PREDATOR_STARVE
        {
            update.move_to = -1;
            update.new_bug_idx = index as i32;
            update.bug_type = BugType::Empty;
            return update;
        }

        if self.starve > 0
        {
            // Then we try to breed
            self.attempt_breed(&mut update, neighbors, PREDATOR_BREED);

            // Try to move 
            self.attempt_move(&mut update, neighbors);
        }

        return update;
    }

    fn live_prey(&mut self, index: usize, neighbors: &mut Vec<Neighbor>) -> Update
    {
        let mut update = Update::new(index as i32);

        //First we breed
        self.attempt_breed(&mut update, neighbors, PREY_BREED);

        // Try to move 
        self.attempt_move(&mut update, neighbors);


        return update;
    }
}

impl Copy for Bug { }
impl Clone for Bug
{
    fn clone(&self) -> Bug
    {
        *self
    }
}
