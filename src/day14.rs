use std::collections::HashMap;

fn get_masks(s:&str)->(u64,u64,u64,u64)
{
    let mut mask0  = 0u64;
    let mut mask1  = 0u64;
    let mut maskf  = 0u64;
    let mut number_x = 0u64;

    for c in s.chars() 
    {
        mask0<<=1;
        mask1<<=1;
        maskf<<=1;
        if c=='0' { mask0|=1; }
        if c=='1' { mask1|=1; }
        if c=='X' { maskf|=1; number_x+=1; }
    }
    (mask0,mask1,maskf,number_x)
}

fn vector_to_binary(table:&[u8])->u64
{
    table.iter().rev().fold(0,|acc,&x| ((acc<<1) | x as u64))
}

fn get_mask01(i:u64,mask_x:u64)->u64
{    
    let mut mask= mask_x;
    let mut pos= 0u64;
    let mut tab0:Vec<u8> = vec![];

    while mask>0
    {
        tab0.push(0);

        if mask&1==1
        {
            if i&(1u64<<pos)!=0
            {                
                let last = tab0.len()-1;
                tab0[last] = 1;
            }
            pos+=1;
        }      
        mask>>=1;
    }
  
    vector_to_binary(&tab0)
}

fn parse_val_adr(tab:&[&str])->(u64,u64)
{
    let val:u64    = tab[1].parse().unwrap();
    let add1 = tab[0].find('[').unwrap();
    let add2 = tab[0].find(']').unwrap();
    let addr:u64   = tab[0][add1+1..add2].parse().unwrap();
    (val,addr)
}

fn solve1(data:&[String])->u64
{
    let mut mask_0 : u64 = 0;
    let mut mask_1 : u64 = 0;
    let mut mem    : HashMap<u64,u64> = HashMap::new();

    for line in data {
        let tab:Vec<_> = line.split(" = ").collect();

        match &line[0..3]
        {
            "mem"=>{
                let (val,addr) = parse_val_adr(&tab);
                let mut vv = val;
                vv&=!mask_0;
                vv|= mask_1;
                mem.insert(addr,vv);
            },
            "mas"=>{
                let masks = get_masks(tab[1]);
                mask_0 = masks.0;
                mask_1 = masks.1;
            },
            _ =>panic!("err"),
        }
    }

    mem.values().sum()
}


fn solve2(data:&[String])->u64
{
    let mut mask_1 : u64 = 0;
    let mut mask_x : u64 = 0;
    let mut mask_c : u64 = 0;
    let mut mem    : HashMap<u64,u64> = HashMap::new();

    for line in data {
        let tab:Vec<_> = line.split(" = ").collect();

        match &line[0..3]
        {
            "mem"=>{
                let (val,addr) = parse_val_adr(&tab);              
                let count = 1u64<<mask_c;

                for i in 0u64..count {
                    let mut adr =addr;
                    adr|=  mask_1;               
                    adr&= !mask_x;
                    adr|= get_mask01(i,mask_x);
                    mem.insert(adr,val);
                }   
            },
            "mas"=>{
                let masks = get_masks(tab[1]);
                mask_1 = masks.1;
                mask_x = masks.2;
                mask_c = masks.3;
            },
            _ =>panic!("err"),
        }
    }

    mem.values().sum()
}

#[allow(unused)]
pub fn solve(data:&[String])->(u64,u64)
{
    let res = (solve1(data),solve2(data));

    println!("Day14");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        "mem[8] = 11".to_string(),
        "mem[7] = 101".to_string(),
        "mem[8] = 0".to_string(),
        ];
    assert_eq!(solve1(&v),165);
}

#[test]
fn test2()
{
    let v = vec![
        "mask = 000000000000000000000000000000X1001X".to_string(),
        "mem[42] = 100".to_string(),
        "mask = 00000000000000000000000000000000X0XX".to_string(),
        "mem[26] = 1".to_string(),
        ];    
        assert_eq!(solve2(&v),208);
}


#[test]
fn test_mask1()
{    
    assert_eq!( get_mask01(1,0b100001),0b000001);
    assert_eq!( get_mask01(1,0b111),0b001);
    assert_eq!( get_mask01(2,0b111),0b010);
    assert_eq!( get_mask01(7,0b111),0b111);
    assert_eq!( get_mask01(5,0b111),0b101);
}

#[test]
fn test_mask2()
{
    assert_eq!( get_mask01(2,0b100001),0b100000);
}

#[test]
fn test_mask3()
{
    assert_eq!( get_mask01(3,0b100001),0b100001);
}