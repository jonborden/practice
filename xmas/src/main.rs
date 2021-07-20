// rust homework, ch 3.5 "control flow"
// 7.20.21
// 'Print the lyrics to the Christmas Carol "The Twelve Days of Christmas," taking advantage of the repetition in the song.'

fn main() {
	let mut counter = 0;
	let mut date = "st";
	
	loop {
		counter += 1;

		if counter == 2 {
			date = "nd";
		}
		// is this sequence better done with 'else if's instead of 'if's? it seems implied as results are exclusive
		if counter == 3 {
			date = "rd"
		}
		if counter == 4 {
			date = "th"
		}
		if counter > 12 {
			break;
		}
	    
		println!("On the {}{} day of Christmas, my true love gave to me,", counter, date);
		
		// there's probably a wait to add a 'wait' or 'pause' command somehow to delay the line printing s.t. it feels more like an actual song
		if counter > 11 {
			println!("twelve drummers drumming");
		}
		if counter > 10 {
			println!("eleven pipers piping");
		}
		if counter > 9 {
			println!("ten lords a'leaping");
		}
		if counter > 8 {
			println!("nine ladies dancing");
		}
		if counter > 7 {
			println!("eight maids a'milking");
		}
		if counter > 6 {
			println!("seven swans a'swimming");
		}
		if counter > 5 {
			println!("six geese a'laying");
		}
		if counter > 4 {
			println!("fiiiiiiiiiiiiiive goooooooooooooooold riiiiiiiiiiiiiiings");
		}
		if counter > 3 {
			println!("four calling birds");
		}
		if counter > 2 {
			println!("three french hens");
		}
		if counter > 1 {
			println!("two turtle doves, and");
		}
		if counter > 0 {
			println!("a partridge in a pear tree");
			println!("");
		} 
	};
}
