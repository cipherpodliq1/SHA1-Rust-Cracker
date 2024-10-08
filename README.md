# SHA1-Rust-Cracker
 A SHA1 cracker written in rust. From the book Black Hat Rust

The provided Rust code is a simple SHA-1 hash cracker. It attempts to find the original password corresponding to a given SHA-1 hash by using a wordlist. It parses the arguments from the user, it prints the usage in case you do not understand. Validates the arguments by checking if the provided hash length meets the constant which represents the length. 

The program then opens the wordlist file and reads it line by line with the help of a BufReader.

Finally, for each password in the wordlist, it computes the SHA-1 hash of the password and compares it to the given target hash.

If a match is found, the program prints the password and exits successfully.

If no match is found after all lines are checked, the program prints a "NOT FOUND" message and exits.
