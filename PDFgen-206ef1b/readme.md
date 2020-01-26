# a vulnerability in PDFgen (**CVE-2018-11363**)
There is a vulnerability in pdffigures (git repository: https://github.com/AndreRenaud/PDFGen, 206ef1b  on Apr 4, 2018).

git log

    commit 206ef1b560efed48aabcb9374ea0e6e832b59f7e
    Author: Andre Renaud <arenaud@designa-electronics.com>
    Date:   Wed Apr 4 08:59:06 2018 +1200

I set up a driver to fuzz the pdffigures libarary, and a crash was triggered due to heap-buffer-overflow. 
The driver is:

    #include "pdfgen.h"
    #include <stdio.h>
    int main(int argc, char *argv[])
    {
        if (argc != 2) {
            return 0;
        }
        // printf("%s\n",argv[1]);
        struct pdf_info info = {.creator = "1",
                                .producer = "2",
                                .title = "3",
                                .author = "4",
                                .subject = "5",
                                .date = "6"};
        struct pdf_doc *pdf = pdf_create(PDF_A4_WIDTH, PDF_A4_HEIGHT, &info);
        pdf_append_page(pdf);
        pdf_add_jpeg(pdf, NULL, 100, 500, 50, 150, argv[1]);
        pdf_save(pdf, "fuzz.pdf");
        pdf_destroy(pdf);
        return 0;
    }

And compile it as follow:

    CC=afl-clang-fast
    CFLAGS=-g -Wall -pipe --std=c1x -O3 -pedantic -Wsuggest-attribute=const -Wsuggest-attribute=format -Wclobbered -Wempty-body -Wignored-qualifiers -Wmissing-field-initializers -Wold-style-declaration -Wmissing-parameter-type -Woverride-init -Wtype-limits -Wuninitialized -Wunused-but-set-parameter -fprofile-arcs -ftest-coverage -fsanitize=leak,address
    LFLAGS=-fprofile-arcs -ftest-coverage -fsanitize=leak,address


    default: testpdf

    testpdf: testpdf.o pdfgen.o
        $(CC) -o testpdf pdfgen.o testpdf.o $(LFLAGS)
    %.o: %.c Makefile
        $(CC) -c -o $@ $< $(CFLAGS)

When the testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/PDFgen-206ef1b/crash.jpg) was input (command: ./testpdf testfile), a crash was trigger.

It seems that it try to access a memory out of array *data* in pdfgen.c:2015.

    if (i + 3 < data_size && data[i] == 0xFF && data[i+`1] == 0xD8 &&
                data[i+2] == 0xFF && data[i+3] == 0xE0)`

AddressSanitizer provided information as below:

    ==5926==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x6020000000b6 at pc 0x000000528b63 bp 0x7ffeb9450530 sp 0x7ffeb9450528
    READ of size 1 at 0x6020000000b6 thread T0
        #0 0x528b62 in jpeg_size path/to/PDFGen/pdfgen.c:2015:12
        #1 0x528b62 in pdf_add_raw_jpeg path/to/PDFGen/pdfgen.c:2079
        #2 0x528b62 in pdf_add_jpeg path/to/PDFGen/pdfgen.c:2208
        #3 0x52b59c in main path/to/PDFGen/testpdf.c:17:5
        #4 0x7f568074f82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #5 0x41a418 in _start (path/to/PDFGen/testpdf+0x41a418)