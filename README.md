# One billion row challenge

These tests were run on a machine with r9 5900x, 64gb memory.


* single threaded, no mmap: 29.7s (4.5s to read the file into memory, 24s to process rows and ~150micros to generate the output)

* multi threaded, no mmap: read = 4.682276171s processed = 2.057576429s output_gen = 188.113µs

* multi threaded + mmap: read = 5.75µs processed = 2.144708s output_gen = 276.73µs

