use std::collections::HashSet;

fn compute_score(tab:&[u8])->i64
{
    tab.iter().enumerate().map(|(id,&x)| (tab.len() as i64-id as i64)*x as i64).sum()
}

fn sub_game(id:i32,p1:&[u8],p2:&[u8])->(bool,i64)
{
    let mut states: HashSet<Vec<u8>> = HashSet::new();
    let mut pd1 = p1.to_owned();
    let mut pd2 = p2.to_owned();

    while !pd1.is_empty() && !pd2.is_empty()
    {
        let hash = vec![pd1.clone(),vec![255u8],pd2.clone()].into_iter().flatten().collect::<Vec<u8>>();

        if states.get(&hash)!=None
        {
            return (true,compute_score(p1));
        }
        states.insert(hash);

        let p1 = pd1.remove(0);
        let p2 = pd2.remove(0);
        let mut p1won = p1>p2;

        if p1 as usize<=pd1.len() && p2 as usize<=pd2.len()
        {
            p1won = sub_game(id+1, &pd1[0..p1 as usize].to_vec(), &pd2[0..p2 as usize].to_vec()).0;
        }
        
        if p1won
        {
            pd1.push(p1);
            pd1.push(p2);
        }
          else
        {
            pd2.push(p2);
            pd2.push(p1);
        }
    }

    let res = if !pd1.is_empty() { compute_score(&pd1) }
                                else { compute_score(&pd2) };    

    (!pd1.is_empty(),res)
}

pub fn solve1(deck1:&[u8],deck2:&[u8])->i64
{    
    let mut player1_deck = deck1.to_owned();
    let mut player2_deck = deck2.to_owned();
    
    while !player1_deck.is_empty() && !player2_deck.is_empty()
    {
        let p1 = player1_deck.remove(0);
        let p2 = player2_deck.remove(0);
        
        if p1>p2 
        {
            player1_deck.push(p1);
            player1_deck.push(p2);
        }
        else
        {
            player2_deck.push(p2);
            player2_deck.push(p1);
        }
    }

    if !player1_deck.is_empty() { compute_score(&player1_deck) }
                           else { compute_score(&player2_deck) }   
}

pub fn solve2(deck1:&[u8],deck2:&[u8])->i64
{
    sub_game(1,deck1,deck2).1    
}

#[allow(unused)]
pub fn solve(data:&[String])->(i64,i64)
{
    let mut player1_deck = vec![];
    let mut player2_deck = vec![];
    let mut player1 = true;
    
    for line in data {
             if line.contains("Player 1:") { }
        else if line.contains("Player 2:")
        {
            player1 = false;
        }
        else if !line.is_empty()
        {
            let num:u8 = line.parse().unwrap();
            if player1 { player1_deck.push(num); }
                  else { player2_deck.push(num); }
        }        
    }

    let res = (solve1(&player1_deck,&player2_deck),
                         solve2(&player1_deck,&player2_deck));

    println!("Day22");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        "Player 1:".to_string(),
        "9".to_string(),
        "2".to_string(),
        "6".to_string(),
        "3".to_string(),
        "1".to_string(),
        "".to_string(),
        "Player 2:".to_string(),
        "5".to_string(),
        "8".to_string(),
        "4".to_string(),
        "7".to_string(),
        "10".to_string(),
    ];
    assert_eq!(solve(&v).0,306);
}

#[test]
fn test2()
{
    let v = vec![
        "Player 1:".to_string(),
        "9".to_string(),
        "2".to_string(),
        "6".to_string(),
        "3".to_string(),
        "1".to_string(),
        "".to_string(),
        "Player 2:".to_string(),
        "5".to_string(),
        "8".to_string(),
        "4".to_string(),
        "7".to_string(),
        "10".to_string(),
    ];
    assert_eq!(solve(&v).1,291);
}
