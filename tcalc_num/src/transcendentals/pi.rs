use num::{BigInt, FromPrimitive, One, Zero};

// Pi spigot algorithm from https://github.com/transmogrifier/pidigits/tree/master

fn comp(
    a: (BigInt, BigInt, BigInt, BigInt),
    b: (BigInt, BigInt, BigInt, BigInt),
) -> (BigInt, BigInt, BigInt, BigInt) {
    let (q, r, s, t) = a;
    let (u, v, w, x) = b;
    (
        q.clone() * u.clone() + r.clone() * w.clone(),
        q * v.clone() + r * x.clone(),
        s.clone() * u + t.clone() * w,
        s * v + t * x,
    )
}

fn prod(a: (BigInt, BigInt, BigInt, BigInt), n: BigInt) -> (BigInt, BigInt, BigInt, BigInt) {
    comp((BigInt::from_u32(10).unwrap(), BigInt::from_i32(-10).unwrap() * n, BigInt::zero(), BigInt::one()), a)
}

fn safe(b: (BigInt, BigInt, BigInt, BigInt), n: BigInt) -> bool {
    let a = extr(b, BigInt::from_u32(4).unwrap());
    n == a.0 / a.1
}

fn extr(a: (BigInt, BigInt, BigInt, BigInt), x: BigInt) -> (BigInt, BigInt) {
    let (q, r, s, t) = a;
    (q * x.clone() + r, s * x + t)
}

fn next(z: (BigInt, BigInt, BigInt, BigInt)) -> BigInt {
    let a = extr(z, BigInt::from_u32(3).unwrap());
    a.0 / a.1
}

fn lfts(k: BigInt) -> (BigInt, BigInt, BigInt, BigInt) {
    (k.clone(), 4 * k.clone() + 2, BigInt::zero(), 2 * k + 1)
}

pub fn pi_digits() -> impl Iterator<Item = BigInt> {
    gen {
        let mut k = BigInt::one();
        let mut z = (BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one());
        loop {
            let lft = lfts(k.clone());
            let n = next(z.clone());
            if safe(z.clone(), n.clone()) {
                z = prod(z, n.clone());
                yield n
            } else {
                z = comp(z, lft);
                k += 1
            }
        }
    }
}
