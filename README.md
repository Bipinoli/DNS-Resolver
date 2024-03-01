# DNS resolver

<img width="693" alt="Screenshot 2024-03-01 at 11 19 14" src="https://github.com/Bipinoli/DNS-Resolver/assets/11765482/e08409bf-196d-4928-a21b-b1b7ffd80125">

It is an implementation of a DNS resolution tool similar to dig.
I have implemented it based on the DNS protocol specified in [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035).

It is by no means complete and is purely made for fun.

Before starting the implementation, I skimmed over [this](https://github.com/EmilHernvall/dnsguide/tree/master) guide.
I started with his workflow of using Netcat + Hexdump to understand the DNS packets. However, later I found that Tcpdump + Wireshark works better for me.


## Tools used to understand the protocol:
- Official [RFC](https://datatracker.ietf.org/doc/html/rfc1035)
- Tcpdump
- Wireshark (to view tcpdump pcap)
- Netcat

