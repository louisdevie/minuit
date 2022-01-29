if cargo build --example demo
then
    ./target/debug/examples/demo
else
    echo "=> Compilation failed, aborting"
fi