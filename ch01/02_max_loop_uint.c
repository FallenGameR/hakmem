for( uint i=0; ; i++ )
{
    // code here
    // Otherwise will overflow and e2 will still be true
    if( i == 0xFFFFFFFF ) { break }
}

uint i = 0xFFFFFFFF
do
{
    i += 1
    // code here
}
while( i != 0xFFFFFFFF );
