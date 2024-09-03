# CLI Password Generator

Command line password generator, in Rust.

## Quickstart

```terminal
$ cargo run --release

pw_length               45
extended_special_chars  true
num_generated           10

\[0dc98m+N3~F!/\?`EPMyJq^xBTW9VC+e[f@z'Z]G*KV
P.0.~TeugOHq9R7Jvit^-5'X6,J6{E:L>sax[CQ-&&bH.
VfQvG~Z12L5cR\Wg[|{{dd^VQ6kgN"q=;ve~P$}{D*C(B
f?iB1IEhlWXFn6U#(NL6<.R.*+Cmbz-I(,3DZWDgv9-Gb
rq8k=frt*1pg,_TS;4W<o,"uyD'_a/8#+0=K<Y<qc;VYd
(tMAbm<PI8E!>?`1z@QrTGqoj(<s*oO;r}Q{5L75))[*F
oHYDWz[}O%c;L,R7#Hf!vn`=<%{V5#5J=|Iwf};R1!*Z{
--^2HKT><+ROF_lJM;`a6=9}Wv$mE5`hx*9R|swFN#'`g
F1V[e6E|\+FSbm"RVXR1&FhW'1j.|:Z";9af">,!hvcIV
V#NZRmign~n}?7=RNE}Y\luz(tR.Hj?"vJOs,$``$XZ=:
```


## Usage

```terminal
$ cargo run --release -- --help

Usage: cli_password_generator [OPTIONS]

Options:
  -l, --length <LENGTH>                                    [default: 45]
  -e, --extended-special-chars <EXTENDED_SPECIAL_CHARS>    [default: true]
  -n, --num-passwords-generated <NUM_PASSWORDS_GENERATED>  [default: 10]
  -h, --help   
```


## Example

```terminal
cargo run --release -- --length 25 --extended-special-chars false --num-passwords-generated 5

pw_length               25
extended_special_chars  false
num_generated           5

d@jZtUtZsmlXt@GhLJd0MS&op
XgFqE#a^!JNunLCv%cnqfICFH
5d$M&%yElGo2td*&LGChk39gH
HXlQhs89GbNOV!NnqU!%yp9ad
qgf6qbQ#a99SzIUlGke9R*ZBY
```


## Special Characters

The option flag ```-e, --extended-special-chars <bool>``` controls allowed special characters.

 - Standard: ```!@#$%^&*```
 - Extended: ```~`()-_+={}[]|\\:;"\',<>./?```

```bash
# --extended-special-chars defaults to true -> standard and extended are used.
$ cargo run --release

# only use standard.
$ cargo run --release -- --extended-special-chars false
```

*Note: With small passwords (<25 characters) that only use the standard set, sometimes no special character appears.*


## Password Best Practices

 - Use a password manager.
 - Use a different password for each website/app. 
 - Use the highest complexity+length password allowed.
 - Clear your clipboard after saving the password.


## Additional Notes

**Distribution**
 - All characters have an equal chance of being selected.

**Generation Method**
 - An array (vector) of 10 million random characters is created. From this, characters are randomly selected.


## About

I made this program because I was dissatisfied with password generators I had used. Subjectively viewing those passwords side-by-side with known high-complexity password generators, clearly showed they were lacking. The true high complexity password generators were local-database-only password managers, and by design choices, made them inconvenient to use solely for password generation. Thus, here we are.


## License

None. It's yours.
