A super simple translator for a cipher from a puzzle. shift based on character index (ignoring space) + word length - 1. vowels and consonants shift on sperate tapes (vowels stay vowels, consonants stay consonants. y is always a vowel here).

by default it'll encode your input, pass -d or --decode for decoding instead.

this makes a lot of assumptions about the input text and is probably really easy to crash if you pass it something unexpected.


Example:

- `$ printf 'example plaintext' | translator` outputs `anozbva tpaensotp`
- `$ printf 'anozbva tpaensotp' | translator -d` outputs `example plaintext`
