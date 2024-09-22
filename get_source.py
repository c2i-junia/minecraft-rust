#!/bin/python
import os

def collect_rust_files(directory, output_file):
    with open(output_file, 'w') as outfile:
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    outfile.write(f"---- {file_path} ----\n\n") 
                    with open(file_path, 'r') as infile:
                        outfile.write(infile.read())
                    outfile.write("\n\n") 

if __name__ == "__main__":
    source_directory = "./src"
    output_file = "source_code.txt"

    collect_rust_files(source_directory, output_file)
    print(f"source code collected in {output_file}")
