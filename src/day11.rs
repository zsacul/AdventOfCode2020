fn cnt(d:&[char],c:char)->i32
{    
    d.iter().map(|&letter| if letter==c {1} else {0}).sum()
}

fn get1(data:&mut Vec<Vec<char>>,x:i32,y:i32)->i32
{
    let dy = data.len() as i32;
    let dx = data[0].len() as i32;
    if x<0 || y<0 || x>=dx || y>=dy      { return 0; }
    if data[y as usize][x as usize]=='#' { return 1; } 
    0
}

fn get2(data:&mut Vec<Vec<char>>,x:i32,y:i32,delx:i32,dely:i32)->i32
{
    let dy = data.len() as i32;
    let dx = data[0].len() as i32;
    let mut xx = x+delx;
    let mut yy = y+dely;
    
    loop {
        if xx<0 || yy<0 || xx>=dx || yy>=dy    { return 0;}
        if data[yy as usize][xx as usize]=='#' { return 1; } 
        if data[yy as usize][xx as usize]=='L' { return 0; } 
        xx+=delx;
        yy+=dely;
    }
}

fn moving(next:&mut Vec<Vec<char>>,prev:&mut Vec<Vec<char>>,part1:bool)->bool
{
    let dy = prev.len();
    let dx = prev[0].len();

    for y in 0..dy {
    for x in 0..dx {

        next[y][x] = prev[y][x];

        if prev[y][x]!='.'
        {
            let mut n = 0;
            
            for yy in -1..=1 {   
            for xx in -1..=1 {   
                if xx!=0 || yy!=0 {
                    if part1 { n+=get1(prev,x as i32 + xx,y as i32 + yy);  }
                        else { n+=get2(prev,x as i32     ,y as i32      ,xx ,yy); }
                }
            }
            }

            let limit= if part1 { 4 } else { 5 };                

                 if prev[y][x]=='L' && n==0     { next[y][x] = '#'; }
            else if prev[y][x]=='#' && n>=limit { next[y][x] = 'L'; }
        }
    }
    }

    for y in 0..dy {
    for x in 0..dx
    {
        if next[y][x]!=prev[y][x] { return true; }
    }
    }
    
    false
}

fn count_seats(data:&mut Vec<Vec<char>>)->i32
{
    data.iter().map(|d| cnt(d,'#')).sum()
}

#[allow(unused)]
pub fn solve(data:&[String])->(i32,i32)
{
    let mut res = (0,0);
    let mut prev:Vec<Vec<char>> = vec![];
    let mut next:Vec<Vec<char>> = vec![];
    let mut org :Vec<Vec<char>> = vec![];

    for l in data
    {
        let mut line:Vec<char> = vec![];
        for c in l.chars()
        {            
            line.push(c);
        }
        prev.push(line.clone());
    }
    next = prev.clone();
    org  = prev.clone();


    while (moving(&mut next,&mut prev,true))
    {        
        prev = next.clone();
    }
   
    res.0 = count_seats(&mut next);

    prev = org;
    
    while (moving(&mut next,&mut prev,false))
    {        
        prev = next.clone();
    }
  
    res.1 = count_seats(&mut next);

    println!("Day11");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        "L.LL.LL.LL".to_string(),
        "LLLLLLL.LL".to_string(),
        "L.L.L..L..".to_string(),
        "LLLL.LL.LL".to_string(),
        "L.LL.LL.LL".to_string(),
        "L.LLLLL.LL".to_string(),
        "..L.L.....".to_string(),
        "LLLLLLLLLL".to_string(),
        "L.LLLLLL.L".to_string(),
        "L.LLLLL.LL".to_string(),
        ];
    assert_eq!(solve(&v).0,37);
}


#[test]
fn test2()
{
    let v = vec![
        "L.LL.LL.LL".to_string(),
        "LLLLLLL.LL".to_string(),
        "L.L.L..L..".to_string(),
        "LLLL.LL.LL".to_string(),
        "L.LL.LL.LL".to_string(),
        "L.LLLLL.LL".to_string(),
        "..L.L.....".to_string(),
        "LLLLLLLLLL".to_string(),
        "L.LLLLLL.L".to_string(),
        "L.LLLLL.LL".to_string(),
        ];    
    assert_eq!(solve(&v).1,26);
}
