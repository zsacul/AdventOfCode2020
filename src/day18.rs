fn eval_pure(l:&String)->i64
{
    let mut acc=0;
    let mut num:i64=0;
    let mut operator : Option<char> = None;
    let mut line = l.clone();
    line.push_str(" ");

    for c in line.chars() {
        match c 
        {            
            '0'..='9'=> { 
                            num = num*10 + c.to_digit(10).unwrap() as i64;
                        },
            '+'=> { operator = Some(c); },
            '-'=> { operator = Some(c); },
            '*'=> { operator = Some(c); },
            '/'=> { operator = Some(c); },
            ' '=> {  
                    if num!=0 
                    {
                        if operator!=None 
                        {
                            match operator.unwrap()
                            {
                                '+'=> { acc+=num; num=0; },
                                '-'=> { acc-=num; num=0; },
                                '*'=> { acc*=num; num=0; },
                                '/'=> { acc/=num; num=0; },
                                _  => { panic!("wrong operator")},
                            }
                            operator = None;
                        }
                        else
                        {
                            acc = num; 
                            num = 0;
                        }       
                    }          
                  },
            _ => { println!("unknown char:{}",c); }
        }
    }
    acc
}

fn find_pos(l:&String,pos:i32,delta:i32)->i32
{
    let mut p = pos;
    loop 
    {
        if p<0 || p>=l.len() as i32 { return p; }
        if l.chars().nth(p as usize).unwrap_or(' ')==' ' { return p; }
        p+=delta;
    }
}

fn eval(l:&String,part1:bool)->i64
{
    let mut line = (*l).clone();

    //eval and replace all equations in barackets first
    while line.find("(")!=None
    {
        let bracket_start = line.find('(').unwrap();
        let mut bracket_end = 0;
        let mut opened_brackets=1;

        //find matching baracket
        for i in bracket_start+1..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c=='(' {
                opened_brackets+=1;
            }
            if c==')' {
                opened_brackets-=1;
                if opened_brackets==0 {
                    bracket_end = i;
                    break;
                }
            }
        }

        let mut ns = line[..bracket_start].to_string();        
        ns.push_str(&eval(&line[bracket_start+1..bracket_end].to_string(),part1).to_string()[..]);
        ns.push_str(&line[bracket_end+1..].to_string());

        line = ns;
    }

    if !part1
    {
        //eval and replace all + operators first
        while line.find("+")!=None
        {
            let l_pos = find_pos(&line,line.find("+").unwrap() as i32-2,-1);
            let r_pos = find_pos(&line,line.find("+").unwrap() as i32+2, 1);

            let mut ns = "".to_string();

            if l_pos>0 {
                ns.push_str(&line[..l_pos as usize].to_string());
            }

                ns.push_str(" ");
                ns.push_str(&eval_pure(&line[(l_pos+1) as usize..r_pos as usize].to_string()).to_string()[..]);
                ns.push_str(" ");

            if r_pos+1<line.len() as i32 {
                ns.push_str(&line[(r_pos+1) as usize..].to_string());
            }

            line = ns;
        }
    }

    eval_pure(&line)
}

pub fn solve1(data:&Vec<String>)->i64
{
    data.iter().map(|l| eval(l,true)).sum()
}

pub fn solve2(data:&Vec<String>)->i64
{
    data.iter().map(|l| eval(l,false)).sum()
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i64,i64)
{
    let res = (solve1(data),solve2(data));

    println!("Day18");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec!["1 + 2 * 3 + 4 * 5 + 6".to_string()];
    assert_eq!(solve1(&v),71);
}

#[test]
fn test2()
{
    let v = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()];    
    assert_eq!(solve1(&v),51);
}

#[test]
fn test3()
{
    let v = vec!["2 * 3 + (4 * 5)".to_string()];
    assert_eq!(solve1(&v),26);
}

#[test]
fn test4()
{
    let v = vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()];
    assert_eq!(solve1(&v),437);
}

#[test]
fn test5()
{
    let v = vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()];
    assert_eq!(solve1(&v),12240);
}

#[test]
fn test6()
{
    let v = vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()];
    assert_eq!(solve1(&v),13632);
}

#[test]
fn test_part2_6()
{
    let v = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()];
    assert_eq!(solve2(&v),51);
}

#[test]
fn test_part2_7()
{
    let v = vec!["2 * 3 + (4 * 5)".to_string()];
    assert_eq!(solve2(&v),46);
}

#[test]
fn test_part2_8()
{
    let v = vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()];
    assert_eq!(solve2(&v),1445);
}

#[test]
fn test_part2_9()
{
    let v = vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()];
    assert_eq!(solve2(&v),669060);
}

#[test]
fn test_part2_10()
{
    let v = vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()];
    assert_eq!(solve2(&v),23340);
}
