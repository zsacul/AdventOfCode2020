use std::collections::HashSet;

#[derive(Clone,Copy,Debug)]
enum Instructions
{
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn execute(code : &mut Vec<Instructions>,part1:bool)->Option<i32> {
    let mut hash : HashSet<i32> = HashSet::new();
    let mut line = 0;
    let mut old_acc= 0;
    let mut acc    = 0;

    while hash.get(&line)==None && (part1 || line<code.len() as i32)
    {
        hash.insert(line);
        old_acc = acc;

        match code[line as usize] {
            Instructions::Acc(v)=>{ acc+=v; line+=1; },
            Instructions::Nop(_    )=>{         line+=1; },
            Instructions::Jmp(v)=>{         line+=v; },
        }
    }

    if part1  
    {
        Some(old_acc)
    }
    else
    {        
        if line>=code.len() as i32 { Some(acc) }
                              else { None      }
    }    
}

fn fuzz(code :&mut Vec<Instructions>)->i32
{
    for i in 0..code.len() {
        let old_code = code[i];
        
        match code[i] {
            Instructions::Nop(v)=> { code[i] = Instructions::Jmp(v); },
            Instructions::Jmp(v)=> { code[i] = Instructions::Nop(v); },
            Instructions::Acc(_    )=> {                             },
        };

        let exec = execute(code,false);
        if exec!=None { return exec.unwrap(); }
        
        code[i] = old_code;
    }
    -1
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i32,i32)
{
    let mut res = (0,0);    
    let mut code : Vec<Instructions> = vec![]; 

    for line in data {
        let v   : Vec<_> = line.split(" ").collect();
        let val : i32    = v[1].parse().unwrap();        

        match v[0] {
            "nop" => { code.push(Instructions::Nop(val)) },
            "acc" => { code.push(Instructions::Acc(val)) },
            "jmp" => { code.push(Instructions::Jmp(val)) },
            _     => { panic!("bad code");               }
        }        
    }

    res.0 = execute(&mut code,true).unwrap_or(-1);
    res.1 = fuzz(&mut code);

    println!("Day8");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}

#[test]
fn test1()
{
    let v = vec![
        "nop +0".to_string(),
        "acc +1".to_string(),
        "jmp +4".to_string(),
        "acc +3".to_string(),
        "jmp -3".to_string(),
        "acc -99".to_string(),
        "acc +1".to_string(),
        "jmp -4".to_string(),
        "acc +6".to_string(),
    ];
    assert_eq!(solve(&v).0,5);
}


#[test]
fn test2()
{
    let v = vec![
        "nop +0".to_string(),
        "acc +1".to_string(),
        "jmp +4".to_string(),
        "acc +3".to_string(),
        "jmp -3".to_string(),
        "acc -99".to_string(),
        "acc +1".to_string(),
        "jmp -4".to_string(),
        "acc +6".to_string(),
        ];    
    assert_eq!(solve(&v).1,8);
}
