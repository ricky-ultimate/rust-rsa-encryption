# `Rust RSA ENCRYPTION`

An RSA encryption implementation for secure data transmission, written in Rust.

## Algorithm

1. Choose p and q
2. Compute n = p * q
3. Compute φ(n) = (p - 1) * (q - 1)
4. Choose e such that 1 < e < φ(n) and e and φ (n) are coprime.
5. Compute a value for d such that (d * e) % φ(n) = 1.
6. Public key is (e, n)
7. Private key is (d, n)

   ### Encryption
   Sender A does the following:-
   1. Obtains the recipient B's public key (n,e)
   2. Represents the plaintext message as a positive integer m with $` 1 < m < n `$
   3. Computes the ciphertext $`c = m^e mod n `$
   4. Sends the ciphertext c to B.
