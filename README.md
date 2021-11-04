# facial

A safety protocol for cloud based on cryptography and TEE.

- SCCTEE: safety compute platform based on enclave.
- SCSP32: safety object storage.
- SKM32: smart key management hardware.

# Security Cloud Storage Protocol by BIP32

## File Key Tree

### Store Process

This process don't need any Secret Key from key management hardware.

Get file these parameters.

- Upper Secret Key: $s_u$
- Upper Seed: $e_u$
- Upper Public Key: $P_u$

Compute file's merkle root.

- Merkle Root: $m_0$

Compute file root keys.

- Root Secret Key: $s_0 = skd(s_u, m_0)$
- Root Seed: $e_0 = sdd(e_u, m_0)$
- Root Public Key: $P_0 = pkd(P_u, m_0)$

Compute each file chunk's hash and keys.

- Chunk Hash: $h_i$
- Chunk Secret Key: $s_i = skd(s_0, h_i)$
- Chunk Seed: $e_i = sdd(e_0, h_i)$
- Chunk Public Key: $P_i = pkd(P_0, h_i)$

Generate random encrypt keypair.

- Random Secret Key: $s^{\prime}_i$
- Random Public Key: $P^{\prime}_i$

Compute Synthetic Key by DH.

- Shared Key: $k = dh(P_i, s^{\prime}_i)$

Encryption file chunk.

- $C^{enc}_i = encrypt(C_i)$

Compute Signature.

- $S = sign(H(C^{enc}_i, P^{\prime}_i), s_i)$

Link file chunk.
- Encrypted file $F = (S, h_0, C^{enc}_i, P^{\prime}_i)$




