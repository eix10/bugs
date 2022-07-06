//World
use std::process::Command;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::bugs::Bug;
use crate::bugs::BugType;
use crate::bugs::Neighbor;

const ROW:usize = 5;
const COL:usize = 5;

pub struct World
{
    grid: [[Bug; ROW]; COL],
}

impl World 
{
    pub fn new() -> World
    {
        return World 
        { 
            grid: [[Bug::new(BugType::Empty); ROW]; COL],
        };
    }

    fn clear()
    {
        let mut clear = Command::new("clear");
        clear.status();
        //println!("\033[2J\033[1:1H");
    }

    fn gotoxy(x: u8, y: u8)
    {
        println!("{}[{};{}f", 0x1B as char, y, x);
    }

    pub fn cycle(&mut self)
    {
        let mut skip:Vec<i32> = Vec::new();
        for r in 0..ROW
        {
            for c in 0..COL
            {
                let index = r*ROW+c;

                if skip.last() == Some(&(index as i32))
                {
                    skip.pop();
                    continue;
                }

                let mut neighbors:Vec<Neighbor> = Vec::new();
                if r > 0 { neighbors.push(Neighbor::new((r-1)*ROW + c, self.grid[r-1][c].get_bug_type())); }
                if r < ROW-1 { neighbors.push(Neighbor::new((r+1)*ROW + c, self.grid[r+1][c].get_bug_type())); }
                if c > 0 { neighbors.push(Neighbor::new(r*ROW + c-1, self.grid[r][c-1].get_bug_type())); }
                if c < COL-1 { neighbors.push(Neighbor::new(r*ROW + c+1, self.grid[r][c+1].get_bug_type())); }

                let update = self.grid[r][c].live(index,  &mut neighbors);
                
                if update.new_bug_idx >= 0
                {
                    self.update_index(update.new_bug_idx as usize, Bug::new(update.bug_type));
                    if update.new_bug_idx > index as i32
                    {
                        skip.insert(0, update.new_bug_idx);
                    }
                }

                if update.move_to >= 0 && update.move_to != index as i32
                {
                    self.update_index(update.move_to as usize, self.grid[r][c]);
                    self.update_index(index, Bug::new(BugType::Empty));
                    if update.move_to > index as i32
                    {
                        skip.insert(0, update.move_to);
                    }
                }
                if update.move_to != -1 || update.new_bug_idx != -1
                {

                //println!("Index: {} Move: {} New: {} -> {}", index, update.move_to, update.new_bug_idx, Bug::new(update.bug_type).get_bug());
                }
            }
        }
    }
    

    pub fn print(&self)
    {
//        World::clear();
//        World::gotoxy(0,0);
        print!(" - - - - -\n");
        for r in self.grid
        {
            print!("|");
            for c in r
            {
                print!("{}|", c.get_bug());
            }
            print!("\n - - - - -\n");
        }
    }

    pub fn update_index(&mut self, index:usize, bug:Bug)
    {
        let row = index / ROW;
        let col = index - row*ROW;


        self.grid[row][col] = bug;
    }

}
