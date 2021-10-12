# Credits
Once again in this project, I'm crediting Philipp Oppermann and
[his blog](https://os.phil-opp.com/minimal-rust-kernel) for giving an important advice
on target feature specification. Namely, disabling `mmx`, `sse`, and enabling `soft-float`
seems to have resolved the obscure crash in #1.
