use super::cycliclist::{CyclicList};
use std::io::{self, Write};

pub fn solve1(data:&[String],moves:usize)->String
{
    let mut table = CyclicList::new();
    let mut max_label = 0;

    for c in data[0].chars() {
        let v = (c as u8-b'0') as i32;
        table.push_right(v);
        if v>max_label {
            max_label = v;
        }
    }
    table.right();
    const SHOW_DEBUG : bool = false;

    for _m in 1..=moves {

        if SHOW_DEBUG 
        {
            table.print();
        }
        
        let curr_val = table.peek().unwrap();
        table.right();
        let p1 = table.pop().unwrap(); table.right();
        let p2 = table.pop().unwrap(); table.right();
        let p3 = table.pop().unwrap(); table.right();

        let mut dest_v = curr_val-1;
        if dest_v<1 { dest_v = max_label; }

        while dest_v==p1 || dest_v==p2 || dest_v==p3
        {
            dest_v-=1;
            if dest_v<1 { dest_v = max_label; }
        }      

        table.move_right_till_value(dest_v);
        
        table.push_right(p1);
        table.push_right(p2);
        table.push_right(p3);

        table.move_right_till_value(curr_val);
        table.right();
    }
    
    let mut res ="".to_string();

    table.move_right_till_value(1);
    table.right();
     
    while table.peek().unwrap()!=1 
    {      
        res.push_str(&table.peek().unwrap().to_string()[..]);
        table.right();
    }

    res
}


pub fn solve2(data:&[String],moves:usize)->i64
{
    let mut table = CyclicList::new();
    let mut max_label = 0;

    for c in data[0].chars() {
        let v = (c as u8-b'0') as i32;
        table.push_right(v);
        if v>max_label {
            max_label = v;
        }
    }

    while table.len()<1000000
    {
        max_label+=1;
        table.push_right(max_label);    
    }    
    table.right();

    for m in 1..=moves {

        if m%100_000==0 
        {
            print!("{}..{}%\r",m,(m*100)/moves);            
            io::stdout().flush().unwrap();
        }
        
        let curr_val = table.peek().unwrap();
                                           table.right();
        let p1 = table.pop().unwrap(); table.right();
        let p2 = table.pop().unwrap(); table.right();
        let p3 = table.pop().unwrap(); table.right();

        let mut dest_v = curr_val-1;
        if dest_v<1 { dest_v = max_label; }

        while dest_v==p1 || dest_v==p2 || dest_v==p3
        {
            dest_v-=1;
            if dest_v<1 { dest_v = max_label; }
        }

        table.move_left_till_value(dest_v);

        table.push_right(p1);
        table.push_right(p2);
        table.push_right(p3);

        table.move_right_till_value(curr_val);
        table.right();
    }

    table.move_right_till_value(1);
    table.right();
    let pos1 = table.peek().unwrap() as i64;
    table.right();
    let pos2 = table.peek().unwrap() as i64;

    pos1*pos2
}

pub fn solve(data:&[String])->(String,i64)
{
    let res = (solve1(data,100),solve2(data,10_000_000));

    println!("Day23            ");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec!["389125467".to_string()];
    assert_eq!(solve1(&v,10),"92658374");
}

#[test]
fn test2()
{
    let v = vec!["389125467".to_string()];
    assert_eq!(solve1(&v,100),"67384529");
}

//#[ignore = "too slow"]
#[test]
fn test3()
{
    let v = vec!["389125467".to_string()];
    assert_eq!(solve2(&v,10000000),149245887792);
}
