# no_discrimination

Normal rust enums can be created with specified with custom discriminants:

```rust
enum MyEnum {
    A = 1,
    B = 2
}
```

The backing integer type can even be specified specifically:

```rust
#[repr(u8)]
enum MyByteEnum {
    A = 1,
    B = 2
}
```

However conversion between integers and the enum can be clunky and could
potentially result in an error if the enum cases are not completely covered.
Even for u8 enums, covering all 256 possibilities is untenable.

To fix this you can apply the no_discrimination attribute to enums:

```rust
#[no_discrimination_bits(u8, 1)]
enum MyDiscriminantEnum {
    A = 0,
    B = 1
}
```

where the first argument is the backing integer type and the second argument is
how many least-significant bits of the backing integer to interpret as the enum.

You can also specify if an integer is supposed to be zero'd except for the enum
fields when converting it to the enum:

```rust
#[no_discrimination_bits(u8, 1)]
enum MyDiscriminantEnum {
    A = 0,
    B = 1
}
```

You do not have to specify every possible value for the enum you are specifying,
but if not you need to add an un-valued field named Default

```rust
#[no_discrimination_bits(u8, 2)]
enum MyDiscriminantEnum {
    A = 0,
    B = 1,
    Default
}
```

The default variant must be last if it is going to exist at all. The
no_discrimination attribute also provides two functions for each enum it is applied
to

```rust
let a: u8 = MyDiscriminantEnum::A.to_int();
let b: MyDiscriminantEnum = MyDiscriminantEnum::from_int(a);
```

The functions are guaranteed to succeed, hence the requirements on Default
fields when necessary as the enum is defined.