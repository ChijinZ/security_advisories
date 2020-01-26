# Multiple vulnerabilities in tinyexr
There are multiple vulnerabilities in tinyexr (git repository: https://github.com/syoyo/tinyexr, Latest commit 16aba30, on Jun 16, 2018).

git log

    commit 16aba303c20ab7712830b36afa97b770a84e23b7
    Author: Syoyo Fujita <syoyo@lighttransport.com>
    Date:   Sat Jun 16 20:59:37 2018 +0900

## segmentation fault (**CVE-2018-12688**)

I build tinyexr with gcc. When testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/tinyexr_16aba30/segmentation_fault) is input into test_tinyexr (command: ./test_tinyexr testcase), a segmentation fault is triggered. GDB provides information as follow:

    #0  0x000000000040f031 in tinyexr::wav2Decode(unsigned short*, int, int, int, int, unsigned short) ()
    #1  0x000000000041223e in tinyexr::DecompressPiz(unsigned char*, unsigned char const*, unsigned long, unsigned long, int, _EXRChannelInfo const*, int, int) ()
    #2  0x00000000004125ad in tinyexr::DecodePixelData(unsigned char**, int const*, unsigned char const*, unsigned long, int, int, int, int, int, int, int, int, unsigned long, unsigned long, _EXRAttribute const*, unsigned long, _EXRChannelInfo const*, std::vector<unsigned long, std::allocator<unsigned long> > const&) ()
    #3  0x000000000041639b in tinyexr::DecodeChunk(_EXRImage*, _EXRHeader const*, std::vector<unsigned long long, std::allocator<unsigned long long> > const&, unsigned char const*, unsigned long) ()
    #4  0x0000000000416cee in tinyexr::DecodeEXRImage(_EXRImage*, _EXRHeader const*, unsigned char const*, unsigned char const*, unsigned long, char const**) ()
    #5  0x0000000000418a99 in LoadEXRImageFromMemory ()
    #6  0x000000000041887a in LoadEXRImageFromFile ()
    #7  0x00000000004170d0 in LoadEXR ()
    #8  0x000000000041f2e1 in main ()

## Assert failure (**CVE-2018-12687**)
When testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/tinyexr_16aba30/assert_failure) is input into test_tinyexr (command: ./test_tinyexr testcase), a assert-failure problem is triggered in tinyexr.h:9589.

    test_tinyexr: tinyexr.h:9589: bool tinyexr::DecodePixelData(unsigned char**, const int*, const unsigned char*, size_t, int, int, int, int, int, int, int, int, size_t, size_t, const EXRAttribute*, size_t, const EXRChannelInfo*, const std::vector<long unsigned int>&): Assertion `ret' failed.
    Aborted