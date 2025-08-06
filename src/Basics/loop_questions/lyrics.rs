fn main() {

    let lyrics_diff_arr: [&str ; 12] = ["","Two turtle doves", "Three French hens", "Four calling birds",
    "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking",
    "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    let days : [&str ; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    print!("On the first day of Christmas\nMy true love gave to me\nA partridge in a pear tree\n\n");

    for mut i in 1..12 {

        print!("On the {} day of Christmas\nMy true love gave to me\n", days[i]);

        while i > 0 {
            print!("{}\n", lyrics_diff_arr[i]);
            i -= 1;
        };

        print!("A partridge in a pear tree\n\n");

    }

    println!("\n\nSong ended.\n");

}