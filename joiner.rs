use std::os;
use std::io::File;

fn main() {
	let args: ~[~str] = os::args();
	if args.len() != 3 {
		println!("Usage: {:s} <file1> <file2>", args[0]);
	} else {
		let ref fname1 = args[1];
		let ref fname2 = args[2];
		let path1 = Path::new((*fname1).clone());
		let path2 = Path::new((*fname2).clone());
		let msg_file1 = File::open(&path1);
		let msg_file2 = File::open(&path2);

		match(msg_file1, msg_file2) {
			(Some(mut msg1), Some(mut msg2)) => {
				let msg_bytes1: ~[u8] = msg1.read_to_end();
				let msg_bytes2: ~[u8] = msg2.read_to_end();
				let joined_file = File::create(&Path::new("message.joined"));

				match (joined_file) {
					Some (joined_file) => {
						join(msg_bytes1, msg_bytes2, joined_file);
					} ,
					None => fail!("Error opening final file!") ,
				}
			} ,
			(Some(_), _) => fail!("Error opening share file 1: {:s}", *fname1) ,
			(_, Some(_)) => fail!("Error opening share file 2: {:s}", *fname2) ,
			(_, _) => fail!("Error opening file 1: {:s} and file 2: {:s}", *fname1, *fname2) ,
		}
	}

}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
	let mut ret = ~[];
	for i in range(0, a.len()) {
		ret.push(a[i] ^ b[i]);
	}
	ret
}

fn join(msg_bytes1 : &[u8], msg_bytes2 : &[u8], mut joined_file : File) {
	let message_bytes = xor(msg_bytes1, msg_bytes2);
	joined_file.write(message_bytes);
}