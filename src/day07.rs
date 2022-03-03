use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Bag {
    contain : HashMap<String,usize>,
    color   : String,
}

impl Bag{
    fn new()->Self
    {
        Self 
        {
            contain : HashMap::new(),
            color   : "".to_string(),
        }
    }

    fn parse(&mut self,s:String)
    {
        let v: Vec<_> = s.split(" bags contain ").collect();
        self.color = v[0].to_string();
        let bags: Vec<_> = v[1].split(", ").collect();

        if !bags.is_empty()
        {
            for b in bags {
                let nums  : Vec<_> = b.split(' ').collect();            
                let count : usize  = nums[0].parse().unwrap_or(0);
    
                let n    = b[1+b.find(' ').unwrap()..].to_string();
                let name = n.to_string().replace('.', "").replace(" bags", "").replace(" bag", "");
                self.contain.insert(name,count);
            }
        }
    }

    fn is_gold(&self)->bool 
    {
        self.color=="shiny gold"
    }
}

fn contain_gold(bags:&HashMap<String,Bag>,b:&Bag)->bool 
{  
    if b.is_gold() { return true; }

    let mut res = false;
    
    for key in b.contain.keys()
    {
        if bags.contains_key(key)
        {
            res|=contain_gold(bags,&bags[key]);
        }
    }
    res
}

fn count_cost(bags:&HashMap<String,Bag>,b:&Bag)->usize {
    let mut res = 0;   

    for (key,count) in &b.contain 
    {
        if bags.contains_key(key)
        {
            res+= count*count_cost(bags,&bags[key]);
        }
    }    

    res+1
}

#[allow(unused)]
pub fn solve(data:&[String])->(usize,usize)
{
    let mut res = (0,0);
    let mut bags:HashMap<String,Bag> = HashMap::new();

    for line in data {
        let mut b = Bag::new();
        b.parse(line.to_string());
        bags.insert(b.color.clone(), b);
    }

    for k in bags.iter() {
        if k.0!="shiny gold" 
        { 
            if contain_gold(&bags,k.1) { res.0+=1; }
        }
        else
        {
            res.1+=count_cost(&bags,k.1); 
        }
    }
    res.1-=1;

    println!("Day7");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}

#[test]
fn test1()
{
    let v = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];
    assert_eq!(solve(&v).0,4);
}


#[test]
fn test2()
{
    let v = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];    
    assert_eq!(solve(&v).1,32);
}



#[test]
fn test3()
{
    let v = vec![
        "shiny gold bags contain 2 dark red bags.".to_string(),
        "dark red bags contain 2 dark orange bags.".to_string(),
        "dark orange bags contain 2 dark yellow bags.".to_string(),
        "dark yellow bags contain 2 dark green bags.".to_string(),
        "dark green bags contain 2 dark blue bags.".to_string(),
        "dark blue bags contain 2 dark violet bags.".to_string(),
        "dark violet bags contain no other bags.".to_string(),
    ];
    assert_eq!(solve(&v).1,126);
}


