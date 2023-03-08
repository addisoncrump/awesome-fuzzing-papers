# Fuzzing Summaries

Summaries and relationship data for fuzzing papers.

## General idea

Exploring relationships between papers can be hard, and it's not always easy to
find papers you know you've read and can no longer find. I have been struggling
to keep track of fuzzing papers recently and figure others are too, so I decided
to make this.

This repository contains summaries, bibliographies, and referrer/referee data
for as many fuzzing papers and works as I could read and summarise.

## "I want a paper summarised!"

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

## Adding a new paper

The following fields should be present:
 - author
 - 