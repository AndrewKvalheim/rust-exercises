use ocr_numbers as ocr;

#[test]
#[rustfmt::skip]
fn empty_input() {
    let input = "".to_string();

    assert_eq!(Ok("".to_string()), ocr::convert(&input));
}

#[test]
#[rustfmt::skip]
fn nonuniform_group_length() {
    let input = "    _ \n".to_string() +
                "  | _|\n" +
                "  ||_ \n" +
                "      \n" +
                " _     _  _ \n" +
                " _||_||_ |_ \n" +
                " _|  | _||_|\n" +
                "            \n" +
                " _  _  _ \n" +
                "  ||_||_|\n" +
                "  ||_| _|\n" +
                "         ";
    assert_eq!(Ok("12,3456,789".to_string()), ocr::convert(&input));
}

#[test]
#[rustfmt::skip]
fn missing_columns() {
    let input = " _    \n".to_string() +
                "| |  |\n" +
                "|_|  |\n" +
                "   ";

    assert_eq!(Err(ocr::Error::InvalidColumnCount(3)), ocr::convert(&input));
}

#[test]
#[rustfmt::skip]
fn extra_columns() {
    let input = " _    \n".to_string() +
                "| |  |\n" +
                "|_|  |\n" +
                "         ";

    assert_eq!(Err(ocr::Error::InvalidColumnCount(9)), ocr::convert(&input));
}
