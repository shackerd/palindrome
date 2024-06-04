use super::*;

#[test]
fn reverse_string_tests() {
    let res: String = reverse_string(String::from("test")).unwrap();    
    assert_eq!(res, "tset");
}

#[test]
fn is_palindrome_tests() {

    let res1: String = is_palindrome(String::from("wow")).unwrap();
    assert_eq!(res1, "wow");

    let res2: String = is_palindrome(String::from("Saippuakivikauppias")).unwrap();
    assert_eq!(res2, "Saippuakivikauppias");        

    let res3: Option<String> = is_palindrome(String::from("test"));
    assert_eq!(res3, None);

    let res4: String = 
        is_palindrome(String::from("Sir, I demand, I am a maid named Iris")).unwrap();
    assert_eq!(res4, "Sir, I demand, I am a maid named Iris");

    let res5: String = 
        is_palindrome(String::from("Satire: Veritas")).unwrap();
    assert_eq!(res5, "Satire: Veritas");


    // let res6: String = 
    //     is_palindrome(String::from("Dr Awkward & Olson in Oslo")).unwrap();
    // assert_eq!(res6, "Dr Awkward & Olson in Oslo");
}

#[test]
fn sanitize_string_tests() {
    
    let res: String = sanitize_string(String::from("foo bar"));
    assert_eq!(res, "foobar");

    let res2: String = sanitize_string(String::from("foo, bar"));
    assert_eq!(res2, "foobar");
}

#[test]
fn get_string_from_sanitized_tests() {
    
    let res: String = 
        get_string_from_sanitized(
            String::from("SirIdemandIamamaidnamedIris"), 
            String::from("Sir, I demand, I am a maid named Iris")
        );
        
    assert_eq!(res, "Sir, I demand, I am a maid named Iris");        

    let res2: String = 
        get_string_from_sanitized(
            String::from("SatireVeritas"), 
            String::from("Satire: Veritas")
        );
    assert_eq!(res2, "Satire: Veritas");
}
