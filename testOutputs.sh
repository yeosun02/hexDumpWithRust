cargo build --release

#### Should not print out anything ####
# Dealing with small file
./target/release/hexdump -n 7 sample.txt > myOut.txt
hexdump -n 7 sample.txt > realOut.txt
diff -w myOut.txt realOut.txt

# Dealing with larger file
./target/release/hexdump -n 256 ./src/main.rs > myOut.txt
hexdump -n 256 ./src/main.rs > realOut.txt
diff -w myOut.txt realOut.txt

# Dealing with larger file with odd num
./target/release/hexdump -n 511 ./src/main.rs > myOut.txt
hexdump -n 511 ./src/main.rs > realOut.txt
diff -w myOut.txt realOut.txt

#### Should print out "hexdump: bad length value" ####
./target/release/hexdump -n -2 ./src/main.rs
./target/release/hexdump -n dd ./src/main.rs

#### Should print out "Usage: hexdump [-n Len] FILE" ####
./target/release/hexdump 
./target/release/hexdump -n ./src/main.rs
./target/release/hexdump -k 3 ./src/main.rs
./target/release/hexdump -k 3 ./src/main.rs sample.txt

#### Should print out error message according to its error ####
./target/release/hexdump -n 5 dumb
