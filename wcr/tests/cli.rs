// test for default : lines, words and bytes
// - DEFAULT ORDER : lines, words, byte/characters

// test [flags] => -l : lines
// test [flags] => -c : bytes
// test [flags] => -w : words
// test [flags] => -m : characters
// test [flags] => -mc : bytes
// test [flags] => -cm : characters
// test [flags] => -cw | -wc : DEFAULT order regardless of flags
//
// test [read from STDIN] does NOT print a filename
//
// [multi-files] => [total] # lines | words | bytes for all inputs
// [file-error] => Nonexistent files note wraning to STDERR as files process
