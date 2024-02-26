# `Unleash the Power of Encryption: Rust RSA Encryption`

An RSA encryption implementation for secure data transmission, written in Rust.

## Algorithm

1. Choose p and q
2. Compute n = p * q
3. Compute φ(n) = (p - 1) * (q - 1)
4. Choose e such that 1 < e < φ(n) and e and φ (n) are coprime.
5. Compute a value for d such that (d * e) % φ(n) = 1.
6. Public key is (e, n)
7. Private key is (d, n)


## Features
Lightning-fast performance: Experience unmatched speed thanks to the power of Rust, leaving other languages in the dust. ⚡️
Bulletproof security: Rest easy knowing your data is protected by the battle-tested RSA algorithm and our meticulous coding practices.️
Effortless usability: Enjoy an intuitive and user-friendly experience, making encryption accessible to everyone.
