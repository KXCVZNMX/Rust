const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub fn encrypt(message: &str) -> String {
    fn ch(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (!x & z)
    }
    fn maj(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }
    fn cs0(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }
    fn cs1(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }
    fn s0(x: u32) -> u32 {
        x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
    }
    fn s1(x: u32) -> u32 {
        x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
    }

    fn pad(list: &Vec<u8>) -> Vec<u8> {
        let k = {
            let mut count = 0;
            while ((list.len() * 8) + 1 + count + 64) % 512 != 0 {
                count += 1;
            }
            (count - 7) / 8
        };
        let mut ret = list.clone();
        ret.push(0x80);
        ret.append(&mut vec![0x00; k]);
        let last_64_be = ((list.len() * 8) as u64).to_be_bytes();
        ret.extend_from_slice(&last_64_be);
        ret
    }

    fn make_block(padded: &Vec<u8>) -> Vec<[u32; 16]> {
        let n = padded.len() / 64;
        let mut ret: Vec<[u32; 16]> = Vec::new();
        for i in 0..n {
            let mut temp: [u32; 16] = [0; 16];
            let mut index = 0;
            for j in ((0 + i * 64)..(64 + i * 64)).step_by(4) {
                temp[index] =
                    ((padded[j] as u32) << 24)
                    | ((padded[j + 1] as u32) << 16)
                    | ((padded[j + 2] as u32) << 8)
                    | padded[j + 3] as u32;
                index += 1;
            }
            ret.push(temp);
        }
        ret
    }

    fn make_word(blocks: &Vec<[u32; 16]>) -> Vec<[u32; 64]> {
        let mut ret: Vec<[u32; 64]> = Vec::new();
        for i in 0..blocks.len() {
            let mut temp: [u32; 64] = [0; 64];
            for j in 0..16 {
                temp[j] = blocks[i][j];
            }
            for j in 16..64 {
                temp[j] = s1(temp[j - 2])
                    .wrapping_add(temp[j - 7])
                    .wrapping_add(s0(temp[j - 15]))
                    .wrapping_add(temp[j - 16]);
            }
            ret.push(temp);
        }
        ret
    }

    let (
        mut store_a,
        mut store_b,
        mut store_c,
        mut store_d,
        mut store_e,
        mut store_f,
        mut store_g,
        mut store_h,
    ) = (H[0], H[1], H[2], H[3], H[4], H[5], H[6], H[7]);
    
    let words = make_word(&make_block(&pad(&Vec::from(message.as_bytes()))));

    for i in 0..words.len() {
        let this_block = words[i];
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (
            store_a, store_b, store_c, store_d, store_e, store_f, store_g, store_h,
        );
        for j in 0..64 {
            let t1 = h.wrapping_add(cs1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[j])
                .wrapping_add(this_block[j]);
            let t2 = cs0(a).wrapping_add(maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        store_a = store_a.wrapping_add(a);
        store_b = store_b.wrapping_add(b);
        store_c = store_c.wrapping_add(c);
        store_d = store_d.wrapping_add(d);
        store_e = store_e.wrapping_add(e);
        store_f = store_f.wrapping_add(f);
        store_g = store_g.wrapping_add(g);
        store_h = store_h.wrapping_add(h);
    }

    format!(
        "{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
        store_a, store_b, store_c, store_d, store_e, store_f, store_g, store_h
    )
}

#[cfg(test)]
mod test {
    use rand::{Rng, distributions::Alphanumeric};
    use sha2::{Digest, Sha256};
    use super::*;

    #[test]
    fn foo() {
        for i in 0..5000 {
            let rand_string: String = (0..i)
                .map(|_| rand::thread_rng().sample(Alphanumeric) as char)
                .collect();

            let mut hasher = Sha256::new();
            hasher.update(rand_string.as_bytes());
            let result = hasher.finalize();
            let finalised = format!("{:x}", result);
            assert_eq!(encrypt(rand_string.as_str()), finalised);

        }
    }
}