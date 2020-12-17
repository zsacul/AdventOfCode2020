use std::collections::HashMap;

fn count3d(hash:&HashMap<(i32,i32,i32),bool>,x:i32,y:i32,z:i32)->i32
{
    let mut cnt=0;
    
    for zz in -1..=1 {
    for yy in -1..=1 {
    for xx in -1..=1 {
        if xx!=0 || yy!=0 || zz!=0 
        {
            if *hash.get(&(x+xx,y+yy,z+zz)).unwrap_or(&false)
            {
                cnt+=1;
            }
        }
    }}}

    cnt
}

fn simulate3d(prev:&HashMap<(i32,i32,i32),bool>,next:&mut HashMap<(i32,i32,i32),bool>)
{
    for e in prev.clone() {
        let (px,py,pz) = e.0;
        for zz in -1..=1 {
        for yy in -1..=1 {
        for xx in -1..=1 {
            let newp = (px+xx,py+yy,pz+zz);
            let c = count3d(prev,newp.0,newp.1,newp.2);
            next.insert(newp, false);
            
            if *prev.get(&newp).unwrap_or(&false)
            {
                if c==2 || c==3 { next.insert(newp, true);  }                                  
            }
                else
            {
                if c==3 { next.insert(newp, true); }
            }
        }}}
    }
}

fn count4d(hash:&HashMap<(i32,i32,i32,i32),bool>,x:i32,y:i32,z:i32,w:i32)->i32
{
    let mut cnt=0;
    
    for ww in -1..=1 {
    for zz in -1..=1 {
    for yy in -1..=1 {
    for xx in -1..=1 {
        if xx!=0 || yy!=0 || zz!=0 || ww!=0
        {
            if *hash.get(&(x+xx,y+yy,z+zz,w+ww)).unwrap_or(&false)
            {
                cnt+=1;
            }
        }
    }}}}

    cnt
}

fn simulate4d(prev:&HashMap<(i32,i32,i32,i32),bool>,next:&mut HashMap<(i32,i32,i32,i32),bool>)
{
    for e in prev.clone() {
        let (px,py,pz,pw) = e.0;

        for ww in -1..=1 {
        for zz in -1..=1 {
        for yy in -1..=1 {
        for xx in -1..=1 {
            let newp = (px+xx,py+yy,pz+zz,pw+ww);
            let c = count4d(prev,newp.0,newp.1,newp.2,newp.3);
            next.insert(newp, false);
            
            if *prev.get(&newp).unwrap_or(&false)
            {
                if c==2 || c==3 { next.insert(newp, true);  }                                  
            }
              else
            {
                if c==3 { next.insert(newp, true); }
            }
        }}}}
    }
}

pub fn solve1(data:&Vec<String>)->i32
{
    let mut next:HashMap<(i32,i32,i32),bool> = HashMap::new();

    for (y,l) in data.iter().enumerate() {
    for (x,c) in l.chars().enumerate()
    {            
        if c=='#' { next.insert((x as i32,y as i32,0),true); }        
    }}

    for _ in 0..6
    {
        simulate3d(&next.clone(),&mut next);        
        next = next.into_iter().filter(|x| x.1==true).collect();
    }    

    next.len() as i32
}

pub fn solve2(data:&Vec<String>)->i32
{
    let mut next:HashMap<(i32,i32,i32,i32),bool> = HashMap::new();
   
    for (y,l) in data.iter().enumerate() {
    for (x,c) in l.chars().enumerate()
    {            
        if c=='#' { next.insert((x as i32,y as i32,0,0),true); }        
    }}

    for _ in 0..6
    {
        simulate4d(&next.clone(),&mut next);    
        next = next.into_iter().filter(|x| x.1==true).collect();
    }

    next.len() as i32 
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i32,i32)
{
    let res = (solve1(data),solve2(data));

    println!("Day17");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        ".#.".to_string(),
        "..#".to_string(),
        "###".to_string(),
    ];
    assert_eq!(solve(&v).0,112);
}


#[test]
fn test2()
{
    let v = vec![
        ".#.".to_string(),
        "..#".to_string(),
        "###".to_string(),
        ];    
    assert_eq!(solve(&v).1,848);
}
