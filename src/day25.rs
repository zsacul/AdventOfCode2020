fn transform(val:usize,loop_size:usize)->usize
{
    (1..loop_size).fold(val, |acc,_| (acc*val)%20201227)
}

fn find(val:usize,key:usize)->usize
{
    let mut subject_number = val;
    let mut loop_count = 1;
    while subject_number!=key {
        subject_number = (subject_number*val)%20201227;
        loop_count+=1;
    }
    loop_count
}

pub fn solve1(data:&Vec<String>)->usize
{
    let door_public_key : usize = data[0].parse().unwrap();
    let card_public_key : usize = data[1].parse().unwrap();
    
    let card_loop_size = find(7,card_public_key);
    let door_loop_size = find(7,door_public_key);

    let encryption_key1 = transform(door_public_key,card_loop_size);
    let encryption_key2 = transform(card_public_key,door_loop_size);

    assert_eq!(encryption_key1,encryption_key2);
    encryption_key1
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(usize,usize)
{
    let res = (solve1(data),0);

    println!("Day25");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    assert_eq!(transform(7,8),5764801);
}

#[test]
fn test2()
{
    assert_eq!(transform(7,11),17807724);
}
