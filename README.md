# Awesome Fuzzing Papers

Summaries and relationship data for fuzzing papers. Note that not all of these
papers have been selected for incredible impact, but instead to try to collect
as many papers as possible with as much context as possible in order to expand
the public knowledge of fuzzing.

Paper summaries are written to be as close to plain English as possible, and
glossary entries will be provided for common words/acronyms used in fuzzing.
This repository, and the (soon-to-be) associated website, serve as a means to
make fuzzing literature as accessible and explorable as reasonably possible.

## General idea

Exploring relationships between papers can be hard, and it's not always easy to
find papers you know you've read and can no longer find. I have been struggling
to keep track of fuzzing papers recently and figure others are too, so I decided
to make this.

This repository contains summaries, bibliographies, and referrer/referee data
for as many fuzzing papers and works as I could read and summarise. Note that
not everything must be an _academic_ paper; tool reports/whitepapers are also
on the table, provided that they do not conflict with any requirements specified
in the final section of this README.

## "I want a paper summarised!"

Thank you for recommending a paper!

At least the following fields should be present:
- title
- author
- url
- keywords (not necessarily the original keywords; TODO refer to the keyword list)
- crossref (used slightly incorrectly; should be a comma-separated list of referenced papers)
  - If you can also update existing entries which reference this paper,
  please update their crossrefs!
  - TODO figure out how to derive this; this is O(N^2) in person-work
  - anystyle doesn't seem to work in many cases :(


Please open an issue with the relevant issue template! :)

## What content should be present

This repository should contain the following information:
 - Paper names, links (archival required), and summaries, where
   - Summaries are unbiased and present the work without opinion
   - Authors will be informed and offered the opportunity to correct summaries,
     with author-requested alterations recorded on the page containing the
     summary as well as original content
   - Summaries which have not been confirmed by the authors will be marked
 - Keyword groups
 - Referrer/referee information
 - Additional, clearly separated information key to understanding the context of 
   a paper or work, e.g.,
   - Subsequent findings that contradict paper results/findings with off-site
     sources as a reference, e.g. [Brendan Dolan-Gavitt's secondary analysis of NEUZZ]
     (archival required)
   - References to papers which significantly supersede the work (internal links
     only)
   - Appearances in other non-academic/unofficially published works, e.g. [MOpt]
     as used in [AFL++] (archival required)
   - Empirical information regarding the use of the work with off-site sources
     as a reference, e.g. [epi051's experience with LibAFL] (archival required)
   - Comments regarding paper impact, relevance, and other subjective measures
     with off-site sources as a reference (archival required)

It should _not_ contain:
 - Unsubstantiated or verifiably false claims regarding papers
 - Attacks on authors or biased judgements of quality
 - Summaries which contain commentary other than a literal interpretation of the
   original content of the paper
 - Summaries of work which are not publicly available
 - Summaries of work out of corporate or financial interest
 - Summaries of work which have not been peer-reviewed

[MOpt]: https://www.usenix.org/conference/usenixsecurity19/presentation/lyu
[AFL++]: https://github.com/AFLplusplus/AFLplusplus
[epi051's experience with LibAFL]: https://epi052.gitlab.io/notes-to-self/blog/2021-11-01-fuzzing-101-with-libafl/
[Brendan Dolan-Gavitt's secondary analysis of NEUZZ]: https://twitter.com/moyix/status/1513608538500870154
