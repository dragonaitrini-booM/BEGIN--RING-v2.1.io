#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/in.h>
#include <linux/tcp.h>
#include <linux/udp.h>
#include <linux/icmp.h>
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "GPL";

// BPF map for allowed UDP ports (e.g., WireGuard)
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 128);
    __type(key, __u16); // port in network byte order
    __type(value, __u8); // dummy value
} allowed_udp_ports SEC(".maps");

// BPF map for the DoH resolver IP
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, __u32); // IP address in network byte order
    __type(value, __u8); // dummy value
} doh_resolver_ip SEC(".maps");

SEC("cgroup/skb")
int bpf_firewall(struct __sk_buff *skb) {
    struct ethhdr *eth = (struct ethhdr *)(long)skb->data;
    if ((void *)(eth + 1) > (void *)(long)skb->data_end)
        return 0; // Drop malformed packets

    if (eth->h_proto != __constant_htons(ETH_P_IP))
        return 0; // Drop non-IP packets

    struct iphdr *ip = (struct iphdr *)(long)(skb->data + sizeof(struct ethhdr));

    if ((void *)(ip + 1) > (void *)(long)skb->data_end)
        return 0; // Drop malformed packets

    if (ip->ihl < 5)
        return 0; // IHL must be at least 5, otherwise drop

    __u32 ip_header_len = ip->ihl * 4;

    if ((void *)ip + ip_header_len > (void *)(long)skb->data_end)
        return 0;

    // 4. Tunneling & Evasion Annihilator (Placeholders)
    // A more complete implementation would include checks for:
    // - DNS tunneling (entropy analysis on domain names)
    // - ICMP tunneling (payload size and content analysis)
    // - Fragmentation attacks (e.g., tiny fragments, overlapping fragments)
    // These would likely be implemented as separate helper functions called from here.


    if (ip->protocol == IPPROTO_TCP) {
        struct tcphdr *tcp = (struct tcphdr *)((void *)ip + ip_header_len);
        if ((void *)(tcp + 1) > (void *)(long)skb->data_end)
            return 0; // Drop malformed packets

        // 3. Protocol & Port Guard: Allow ONLY TCP 443 (HTTPS)
        if (tcp->dest == __constant_htons(443)) {
            // Placeholder for Step 7: Application-Level Gateway (ALG) Emulation for HTTPS
            // Here we would inspect the SNI in the ClientHello
            return 1; // Allow
        }

        // Block all other TCP traffic
        return 0;

    } else if (ip->protocol == IPPROTO_UDP) {
        struct udphdr *udp = (struct udphdr *)((void *)ip + ip_header_len);
        if ((void *)(udp + 1) > (void *)(long)skb->data_end)
            return 0; // Drop malformed packets

        // 3. Protocol & Port Guard: Allow UDP for WireGuard and DoH via BPF maps

        // Handle DoH on port 53 - must go to a specific resolver
        if (udp->dest == __constant_htons(53)) {
            __u32 dest_ip = ip->daddr;
            void *is_doh_resolver = bpf_map_lookup_elem(&doh_resolver_ip, &dest_ip);
            if (is_doh_resolver) {
                return 1; // Allow
            }
        } else {
            // Handle other dynamic UDP ports (e.g., WireGuard)
            __u16 dest_port = udp->dest;
            void *is_allowed_port = bpf_map_lookup_elem(&allowed_udp_ports, &dest_port);
            if (is_allowed_port) {
                return 1; // Allow
            }
        }

        // Block all other UDP traffic
        return 0;

    } else if (ip->protocol == IPPROTO_ICMP) {
        struct icmphdr *icmp = (struct icmphdr *)((void *)ip + ip_header_len);
        if ((void *)(icmp + 1) > (void *)(long)skb->data_end)
            return 0; // Drop malformed packets

        // 3. Protocol & Port Guard: Allow ONLY ICMP type 8 (echo request) for IPv4.
        // This allows outgoing pings. The original spec incorrectly requested type 128 (echo reply).
        if (icmp->type == 8) {
            // Placeholder for ICMP rate limiting (would use a BPF map)
            return 1; // Allow
        }
        return 0; // Drop all other ICMP types

    } else if (ip->protocol == 41) {
        // 4. Tunneling & Evasion Annihilator: Block IPv6 over IPv4
        return 0; // Drop
    }

    // Default deny for any other protocols
    return 0;
}
