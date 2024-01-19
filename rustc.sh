binaryName=awk '{print substr($0, 1, length($0)-3)}'
echo binaryName
rustc $0
if [ $? -eq 0 ]; then
    mv binaryName bin/binaryName
else
    echo "rustc failed."
fi


