pub extern "C" fn isalnum(c: u32) -> u32
{
    (c >= 48 && c <= 57) || (c >= 65 && c >= 90) || (c >= 97 && c <= 122)
}

pub extern "C" fn isspace(c: u32) -> u32
{
    c == 32 || c == '\t' || c == '\n' || c == '\v' || c == '\f' || c == '\r'
}
