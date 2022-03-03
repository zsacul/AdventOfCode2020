
fn solve1(buses:&[i32],time:i32)->i32
{
    for t in time..time+1000000000 
    {
        for &b in buses.iter()
        {
            if t%b==0 { return (t-time)*b; }
        }
    }
    -1
}

fn nwd(a:i128,b:i128)->i128
{
    let mut temp: i128;
    let mut aa= a;
    let mut bb= b;

    while bb!=0
    {
        temp = bb;
        bb = aa%bb;
        aa = temp;
    }
    aa
}

fn nww(a:i128,b:i128)->i128
{
    (a*b)/nwd(a,b)
}

fn solve2(buses:&[i32])->i128
{
    let data:Vec<_> = buses.iter().enumerate().map(|(id,&v)| (id,v as i128)).filter(|(_,v)| *v!=-1).collect();

    let mut time = 1;
    let mut locked = 0;
    let mut step = 1;

    loop
    {
        let mut ok=0;
        for (id,val) in data.iter()
        {            
            if (time+*id as i128)%val==0 { ok+=1; }
                                    else { break; }
        }

        if ok==data.len() { return time; }

        if ok>locked {
            locked = ok;
            step = nww(step,data[locked-1].1);
        }
        time+=step;
    }
}

#[allow(unused)]
pub fn solve(data:&[String])->(i32,i128)
{
    let mut res : (i32,i128) = (0,0);   
    let mut time = data[0].parse::<i32>().unwrap();

    let buses1:Vec<_> = data[1].split(',').filter(|&c| c!="x").map(|x|x.parse().unwrap()).collect();
    let buses2:Vec<_> = data[1].split(',').map(|x| if x=="x" {-1} else {x.parse().unwrap()}).collect();

    res.0 = solve1(&buses1,time);
    res.1 = solve2(&buses2);  

    println!("Day13");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        "939".to_string(),
        "7,13,x,x,59,x,31,19".to_string(),
        ];
    assert_eq!(solve(&v).0,295);
}

 #[test]
fn test2()
{
    let v = vec![
        "939".to_string(),
        "17,x,13,19".to_string(),
          ];    
    assert_eq!(solve(&v).1,3417);
}

#[test]
fn test3()
{
    let v = vec![
        "939".to_string(),
        "67,7,59,61".to_string(),        
          ];    
    assert_eq!(solve(&v).1,754018);
}

#[test]
fn test4()
{
    let v = vec![
        "939".to_string(),
        "67,x,7,59,61".to_string(),        
          ];    
    assert_eq!(solve(&v).1,779210);     
}

#[test]
fn test5()
{
    let v = vec![
        "939".to_string(),
        "67,7,x,59,61".to_string(),        
          ];    
    assert_eq!(solve(&v).1,1261476);
}

#[test]
fn test6()
{
    let v = vec![
        "939".to_string(),
        "1789,37,47,1889".to_string(),        
          ];    
    assert_eq!(solve(&v).1,1202161486);
}


 