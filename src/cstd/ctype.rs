pub extern "C" fn isalnum(c: char) -> bool
{
    c.is_ascii_alphanumeric()
}

pub extern "C" fn islower(c: char) -> bool
{
    c.is_ascii_lowercase()
}

pub extern "C" fn isupper(c: char) -> bool
{
    c.is_ascii_uppercase()
}

pub extern "C" fn isdigit(c: char) -> bool
{
    false
}

pub extern "C" fn isxdigit(c: char) -> bool
{
    false
}

pub extern "C" fn iscntrl(c: char) -> bool
{
    false
}

pub extern "C" fn isgraph(c: char) -> bool
{
    false
}

pub extern "C" fn isspace(c: char) -> bool
{
    false
}

pub extern "C" fn isblank(c: char) -> bool
{
    false
}

pub extern "C" fn isprint(c: char) -> bool
{
    false
}

pub extern "C" fn ispunct(c: char) -> bool
{
    c.is_ascii_punctuation()
}

pub extern "C" fn tolower(c: char) -> char
{
    c.make_ascii_lowercase()
}

pub extern "C" fn toupper(c: char) -> char
{
    c.make_ascii_uppercase()
}
