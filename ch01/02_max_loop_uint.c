for( uint i=0; ; i++ )
{
    // Otherwise will overflow and e2 will still be true
    if( i == 0xFFFFFFFF ) { break }
}