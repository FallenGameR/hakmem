for( e1; e2; e3 )
{
    x
}
// xx

e1
while( e2 )
{
    x
    e3
}
// xx


e1
do
{
    x // will be done at least once, without checking e2
    e3
}
while( e2 )
// xx

e1: i = 1
e2: i <= 2
e3: i += 1