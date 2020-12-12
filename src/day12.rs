fn rot90(x: i32,y: i32)->(i32,i32)
{
    (y,-x)
}

fn rot90angle(x:&mut i32,y:&mut i32,angle:i32)
{
    let mut po:(i32,i32) = (*x,*y);
    let times = ((360 + angle)%360)/90; 
    
    for _ in 0..times { po = rot90(po.0,po.1); }
    *x = po.0;
    *y = po.1;
}

fn solve1(data:&Vec<String>)->i32
{
    let dirs = [(0,1),(1,0),(0,-1),(-1,0)];
    let mut x = 0;
    let mut y = 0;
    let mut rot = 90;

    for l in data
    {
        let letter = l.chars().nth(0).unwrap();
        let num = l[1..].parse::<i32>().unwrap();

        match letter
        {
            'L' => { rot = (rot+360-num)%360; },
            'R' => { rot = (rot+    num)%360; },
            'F' => {
                let r = (rot/90) as usize;
                x+=num*dirs[r].0;
                y+=num*dirs[r].1; 
            },
            'N' => { y+=num; },
            'S' => { y-=num; },
            'W' => { x-=num; },
            'E' => { x+=num; },
            _ => panic!("unknown command")
        };
    }
    
    x.abs() + y.abs()
}

fn solve2(data:&Vec<String>)->i32
{
    let mut x  = 0;
    let mut y  = 0;
    let mut px = 10;
    let mut py = 1;

    for l in data
    {
        let letter = l.chars().nth(0).unwrap();
        let num     = l[1..].parse::<i32>().unwrap();

        match letter
        {
            'L' => { rot90angle(&mut px,&mut py,-num); },
            'R' => { rot90angle(&mut px,&mut py, num); },
            'F' => { 
                     x+=num*px;  
                     y+=num*py; 
                   },
            'N' => { py+=num; },
            'S' => { py-=num; },
            'W' => { px-=num; },
            'E' => { px+=num; },
            _ => panic!("err")
        }
    }

    x.abs() + y.abs()
}


#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i32,i32)
{
    let res : (i32,i32) = (solve1(data),solve2(data));
    
    println!("Day12");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        "F10".to_string(),
        "N3".to_string(),
        "F7".to_string(),
        "R90".to_string(),
        "F11".to_string(),
        ];
    assert_eq!(solve(&v).0,25);
}

#[test]
fn test2()
{
    let v = vec![
        "F10".to_string(),
        "N3".to_string(),
        "F7".to_string(),
        "R90".to_string(),
        "F11".to_string(),
          ];    
    assert_eq!(solve(&v).1,286);
}
