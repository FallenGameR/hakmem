# Bit manipulation

## turn_off_rightmost_1bit

`x & (x - 1)`

```text
 10101000
 10100000
 10000000
 00000000 - stays the same
 00100000 -> pow2
 00000000 -> all zeros
```

## turn_off_rightaligned_1bit_group

`x & (x + 1)`

```text
 10100111
 10100000 - stays the same
 00111111 -> pow2 - 1
 00000000 -> all zeros
 11111111 -> all ones
```

## turn_off_rightmost_1bit_group

`x & (x + (x & -x))`

```text
 01011100
 01000000
 00000000 - stays the same
 can be used to detect if number is pow2-pow2
```

## turn_on_rightmost_0bit

`x | (x + 1)`

```text
 10100111
 10101111
 10111111
 11111111 - stays the same
```

## turn_on_rightaligned_0bit_group

`x | (x - 1)`

```text
 10101000
 10101111 - stays the same
```

## mask0_rightmost_1bit

`!x | (x - 1)`

```text
 10101000
 11110111
 11111110
 11111101 - alternates the last two
all ones if there is none
 00000000
 11111111
```

## mask0_rightaligned_1bit_group

`!x | (x + 1)`

```text
 10100111
 11111000
 11111111
 00000000 - alternates the last two
```

## mask1_rightmost_0bit

`!x & (x + 1)`

```text
 10100111
 00001000
 00000001
 00000010 - alternates the last two
all zeros if there is none
 11111111
 00000000
```

## mask1_rightaligned_0bit_group

`!x & (x - 1)`

```text
 10101000
 00000111
 00000000
 11111111 - alternates the last two
```

## mask1_rightmost_1bit

`x & -x`

```text
 10101000
 00001000 - stays the same
all zeros if there is none
 00000000
 00000000
```

## mask1_extended_rightaligned_0bit_group

`x ^ (x - 1)`

```text
 10101000
 00001111
 00000001 - stays the same
```
