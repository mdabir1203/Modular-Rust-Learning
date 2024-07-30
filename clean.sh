find . | grep "Cargo.toml$" |                     # Find all cargo.toml files \
sed 's#/[^/]*$##' |                               # Remove the filename leaving us with the directories containing cargo.toml files \
xargs -L1 printf "cd \"%s\"; cargo clean; rm -f Cargo.lock; cd -\n" |   # Print "cd path/to/crate; cargo clean; rm -f Cargo.lock; cd -" \
bash