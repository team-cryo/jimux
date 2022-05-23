let mut next: u64 = 1;

pub extern "C" fn rand()
{
    next = next * 1103515245 + 12345;
    (next / 65536) % 32768 as u32;
}

pub extern "C" fn srand(seed: u32)
{
    next = seed;
}
