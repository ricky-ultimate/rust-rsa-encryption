# `Rust RSA ENCRYPTION`

An RSA encryption implementation for secure data transmission, written in Rust.

## Algorith

1. Choose p and q
2. Compute n = p * q
3. Compute φ(n) = (p - 1) * (q - 1)
4. Choose e such that 1 < e < φ(n) and e and φ (n) are coprime.
5. Compute a value for d such that (d * e) % φ(n) = 1.
6. Public key is (e, n)
7. Private key is (d, n)
8. Encryption => c=m^e modn
9. 
