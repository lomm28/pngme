## A command line program that lets you hide secret messages in PNG files

*This app is implemented as per guide from [here](https://picklenerd.github.io/pngme_book)*.

### Commands

1. Compile the code and run the executable:
        
        cargo run

2. Run tests:

        cargo test

3. Encode a secret message in file:

        pngme encode <file_path> <chunk_type> <message>

4. Decode a message in file:

        pngme decode <file_path> <chunk_type>

5. Remove a chunk with a secret message from file:

        pngme remove <file_path> <chunk_type>


6. Print info about chunks to console:

        pngme print <file_path>