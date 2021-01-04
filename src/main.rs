// Gejza bol na join3rovom kurze rustu a po 40 hodinách naprogramoval toto
// program funguje takmer ako chceme, ale ako vidíte je škaredý
// vašou úlohou je upraviť program tak, aby bola radosť ho čítať a upravovať
// Zopár pointerov (haha), ktoré Gejza nenasledoval, ale vy by ste mali
// KISS - keep it simple stupid
// DRY - do not repeat yourself

fn main()
{
  println!("Zadaj meno:");
  let mut a = String::new();
  std::io::stdin().read_line(&mut a);
  println!("Zadaj pohlavie (M/Z):");
  let mut b = String::new();
  std::io::stdin().read_line(&mut b);if b.trim()=="M"||b.trim()=="m"||b.trim()=="Z"||b.trim()=="z"{
  println!("Zadaj den narodania");
  let mut c = String::new();
  std::io::stdin().read_line(&mut c);
      let x: i32 = c.trim().parse().unwrap();
       println!("Zadaj mesiac narodenia");
       let mut d = String::new();       
       std::io::stdin().read_line(&mut d);
      let y: i32 = d.trim().parse().unwrap();
       println!("Zadaj rok narodenia");
       let mut input = String::new();
      std::io::stdin().read_line(&mut input);
      let z: i32 = input.trim().parse().unwrap();
      if 2021 - z < 18 {
            println!("Neplnolety, nemozeme pokracovat"); } else
             {
          let a = a.trim();
           let s = b.trim();
           let d = c.trim();
            let f = d.trim();
           let g = input.trim();
           let y = y.to_string();
           let g = vec![a,s,d,&y,g];
           let a: String = g.iter().map(|a | a.trim()).fold(String::new(), |p,a| format!("{},{}", p, a));
           std::fs::write("datafile.csv", a);
           println!("ok");
      }
    }
    else
    {
        println!("Nespravny udaj");}}
