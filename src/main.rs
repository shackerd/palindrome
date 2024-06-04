use std::env;

#[cfg(test)]
mod tests;

// todo: enrich with diacritics
const WORD_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyz";
const MATCH_SENSIBILITY: &'static usize = &3;

fn main() {
    let args: Vec<String> =
        env::args().collect::<Vec<_>>()[1..].to_vec();

    if args.len() == 0{
        println!("Input cannot be empty");
    }

    for i in 0..args.len() {

        let arg: String =
            args[i].to_owned();

        let result: String = match is_palindrome(arg.to_owned())
        {
            Some(s) => {
                format!("Yea! Found a palindrome {} for {}", s, arg)
            }
            None => {
                format!("Crap! Did not find any palindrome for {}", arg)
            }
        };

        println!("{}", result);
    }
}

#[doc = "removes dummy chars such like space, etc."]
fn sanitize_string(str: String) -> String {

    let a_ch: Vec<char> =
        WORD_CHARS.chars().collect::<Vec<_>>();

    let v_str: Vec<char> = str.chars().collect::<Vec<_>>();
    let mut res: String = String::new();

    for c in v_str {

        if !a_ch.contains(&c.to_ascii_lowercase()) {
            continue;
        }

        res.push(c);
    }

    res
}

#[doc = "tries to find a palindrome in the given string, will match with a minimum of 3 chars (default), see MATCH_SENSIBILITY const"]
fn is_palindrome(str: String) -> Option<String> {

    let s_str: String =
        sanitize_string(str.to_owned());

    let sanitized: bool = s_str != str;

    let v_rstr: Vec<char> =
        reverse_string(s_str.clone())
            .unwrap()
            .chars()
            .collect::<Vec<_>>();

    let v_str: Vec<char> =
        s_str.chars().collect::<Vec<_>>();

    let mut res: String = String::new();
    let mut m_cnt: usize = 0;
    let mut m_off: usize = 0;
    let mut m_len: i32 = 0;
    let mut fnd: bool = false;

    for i in 0..v_str.len() {

        let s: char = v_str[i];
        let r: char = v_rstr[i];

        if s.eq_ignore_ascii_case(&r) {
            m_cnt = m_cnt + 1;

            if m_cnt == *MATCH_SENSIBILITY {
                m_off = i - (m_cnt - 1);
                fnd = true;
            }

            m_len = m_len + 1;

        } else {
            m_cnt = 0;

            if m_cnt > 1 {
                break;
            }
        }

    }

    if fnd {
        for i in 0..m_len as usize{
            res.push(v_str[i + m_off]);
        }

        if sanitized {
            let resut = get_string_from_sanitized(res, str);
            res = resut
        }

        return Some(res)
    }

    None

}

#[doc = "reverses a string"]
fn reverse_string(str: String) -> Result<String, ()> {

    let mut result:String = String::new();

    for s in str.chars().rev() {
        result.push(s);
    }

    Ok(result)
}

#[doc = "get the matching part of sanitized string in the source string, ignoring non-word chars"]
fn get_string_from_sanitized(s_str: String, str: String) -> String {

    let v_s_str: Vec<char> =
        s_str.chars().collect::<Vec<_>>();

    let v_str: Vec<char> =
        str.chars().collect::<Vec<_>>();

    let a_ch: Vec<char> =
        WORD_CHARS.chars().collect::<Vec<_>>();

    let mut m_off: usize = 0;
    let mut m_cnt: usize = 0;
    let mut m_end: usize = 0;
    let mut c_s_str: usize = 0;


    for i in 0..v_str.len() {

        let c: char = v_str[i];

        if !a_ch.contains(&c.to_ascii_lowercase()) {
            continue;
        }

        let cs: char = v_s_str[c_s_str];

        if c == cs
        {
            m_cnt = m_cnt + 1;
            c_s_str = c_s_str + 1;

            if m_cnt == *MATCH_SENSIBILITY {
                m_off = i - (m_cnt - 1);
            }
        }
        else {
            m_cnt = 0;
            c_s_str = 0;
        }

        if c_s_str >= v_s_str.len() {
            m_end = i + 1;
            break;
        }
    }

    let res: String = v_str[m_off..m_end].iter().collect();
    res
}
