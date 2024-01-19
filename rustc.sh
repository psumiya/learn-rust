# Check if the Rust file name is provided as an argument
if [ -z "$1" ]; then
    echo "Please provide the Rust file name as an argument"
    exit 1
fi

# Extract the file name and extension
file_name=$(basename "$1")
file_name_no_ext=${file_name%.*}

# Compile the Rust file
rustc "$1"
if [ $? -eq 0 ]; then
    # Create the bin directory if it doesn't exist
    mkdir -p bin
    # Move the resulting binary to the bin directory
    mv "$file_name_no_ext" "bin/"
    echo "Compilation successful. Binary is located in the bin directory."
else
    echo "Compilation failed"
fi