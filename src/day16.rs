use std::collections::{HashMap,HashSet};

#[derive(PartialEq,Debug,Clone)]
struct Pair
{
    name: String,
    a:i32,
    b:i32,
    c:i32,
    d:i32,
}

impl Pair
{
    fn new(s:String)->Self
    {
        let left_right:Vec<_> = s.split(": ").collect();
        let ss: Vec<_> = left_right[1].split(" or ").collect();
        let v1: Vec<_> = ss[0].split("-").collect();
        let v2: Vec<_> = ss[1].split("-").collect();
        let name = left_right[0].to_string();
        let a:i32 = v1[0].parse().unwrap();
        let b:i32 = v1[1].parse().unwrap();
        let c:i32 = v2[0].parse().unwrap();
        let d:i32 = v2[1].parse().unwrap();

        Pair {
            name,
            a,
            b,
            c,
            d,
        }
    }

    fn in_range(&self,i:i32)->bool
    {
        (i>=self.a && i<=self.b) || (i>=self.c && i<=self.d)
    }
}

fn invalid(i:i32,limits:&Vec<Pair>)->bool
{
    for l in limits {
        if l.in_range(i) { return false; }
    }

    true
}

fn cut_or_one(i:usize,t:usize)->usize
{
    if i>=t { 1 } else { i }    
}

fn get_elem(hash:&HashMap <String,usize>,key:&str)->usize
{
    if hash.get(key)!=None {
        hash[key]
    }
    else 
    {
        1
    }
}

fn solve12(data:&Vec<String>)->(i64,i64)
{
    let mut limits = vec![];
    let mut valid = vec![];
    let mut your_ticket:Vec<i32> = vec![];
    let mut your = false;
    let mut scan = false;
    let mut res1:i64 =0;

    for line in data {
        if line.len()==0 { continue; }

        if scan {
            let l:Vec<_> = line.split(",").collect();
            let ticket:Vec<_> = l.iter().map(|&x|x.parse::<i32>().unwrap()).collect();
            let mut v=0;

            let mut ok = true;
            for &t in ticket.iter()
            {
                if invalid(t, &limits)
                {
                    ok = false;
                    v+=t;
                }
            }

            if ok { valid.push(ticket); }
            res1+=v as i64;
        }
        else if your {
            your = false;
            your_ticket = line.split(",").map(|x|x.parse::<i32>().unwrap()).collect();
            valid.push(your_ticket.clone());
        }
        else if line.find("your ticket:")!=None
        {
            your = true;
        }
        if !scan && line.find("nearby tickets:")!=None
        {            
            scan = true;
        }
        else if line.find(" or ")!=None
        {
            limits.push(Pair::new(line.clone()));
        }
    }

    let mut hash : HashMap <String,usize> = HashMap::new();
    let mut used : Vec<(usize,HashSet<String>)> = vec![];

    for x in 0..valid[0].len() {
        let mut set: HashSet<String> = HashSet::new();
        for l in limits.clone()
        {
            let mut c=0;
            for y in 0..valid.len() {
                if l.in_range(valid[y][x]) { c+=1; }
            }

            if c==valid.len() {
                set.insert(l.name.clone());
            }
        }
        used.push((x,set));
    }

    used.sort_by(|a, b| (&a.1.len()).cmp(&b.1.len()) );
    let mut name_used : HashSet<String> = HashSet::new();

    for (_,has) in used.iter().enumerate()
    {
        if has.1.len()>=1
        {
            for s in has.1.iter() {
                if !name_used.contains(s) {
                    name_used.insert(s.clone());
                    hash.insert(s.clone(),has.0);
                    break;
                }
            }
        }
    }

    let f1 = cut_or_one(get_elem(&hash,"departure location") ,your_ticket.len());
    let f2 = cut_or_one(get_elem(&hash,"departure station"),your_ticket.len());
    let f3 = cut_or_one(get_elem(&hash,"departure platform"),your_ticket.len());
    let f4 = cut_or_one(get_elem(&hash,"departure track"),your_ticket.len());
    let f5 = cut_or_one(get_elem(&hash,"departure date"),your_ticket.len());
    let f6 = cut_or_one(get_elem(&hash,"departure time"),your_ticket.len());

    let res2 =  your_ticket[f1] as i64*
                    your_ticket[f2] as i64*
                    your_ticket[f3] as i64*
                    your_ticket[f4] as i64*
                    your_ticket[f5] as i64*
                    your_ticket[f6] as i64;

    (res1,res2)    
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i64,i64)
{
    let res = (solve12(data));

    println!("Day16");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}




#[test]
fn test0()  { 
    let v = vec![
        "class: 1-3 or 5-7".to_string(),
        "row: 6-11 or 33-44".to_string(),
        "seat: 13-40 or 45-50".to_string(),
        "".to_string(),
        "your ticket:".to_string(),
        "7,1,14".to_string(),
        "".to_string(),
        "nearby tickets:".to_string(),
        "7,3,47".to_string(),
        "40,4,50".to_string(),
        "55,2,20".to_string(),
        "38,6,12".to_string(),
        ];      
        assert_eq!(solve12(&v).0,71);    
}
