use std::fmt::Debug;

fn find_id(tab:&Vec<u8>,val:u8)->usize
{
    for (id,&label) in tab.iter().enumerate() {
        if val==label { return id; }
    }
    0
}

fn print_info<T:Debug>(m:usize,table:&Vec<T>,current:usize)
{
    println!("-- move {} --",m);
    println!("cups: ");

    for (i,v) in table.iter().enumerate() {
        if i==current { print!("({:?}) ",v); }
                 else { print!("{:?} ",v);   }
    }
    println!("");

}

pub fn solve1(data:&Vec<String>,moves:usize)->String
{
    let mut table = vec![];
    let mut current = 0;

    for c in data[0].chars() {
        table.push(c as u8-b'0');
    }
    let max_label = *table.iter().max().unwrap();
    let n = table.len();

    const SHOW_DEBUG : bool = false;

    for m in 1..=moves {

        if SHOW_DEBUG 
        {
            print_info::<u8>(m,&table,current);
        }
        
        let curr_val = table[current];
        let pic = vec![(current+1)%n,(current+2)%n,(current+3)%n];
        let picked = vec![table[pic[0]],table[pic[1]],table[pic[2]]];


        if SHOW_DEBUG
        {
            println!("pick up: {}, {}, {}",picked[0],picked[1],picked[2]);
        }

        let mut rem = pic.clone();
        rem.sort();

        let mut dest_v = curr_val-1;
        if dest_v<1 { dest_v = max_label; }

        while picked.contains(&dest_v)
        {
            dest_v-=1;
            if dest_v<1 { dest_v = max_label; }
        }
        table.remove(rem[2]);
        table.remove(rem[1]);
        table.remove(rem[0]);
        
        
        let dest = find_id(&table,dest_v);

        if SHOW_DEBUG
        {
            println!("destination: {}->{}",dest_v,dest);
        }        

        let nn = table.len()+1;
        table.insert((dest+1)%nn, picked[2]);
        table.insert((dest+1)%nn, picked[1]);
        table.insert((dest+1)%nn, picked[0]);
      
        current = find_id(&table,curr_val);
        current = (current+1)%n;
    }

    let mut pos = find_id(&table,1);
    let mut res ="".to_string();
    
    pos= (pos+1)%n;
    
    while table[pos]!=1 
    {      
        res.push_str(&table[pos].to_string()[..]);
        pos = (pos+1)%n;
    }

    res
}

fn find_idb(tab:&Vec<i32>,val:i32)->usize
{
    for (id,&label) in tab.iter().enumerate() {
        if val==label { return id; }
    }
    0
}


pub fn solve2(data:&Vec<String>,moves:usize)->i64
{
    let mut table = vec![];
    let mut current = 0;

    for c in data[0].chars() {
        table.push(c as i32-b'0' as i32);
    }
    let mut mmm = *table.iter().max().unwrap();

    while table.len()<1000000
    {
        mmm+=1;
        table.push(mmm);    
    }    

    let  max_label = *table.iter().max().unwrap();
    let n = table.len();

    const SHOW_DEBUG:bool = false;

    for m in 1..=moves {

        if m%10000 ==0 
        {
            println!("{}..{}%",m,(m*100)/moves)
        }

        if SHOW_DEBUG
        {
            print_info::<i32>(m,&table,current);
        }
        
        let curr_val = table[current];
        let pic = vec![(current+1)%n,(current+2)%n,(current+3)%n];
        let picked = vec![table[pic[0]],table[pic[1]],table[pic[2]]];

        if SHOW_DEBUG
        {
            println!("pick up: {}, {}, {}",picked[0],picked[1],picked[2]);
        }

        let mut rem = pic.clone();
        rem.sort();

        let mut dest_v = curr_val-1;
        if dest_v<1 { dest_v = max_label; }

        while picked.contains(&dest_v)
        {
            dest_v-=1;
            if dest_v<1 { dest_v = max_label; }
        }
        table.remove(rem[2]);
        table.remove(rem[1]);
        table.remove(rem[0]);
        
        
        let dest = find_idb(&table,dest_v);

        if SHOW_DEBUG
        {
            println!("destination: {}->{}",dest_v,dest);
        }        

        let nn = table.len()+1;
        table.insert((dest+1)%nn, picked[2]);
        table.insert((dest+1)%nn, picked[1]);
        table.insert((dest+1)%nn, picked[0]);
      
        if curr_val!=table[current] { current = find_idb(&table,curr_val); }
        current = (current+1)%n;
    }

    let pos = find_idb(&table,1);
    let pos1 = (pos+1)%n;
    let pos2 = (pos+2)%n;
    
    //println!("res1={}",table[pos1]);
    //println!("res2={}",table[pos2]);
    //println!("res={}",(table[pos1] as i64)*(table[pos2] as i64));
    //res1=385778
    //res2=931123

    (table[pos1] as i64)*(table[pos2] as i64)
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(String,i64)
{
    let res = (solve1(data,100),solve2(data,10_000_000));

    println!("Day23");
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

#[test]
fn test3()
{
    let v = vec!["389125467".to_string()];
    assert_eq!(solve2(&v,10000000),149245887792);
}
