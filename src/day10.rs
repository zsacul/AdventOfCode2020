use std::collections::HashMap;

#[derive(Copy, Clone,Debug,PartialEq,Eq,Hash)]
struct Val
{
    act:i32,
    pos:usize,
}

fn part1(adap:&mut Vec<i32>)->i32
{
    let mut act = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for a in adap.iter() {
        if *a<=act+3
        {
            if act+1==*a { diff_1+=1; }
            if act+3==*a { diff_3+=1; }
            act = *a;
        }
    }
    diff_3*diff_1
}

fn count(h:&mut HashMap<Val,i64>,data:&mut Vec<i32>,act:i32, pos:usize)->i64
{
    if pos>=data.len()   { return 0; }    
    if data[pos]-act>3   { return 0; }
    if pos==data.len()-1 { return 1; }
    
    let v = Val {
        act,
        pos,
    };

    if h.contains_key(&v) {
        return *h.get(&v).unwrap();
    }

    let res = count(h,data,data[pos],pos+1) 
                + count(h,data,data[pos],pos+2) 
                + count(h,data,data[pos],pos+3);

    h.insert(v,res);
    res    
}

#[allow(unused)]
pub fn solve(data:&[i32])->(i32,i64)
{
    let mut res = (0,0);
    let mut adap = data.to_owned();

    adap.sort_unstable();
    adap.push(adap[adap.len()-1]+3);
    
    res.0 = part1(&mut adap);

    let mut hash : HashMap<Val,i64> = HashMap::new();    
    adap.insert(0,0);

    res.1 = count(&mut hash,&mut adap,0,0);

    println!("Day10");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        16,
        10,
        15,
        5,
        1,
        11,
        7,
        19,
        6,
        12,
        4,
    ];
    assert_eq!(solve(&v).1,8);
}


#[test]
fn test2()
{
    let v = vec![
        28,
        33,
        18,
        42,
        31,
        14,
        46,
        20,
        48,
        47,
        24,
        23,
        49,
        45,
        19,
        38,
        39,
        11,
        1,
        32,
        25,
        35,
        8,
        17,
        7,
        9,
        4,
        2,
        34,
        10,
        3,
        ];    
    assert_eq!(solve(&v).1,19208);
}

/*

#[test]
fn test3()
{
    let v = vec![
    ];
    assert_eq!(solve(&v).1,126);
}


*/