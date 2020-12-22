use std::collections::{HashSet};

#[derive(Debug,Clone)]
struct Tile
{
    l:u16,
    r:u16,
    u:u16,
    d:u16,
    h:u64,
    id:usize,
    tab:Vec<Vec<char>>
}

impl Tile
{
    fn new(id:usize,s:&Vec<String>)->Self
    {
        let mut t = vec![];
        for l in s {
            let mut line =vec![];
            for c in l.chars(){
                line.push(c);
            }
            t.push(line);
        }

        let mut t = Self
        {
            l:0,
            r:0,
            u:0,
            d:0,
            h:0,
            id,
            tab:t
        };

        t.calc_hashes();
        t
    }

    #[allow(unused)]    
    fn print(&self)
    {
        println!("id:{}",self.id);
        for l in self.tab.iter() {
            for c in l
            {
                print!("{}",c);
            }
            println!("");
        }

        println!("l:{} r:{} u:{} d:{} hash:{}",self.l,self.r,self.u,self.d,self.h);
    }
    
    fn flip_x_s(ta:&mut Vec<Vec<char>>)
    {
        let n = ta.len();
        for y in 0..n {
            for x in 0..n/2 {
                let t   = ta[y][    x];
                ta[y][    x] = ta[y][n-1-x];
                ta[y][n-1-x] = t;
            }
        }
    }

    fn flip_y_s(ta:&mut Vec<Vec<char>>)
    {
        let n = ta.len();
        for x in 0..n {
            for y in 0..n/2 {
                let t   = ta[    y][x];
                ta[    y][x] = ta[n-1-y][x];
                ta[n-1-y][x] = t;
            }
        }        
    }

    fn flip_x(&mut self)
    {        
        Tile::flip_x_s(&mut self.tab);
        self.calc_hashes();
    }

    fn flip_y(&mut self)
    {
        Tile::flip_y_s(&mut self.tab);
        self.calc_hashes();
    }

    fn rot90(n:usize,x: usize,y: usize)->(usize,usize)
    {
        (y,n-1-x)
    }

    fn rotate_s(ta:&mut Vec<Vec<char>>)
    {
        let n = ta.len();
        let mut new_tab:Vec<Vec<char>> = vec![vec![' ';n];n];
        
        for y in 0..n
        {
            for x in 0..n {
                let r = Tile::rot90(n,x,y);
                new_tab[y][x] = ta[r.1][r.0];
            }
        }
        *ta = new_tab.clone();        
    }

    fn rotate(&mut self)
    {
        Tile::rotate_s(&mut self.tab);
        self.calc_hashes();
    }

    fn calc_hashes(&mut self)
    {
        self.l = 0;
        self.r = 0;
        self.u = 0;
        self.d = 0;
        self.h = 0;

        if self.tab.len()>0
        {
            for i in 0..10
            {
                self.l <<= 1;
                self.r <<= 1;
                self.u <<= 1;
                self.d <<= 1;
                if self.tab[0][i]=='#' { self.u|=1; }
                if self.tab[9][i]=='#' { self.d|=1; }
                if self.tab[i][0]=='#' { self.l|=1; }
                if self.tab[i][9]=='#' { self.r|=1; }
            }
        }

        self.h = ((self.l as u64)<<30) | ((self.r as u64)<<20) | ((self.u as u64)<<10u64) | (self.d as u64);
    }
}

fn tile_matches(sol:&Vec<Vec<Tile>>,x:usize,y:usize,t:&Tile)->bool 
{
    if x>0 {  
        if sol[y  ][x-1].r!=0 && sol[y  ][x-1].r!=t.l { return false; }
    }
    if y>0 {  
        if sol[y-1][x  ].d!=0 && sol[y-1][x  ].d!=t.u { return false; }
    }
    if x<sol.len()-1 {  
        if sol[y  ][x+1].l!=0 && sol[y  ][x+1].l!=t.r { return false; }
    }
    if y<sol.len()-1 {  
        if sol[y+1][x  ].u!=0 && sol[y+1][x  ].u!=t.d { return false; }
    }
    true
}

fn try_solve(mut sol:&mut Vec<Vec<Tile>>,mut used:&mut HashSet<usize>,tiles:&Vec<Tile>,tile_i:usize,pos:usize,n:usize)->i64
{
    if pos==n*n {
        return sol[  0][  0].id as i64 * 
               sol[n-1][  0].id as i64 * 
               sol[  0][n-1].id as i64 * 
               sol[n-1][n-1].id as i64;
    }

    let id = tiles[tile_i].id;

    if used.get(&id)!=None { return 0; }
    
    let x = pos%n;
    let y = pos/n;

    if used.get(&id)==None && tile_matches(&mut sol,x,y,&tiles[tile_i]) {

        used.insert(id);
        let org_tile = sol[y][x].clone();
        sol[y][x] = tiles[tile_i].clone();

        for tile_i in 0..tiles.len()
        {
            let r=  try_solve(&mut sol,&mut used,tiles,tile_i,pos+1,n);
            if r>0 { return r; }
        }

        sol[y][x] = org_tile;
        used.remove(&id);
    }

    0
}

fn is_snake(picture:&Vec<Vec<char>>,snake:&Vec<String>,xx:usize,yy:usize)->bool
{
    for y in 0..snake.len() {
        for x in 0..snake[y].len() {
            if snake[y].chars().nth(x).unwrap()=='#'
            {
                if picture[yy+y][xx+x]!='#' { return false; }
            }
        }
    }
    
    true
}

fn get_snake()->Vec<String>
{
    vec![
        "                  # ".to_string(),
        "#    ##    ##    ###".to_string(),
        " #  #  #  #  #  #   ".to_string(),
    ]
}

fn find_snakes(picture:&Vec<Vec<char>>)->Vec<(usize,usize)>
{
    let mut res:Vec<(usize,usize)> = vec![];
    let snake = get_snake();

    for yy in 0..picture.len()-snake.len()
    {
        for xx in 0..picture[yy].len()-snake[0].len()
        {
            if is_snake(&picture,&snake,xx,yy)
            {
                res.push((xx,yy));
            }
        }
    }
    res
}

fn count_non_snakes(picture:&Vec<Vec<char>>,positions:&Vec<(usize,usize)>)->i32
{
    let mut picture = picture.clone();
    let snake = get_snake();

    for &pos in positions
    {
        for y in 0..snake.len() {
            for x in 0..snake[0].len() {
                if snake[y].chars().nth(x).unwrap()=='#' {
                    picture[y+pos.1][x+pos.0]='O';
                }
            }
        }
    }

    picture.iter().map(|l| l.iter().filter(|&&c|c=='#').count() as i32).sum()
}

fn count_snakes(picture:Vec<Vec<char>>)->i32
{
    let mut picture_o = picture.clone();
    let mut picture_x = picture.clone();
    let mut picture_y = picture.clone();

    Tile::flip_x_s(&mut picture_x);
    Tile::flip_y_s(&mut picture_y);
    
    for _ in 0..4
    {
        let s = find_snakes(&picture_o);            
        if s.len()>0 { 
            return count_non_snakes(&picture_o,&s);
        }
        Tile::rotate_s(&mut picture_o);
    }

    for _ in 0..4
    {
        let s = find_snakes(&picture_x);            
        if s.len()>0 { 
            return count_non_snakes(&picture_x,&s);
        }
        Tile::rotate_s(&mut picture_x);            
    }

    for _ in 0..4
    {
        let s = find_snakes(&picture_y);            
        if s.len()>0 { 
            return count_non_snakes(&picture_y,&s);
        }
        Tile::rotate_s(&mut picture_y);
    }
    -1
}

pub fn solve1(data:&Vec<String>)->(i64,i64)
{
    let mut id=0;
    let mut tab:Vec<String> = vec![];
    let mut tiles:Vec<Tile> = vec![];

    for line in data.iter()
    {
        if line.find("Tile ")!=None
        {
            let v:Vec<_> = line.split(" ").collect();
            let v2:Vec<_> = v[1].split(":").collect();
            id = v2[0].parse().unwrap();
        }
        else if line.len()==0
        {
            tiles.push(Tile::new(id,&tab));
            tab.clear();
        }
        else
        {
            tab.push(line.clone());
        }
    }
    
    let n_tiles = tiles.len();
    let n = (n_tiles as f32).sqrt() as usize;
    

    for i in 0..n_tiles {
        let mut tt = tiles[i].clone();       
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.rotate();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.rotate();
        tt.rotate();
        tt.rotate();
        tiles.push(tt);


        let mut tt = tiles[i].clone();       
        tt.flip_x();        
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_x();        
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_x();        
        tt.rotate();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_x();        
        tt.rotate();
        tt.rotate();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_y();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_y();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_y();
        tt.rotate();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_y();
        tt.rotate();
        tt.rotate();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_x();
        tt.flip_y();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_x();
        tt.flip_y();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_x();
        tt.flip_y();
        tt.rotate();
        tt.rotate();
        tiles.push(tt);

        let mut tt = tiles[i].clone();       
        tt.flip_x();
        tt.flip_y();
        tt.rotate();
        tt.rotate();
        tt.rotate();
        tiles.push(tt);   
    }


    //println!("tiles mix:{}",tiles.len());
    let mut vs:Vec<_>=tiles.iter().enumerate().map(|(pos,t)|(t.h as i32,pos)).collect();
    vs.sort();

    let mut final_tiles = vec![];
    let mut last_hash=-1;

    for (h,pos) in vs.iter() {
        if *h!=last_hash {
            last_hash = *h;            
            final_tiles.push(tiles[*pos].clone());
        }
    }

    //println!("final mix2:{:?}",final_tiles.len());

    let mut sol = vec![vec![Tile::new(0, &vec![]);n];n];
    let mut used:HashSet<usize> = HashSet::new();
    
    for tile_i in 0..final_tiles.len()
    {
        let r = try_solve(&mut sol,&mut used,&final_tiles,tile_i,0,n);

        if r>0 
        {
            let mut picture:Vec<Vec<char>> = vec![];

            for yy in 0..n {
                
                for yp in 0..8
                {
                    let mut line:Vec<char> = vec![];
                    for xx in 0..n 
                    {
                        for xp in 0..8
                        {
                            line.push(sol[yy][xx].tab[yp+1][xp+1]);
                        }
                    }               
                    picture.push(line.clone());
                }
                
            }

            return (r,count_snakes(picture) as i64); 
        }
    }
   
    (0,0)
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i64,i64)
{
    let res = solve1(data);

    println!("Day20");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        "Tile 2311:".to_string(),
        "..##.#..#.".to_string(),
        "##..#.....".to_string(),
        "#...##..#.".to_string(),
        "####.#...#".to_string(),
        "##.##.###.".to_string(),
        "##...#.###".to_string(),
        ".#.#.#..##".to_string(),
        "..#....#..".to_string(),
        "###...#.#.".to_string(),
        "..###..###".to_string(),
        "".to_string(),
        "Tile 1951:".to_string(),
        "#.##...##.".to_string(),
        "#.####...#".to_string(),
        ".....#..##".to_string(),
        "#...######".to_string(),
        ".##.#....#".to_string(),
        ".###.#####".to_string(),
        "###.##.##.".to_string(),
        ".###....#.".to_string(),
        "..#.#..#.#".to_string(),
        "#...##.#..".to_string(),
        "".to_string(),
        "Tile 1171:".to_string(),
        "####...##.".to_string(),
        "#..##.#..#".to_string(),
        "##.#..#.#.".to_string(),
        ".###.####.".to_string(),
        "..###.####".to_string(),
        ".##....##.".to_string(),
        ".#...####.".to_string(),
        "#.##.####.".to_string(),
        "####..#...".to_string(),
        ".....##...".to_string(),
        "".to_string(),
        "Tile 1427:".to_string(),
        "###.##.#..".to_string(),
        ".#..#.##..".to_string(),
        ".#.##.#..#".to_string(),
        "#.#.#.##.#".to_string(),
        "....#...##".to_string(),
        "...##..##.".to_string(),
        "...#.#####".to_string(),
        ".#.####.#.".to_string(),
        "..#..###.#".to_string(),
        "..##.#..#.".to_string(),
        "".to_string(),
        "Tile 1489:".to_string(),
        "##.#.#....".to_string(),
        "..##...#..".to_string(),
        ".##..##...".to_string(),
        "..#...#...".to_string(),
        "#####...#.".to_string(),
        "#..#.#.#.#".to_string(),
        "...#.#.#..".to_string(),
        "##.#...##.".to_string(),
        "..##.##.##".to_string(),
        "###.##.#..".to_string(),
        "".to_string(),
        "Tile 2473:".to_string(),
        "#....####.".to_string(),
        "#..#.##...".to_string(),
        "#.##..#...".to_string(),
        "######.#.#".to_string(),
        ".#...#.#.#".to_string(),
        ".#########".to_string(),
        ".###.#..#.".to_string(),
        "########.#".to_string(),
        "##...##.#.".to_string(),
        "..###.#.#.".to_string(),
        "".to_string(),
        "Tile 2971:".to_string(),
        "..#.#....#".to_string(),
        "#...###...".to_string(),
        "#.#.###...".to_string(),
        "##.##..#..".to_string(),
        ".#####..##".to_string(),
        ".#..####.#".to_string(),
        "#..#.#..#.".to_string(),
        "..####.###".to_string(),
        "..#.#.###.".to_string(),
        "...#.#.#.#".to_string(),
        "".to_string(),
        "Tile 2729:".to_string(),
        "...#.#.#.#".to_string(),
        "####.#....".to_string(),
        "..#.#.....".to_string(),
        "....#..#.#".to_string(),
        ".##..##.#.".to_string(),
        ".#.####...".to_string(),
        "####.#.#..".to_string(),
        "##.####...".to_string(),
        "##..#.##..".to_string(),
        "#.##...##.".to_string(),
        "".to_string(),
        "Tile 3079:".to_string(),
        "#.#.#####.".to_string(),
        ".#..######".to_string(),
        "..#.......".to_string(),
        "######....".to_string(),
        "####.#..#.".to_string(),
        ".#...#.##.".to_string(),
        "#.#####.##".to_string(),
        "..#.###...".to_string(),
        "..#.......".to_string(),
        "..#.###...".to_string(),
        "".to_string(),
        ];
    assert_eq!(solve1(&v).0,20899048083289);
}

#[test]
fn test2()
{
    let v:Vec<Vec<char>> = 
    vec![
        ".#.#..#.##...#.##..#####".chars().collect(),
        "###....#.#....#..#......".chars().collect(),
        "##.##.###.#.#..######...".chars().collect(),
        "###.#####...#.#####.#..#".chars().collect(),
        "##.#....#.##.####...#.##".chars().collect(),
        "...########.#....#####.#".chars().collect(),
        "....#..#...##..#.#.###..".chars().collect(),
        ".####...#..#.....#......".chars().collect(),
        "#..#.##..#..###.#.##....".chars().collect(),
        "#.####..#.####.#.#.###..".chars().collect(),
        "###.#.#...#.######.#..##".chars().collect(),
        "#.####....##..########.#".chars().collect(),
        "##..##.#...#...#.#.#.#..".chars().collect(),
        "...#..#..#.#.##..###.###".chars().collect(),
        ".#.#....#.##.#...###.##.".chars().collect(),
        "###.#...#..#.##.######..".chars().collect(),
        ".#.#.###.##.##.#..#.##..".chars().collect(),
        ".####.###.#...###.#..#.#".chars().collect(),
        "..#.#..#..#.#.#.####.###".chars().collect(),
        "#..####...#.#.#.###.###.".chars().collect(),
        "#####..#####...###....##".chars().collect(),
        "#.##..#..#...#..####...#".chars().collect(),
        ".#.###..##..##..####.##.".chars().collect(),
        "...###...##...#...#..###".chars().collect(),
        ];    
    assert_eq!(count_snakes(v),273);
}

//check for r,l,u,d if there is at least one matching hash
//remove if not