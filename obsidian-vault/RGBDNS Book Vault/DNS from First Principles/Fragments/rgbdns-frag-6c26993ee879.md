---
type: "code-fragment"
fragment_id: "rgbdns-frag-6c26993ee879"
source_path: "ietf/draft-khrabrov-dnsop-aname-axfr-00.txt"
code_note: "DNS from First Principles/Code/ietf/draft-khrabrov-dnsop-aname-axfr-00.txt.source"
language: ""
subsystem: "Repository and build"
symbol: "draft-khrabrov-dnsop-aname-axfr-00.txt"
kind: "file"
start_line: 1
end_line: 81
---

# draft-khrabrov-dnsop-aname-axfr-00.txt

- Fragment ID: `rgbdns-frag-6c26993ee879`
- Source file: [[DNS from First Principles/Code/ietf/draft-khrabrov-dnsop-aname-axfr-00.txt.source|ietf/draft-khrabrov-dnsop-aname-axfr-00.txt]]
- Lines: 1-81
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-6c26993ee879", "codeNote": "DNS from First Principles/Code/ietf/draft-khrabrov-dnsop-aname-axfr-00.txt.source", "sourcePath": "ietf/draft-khrabrov-dnsop-aname-axfr-00.txt", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-6c26993ee879" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6c26993ee879: file draft-khrabrov-dnsop-aname-axfr-00.txt

```




Domain Name System Operations                                A. Khrabrov
Internet-Draft                                                QueryGraph
Intended status: Standards Track                            30 July 2026
Expires: 31 January 2027


         Address-specific DNS Aliases (ANAME) and Zone Transfer
                   draft-khrabrov-dnsop-aname-axfr-00

Abstract

   This document defines the ANAME DNS resource record.  ANAME provides
   name-to-name indirection for address queries while allowing other
   resource record types to exist at the same owner name.  It is
   therefore usable at a zone apex.

   This document also defines authoritative processing, TTL and failure
   behavior, DNSSEC considerations, and interoperable transport of ANAME
   records in full and incremental zone transfers.  In particular, each
   ANAME-capable authoritative server resolves the transferred target
   independently.  This avoids treating transient, synthesized address
   records as the portable source of zone data.

Discussion Venue

   This note is to be removed before publishing as an RFC.

   Source for this draft and an issue tracker are available at
   https://github.com/querygraph/rgbdns/tree/master/ietf.

Status of This Memo

   This Internet-Draft is submitted in full conformance with the
   provisions of BCP 78 and BCP 79.

   Internet-Drafts are working documents of the Internet Engineering
   Task Force (IETF).  Note that other groups may also distribute
   working documents as Internet-Drafts.  The list of current Internet-
   Drafts is at https://datatracker.ietf.org/drafts/current/.

   Internet-Drafts are draft documents valid for a maximum of six months
   and may be updated, replaced, or obsoleted by other documents at any
   time.  It is inappropriate to use Internet-Drafts as reference
   material or to cite them other than as "work in progress."

   This Internet-Draft will expire on 31 January 2027.





Khrabrov                 Expires 31 January 2027                [Page 1]


Internet-Draft           ANAME and Zone Transfer               July 2026


Copyright Notice

   Copyright (c) 2026 IETF Trust and the persons identified as the
   document authors.  All rights reserved.

   This document is subject to BCP 78 and the IETF Trust's Legal
   Provisions Relating to IETF Documents (https://trustee.ietf.org/
   license-info) in effect on the date of publication of this document.
   Please review these documents carefully, as they describe your rights
   and restrictions with respect to this document.  Code Components
   extracted from this document must include Revised BSD License text as
   described in Section 4.e of the Trust Legal Provisions and are
   provided without warranty as described in the Revised BSD License.

Table of Contents

   1.  Introduction  . . . . . . . . . . . . . . . . . . . . . . . .   3
   2.  Requirements Language . . . . . . . . . . . . . . . . . . . .   3
   3.  Terminology . . . . . . . . . . . . . . . . . . . . . . . . .   4
   4.  The ANAME Resource Record . . . . . . . . . . . . . . . . . .   4
```
