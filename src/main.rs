fn main() {
    let u_case_letter = 'A';
    let l_case_letter = 'a';

    println!(
        "{} is uppercase ({}), {} is lowercase ({})", 
        u_case_letter, u_case_letter.escape_unicode().to_string(),
        l_case_letter, l_case_letter.escape_unicode().to_string()
    );
}
