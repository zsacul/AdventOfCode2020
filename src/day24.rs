use std::collections::HashMap;

fn get_dir(data:&String,i:&mut usize)->(i32,i32)
{    
    let dir = &[data.chars().nth(*i).unwrap().to_string(),
                     data.chars().nth(*i+1).unwrap_or(' ').to_string()].join("")[..];
    *i+=1;
  
    match dir
    {
        "se" => { *i+=1; ( 1,-1)  },
        "sw" => { *i+=1; ( 0,-1)  },
        "ne" => { *i+=1; ( 0, 1)  },
        "nw" => { *i+=1; (-1, 1)  },
        "e " =>          ( 1, 0),
        "w " =>          (-1, 0),
        _ => 
        { 
            match dir.chars().nth(0).unwrap()
            {
                'e' => ( 1, 0),
                'w' => (-1, 0),
                _  => { panic!("unknown code:[{}]",dir) }
            }
        },
    }        
}

pub fn solve1(data:&Vec<String>,tiles :&mut HashMap<(i32,i32),bool>)
{
    for line in data
    {        
        let mut i=0;
        let mut pos = (0,0);

        while i<line.len()
        {            
            let dir = get_dir(line,&mut i);            
            pos.0+=dir.0;
            pos.1+=dir.1;
        }
        tiles.insert(pos,  !tiles.get(&pos).unwrap_or(&true));
    }
}

fn count(hash:&HashMap<(i32,i32),bool>,x:i32,y:i32)->i32
{
    let dirs = [( 1,-1),( 0,-1),( 0, 1),(-1, 1),( 1, 0),(-1, 0)];
    let mut cnt = 0;
    
    for (xx,yy) in dirs.iter()
    {
        if !*hash.get(&(x+xx,y+yy)).unwrap_or(&true) { cnt+=1; }        
    }

    cnt
}


fn simulate(prev:&HashMap<(i32,i32),bool>,next:&mut HashMap<(i32,i32),bool>)
{
    let positions = [( 0, 0),( 1,-1),( 0,-1),( 0, 1),(-1, 1),( 1, 0),(-1, 0)];

    for ((px,py) ,_)in prev.clone() 
    {
        for (xx,yy) in positions.iter()
        {
            let newp : (i32,i32) = (px+*xx,py+*yy);
            let count = count(prev,newp.0,newp.1);
            next.insert(newp, true);
            
            if *prev.get(&newp).unwrap_or(&true)==true
            {
                if count==2             { next.insert(newp, false); }
            }
                else
            {
                if count==1 || count==2 { next.insert(newp, false); }
            }
        }
    }
}

pub fn solve2(tiles :&mut HashMap<(i32,i32),bool>)->i64
{
    let mut next : HashMap<(i32,i32),bool> = tiles.clone();

    for _ in 1..=100
    {
        simulate(&next.clone(),&mut next);        
        next = next.into_iter().filter(|x| x.1==false).collect();
    }    

    next.len() as i64  
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i64,i64)
{
    let mut tiles : HashMap<(i32,i32),bool> = HashMap::new();
    solve1(data,&mut tiles);
     
    let mut res = (tiles.clone().into_iter().filter(|(_,b)| !*b).collect::<HashMap<(i32,i32),bool>>().len() as i64,
                            solve2(&mut tiles));

    println!("Day24");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}

#[test]
fn test1()
{
    let v = vec![
        "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
        "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
        "seswneswswsenwwnwse".to_string(),
        "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
        "swweswneswnenwsewnwneneseenw".to_string(),
        "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
        "sewnenenenesenwsewnenwwwse".to_string(),
        "wenwwweseeeweswwwnwwe".to_string(),
        "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
        "neeswseenwwswnwswswnw".to_string(),
        "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
        "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
        "sweneswneswneneenwnewenewwneswswnese".to_string(),
        "swwesenesewenwneswnwwneseswwne".to_string(),
        "enesenwswwswneneswsenwnewswseenwsese".to_string(),
        "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
        "nenewswnwewswnenesenwnesewesw".to_string(),
        "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
        "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
        "wseweeenwnesenwwwswnew".to_string(),
        ];
    assert_eq!(solve(&v).0,10);
}

#[test]
fn test2()
{
    let v = vec![
        "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
        "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
        "seswneswswsenwwnwse".to_string(),
        "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
        "swweswneswnenwsewnwneneseenw".to_string(),
        "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
        "sewnenenenesenwsewnenwwwse".to_string(),
        "wenwwweseeeweswwwnwwe".to_string(),
        "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
        "neeswseenwwswnwswswnw".to_string(),
        "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
        "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
        "sweneswneswneneenwnewenewwneswswnese".to_string(),
        "swwesenesewenwneswnwwneseswwne".to_string(),
        "enesenwswwswneneswsenwnewswseenwsese".to_string(),
        "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
        "nenewswnwewswnenesenwnesewesw".to_string(),
        "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
        "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
        "wseweeenwnesenwwwswnew".to_string(),
        ];
    assert_eq!(solve(&v).1,2208);
}
