use regex::Regex;

pub fn has_mention(input: &str) -> bool {
  let re = Regex::new("@.*").expect("Unable to create regex");
  let caps = re.captures(input);
  match caps {
    None => false,
    Some(cap) => true,
  }
}

pub fn ext_user_id_from_mention(input: &str) -> String {
  let re = Regex::new("@.*").expect("Unable to create regex");
  let caps = re.captures(input).expect("Can't capture");
  let res: &str = &caps[0].replace("@!", "").replace("<", "").replace(">", "");
  String::from(res)
}
