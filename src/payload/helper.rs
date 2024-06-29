const fn const_max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

pub const fn size_add(v0: Option<usize>, v1: Option<usize>) -> Option<usize> {
    if let Some(v0) = v0 {
        if let Some(v1) = v1 {
            return Some(v0 + v1);
        }
    }
    
    None
}

pub const fn size_mul(v0: Option<usize>, v1: usize) -> Option<usize> {
    if let Some(len) = v0 {
        Some(len * v1)
    } else {
        None
    }
}

pub const fn size_max(v0: Option<usize>, v1: Option<usize>) -> Option<usize> {
    if let Some(v0) = v0 {
        if let Some(v1) = v1 {
            return Some(const_max(v0, v1));
        }
    }

    None
}

pub const fn size_array<const N: usize>(v0: [Option<usize>; N]) -> Option<usize> {
    let mut total_size = 0usize;
    let mut i = 0;

    while i < N {
        match v0[i] {
            Some(size) => total_size += size,
            None => return None,
        }
        i += 1;
    }

    Some(total_size)
}