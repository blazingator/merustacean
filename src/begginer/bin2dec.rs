use regex::Regex;

pub fn bin2dec(bin_num_string: String){
  println!("Converter binário para decimal");

  if bin_num_string.len() > 8 {
    println!("No maximo oito digitos");
    return;
  }

  if bin_num_string.parse::<i32>().unwrap_or(0) == 0 {
    println!("{} = 0", bin_num_string);
    return;
  }

 // let mut count = 0u32;
  let mut out: i32 = 0;


  let re = Regex::new(r"[0,1]").unwrap();

  for cap in re.captures_iter(&bin_num_string) {
    let pos = &cap[0].parse::<i32>().unwrap();
    out = out * 2 + pos;
  }

  println!("
    bin - dec
    {} = {}
  ",bin_num_string,out);

}

