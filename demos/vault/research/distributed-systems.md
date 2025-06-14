# Distributed Systems Research

## Key Concepts

### CAP Theorem
- Consistency
- Availability  
- Partition tolerance
- Can only guarantee 2 out of 3

### Consensus Algorithms
- Raft
- Paxos
- Byzantine fault tolerance

### Content-Addressable Storage
- Git's approach
- IPFS model
- Merkle trees

## Application to Our System

Git already provides:
- Content addressing via SHA-1
- Distributed replication
- Merge algorithms

We can leverage these for semantic links without reinventing the wheel.

## References
- "Designing Data-Intensive Applications" - Kleppmann
- Git's object model documentation
- IPFS whitepaper