                                                                                                        
                                use                                                                     
                                                                                                        
                             std::error::Error;  use                                                    
                            std::fs::File;          use                                                 
                            std::io::{Read,  Write};  pub                                               
                            struct  Conf  {  ascii:  String,                                            
                             code:  String,  output: String, }                                          
                             impl  Conf  { pub fn new(a: &str, c:                                       
                              &str,  o:  &str)  ->  Conf  {  Conf  {                                    
                             ascii:       String::from(a),       code:                                  
                          String::from(c),  output: String::from(o), } }                                
                         }  pub  fn  run(cfg:  Conf)  -> Result<(), Box<dyn                             
                         Error>>  { let mut asc = File::open(cfg.ascii)?; let                           
                        mut   cod   =  File::open(cfg.code)?;  let  mut  out  =                         
                        File::create(cfg.output)?;  let mut code = String::new();                       
                         cod.read_to_string(&mut  code)?; let mut codewords = code                      
                         .split_whitespace()    .peekable();   let   mut   ascii   =                    
                          String::new();  asc.read_to_string(&mut  ascii)?;  let  mut                   
                           output  =  String::new();  let  mut  canwrite  = 0; for c in                 
                             ascii.chars()  {  if c.is_whitespace() { if canwrite > 0 {                 
                              match  write_justified(&mut  codewords,  &mut  canwrite) {                
                                 StringResult::Some(s)       =>      output.push_str(&s),               
                                 StringResult::Empty(s)  => { output.push_str(&s); break; }             
                              }  }  output.push(match c { '\n' => '\n', _ => ' ', }); } else            
                           {   canwrite   +=   1;  }  }  match  codewords.next()  {  None  =>           
                         out.write_all(output.as_bytes())?,   Some(_)   =>   eprintln!("Error,          
                      ascii  art  has  insufficient  characters!\nTry  using  a larger one (or          
                  duplicate  it)\n{} words couldn'be be written", codewords.count()), }; Ok(())         
               }     enum     StringResult     {     Some(String),     Empty(String),    }    fn        
            write_justified(codewords:  &mut  std::iter::Peekable<std::str::SplitWhitespace>, n:        
          &mut  usize)  ->  StringResult  {  let  mut string = String::with_capacity(*n); let mut       
        words  =  Vec::new(); while codewords.peek().is_some() && codewords.peek().unwrap().len()       
      +  1  <=  *n  {  let  w  = codewords.next().unwrap(); *n -= w.len() + 1; words.push(w) }; if      
      words.len()  < 2 { if words.len() == 1 {    string.push_str(words[0]);  string.push('  '); }      
      write_spaces(&mut  string,  *n);  }             else  {  let  spaces = words.len() - 1; *n +=     
      1;  let  div  =  *n / spaces + 1;                  let  mut  rem = *n % spaces; let mut w_iter    
       =               words.iter();                      string.push_str(w_iter.next().unwrap());      
         for  w  in  w_iter  { if                          rem  >  0 { rem -= 1; string.push(' '); }    
           write_spaces(&mut                                string,  div);  string.push_str(w);  } }    
         *n  = 0; match                                     codewords.peek()     {     Some(_)     =>   
                                                            StringResult::Some(string),    None    =>   
                                                            StringResult::Empty(string),   }   }   fn   
    write_spaces(s:     &mut                                String,  x:  usize)  {  for  _  in 0..x {   
   s.push('     ');    }    }                               #[cfg(test)]     mod    tests    {    use   
   super::{StringResult,   *};                              impl     StringResult     {     pub    fn   
   content(&self)  ->  String {                            match  self  {  StringResult::Some(s)  =>    
   s.clone(),                                             StringResult::Empty(s)  =>  s.clone(), } }    
    }          #[test]          fn                       test_write_justified  ()  {  let mut iter =    
    "testing  out the justified text                   fitter".split_whitespace().peekable();           
    assert_eq!(write_justified(&mut                  iter,           &mut          40).content(),       
    String::from("testing     out     the        justified   text   fitter"));  let  mut  iter  =       
      "testing            out            the           justified           text           fitter        
      withareallylongwordwhoa".split_whitespace().peekable();   assert_eq!(write_justified(&mut         
       iter,  &mut  46).content(),  String::from("testing out the justified text fitter")); let         
       mut           iter          =          "thisisalongword".split_whitespace().peekable();          
        assert_eq!(write_justified(&mut  iter, &mut 8).content(), String::from(" ")); let mut           
         iter               =               "thisisalongword".split_whitespace().peekable();            
           assert_eq!(write_justified(&mut         iter,         &mut        18).content(),             
            String::from("thisisalongword               "));              }              }