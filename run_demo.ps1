cargo build --example demo

if ($?)
{
    .\target\debug\examples\demo.exe
}
else
{
    "=> Compilation failed, aborting"
}