# Description

This project lets you compress files using Huffman encoding.
The compression rate is not too bad, but it is not very fast.
Something like `zip` or `gzip` is currently a lot faster.
I do not recommend using this for anything serious.

The documentation for the library can be found [here](doc/huffman/index.html).

## Building

The project can be built as expected using cargo.
See `cargo --help` for more information.

## Usage

Check out `huffman --help` for more information.

## Huffman Encoding

Here is a short explanation of how Huffman encoding works.

Given the string `aaaaabbbc`, we have the following character frequencies:

```
a: 5
b: 3
c: 1
```

To start off, each one gets its own node.

```
a:5    b:3    c:1
```

We then combine the two lightest nodes to get the following.

```
a:5    bc:4
      /    \
     0      1
     |      |
    b:3    c:1
```

We repeat the process again to get our final tree.

```
   abc:9
  /     \
 0       1
 |       |
a:5    bc:4
      /    \
     0      1
     |      |
    b:3    c:1
```

We can now find the code of each character by looking at the path to them.
We can also decode a sequence of bits by following the corresponding branch.
This gives us the following codes.

```
a: 0
b: 10
c: 11
```

Notice that no code can be a prefix of another, meaning that the bit-sequence can never be ambiguous.
This is due to the structure of the tree: For a code to be a prefix of another, it would have to appear in the path of another.
This can never happen since the characters are in the leaves.

The easy part is now done, and we can start the slightly more difficult task of taking these sequences of ones and zeros and properly encoding them as actual bits, but that's my job.
