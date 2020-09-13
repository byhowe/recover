from dataclasses import dataclass
from typing import List, Tuple
import re
import sys

INDENT: int = 2


def usage():
    """Print the usage information"""
    print("usage: python %s <NAME> <FILE>" % sys.argv[0])


if len(sys.argv) != 3:
    usage()
    exit(1)

with open(sys.argv[2], "r") as f:
    fields = f.read().split("\n")

struct_name = sys.argv[1]

TupleEntry = Tuple[int, str, str, str]


@dataclass
class Entry:
    offset: int
    data_type: str
    name: str
    desc: str
    array_length: int

    def get_byte_length(self) -> int:
        size = int(int(self.data_type[1:]) / 8)
        return size * self.array_length

    def get_data_type(self) -> str:
        if self.array_length > 1:
            return "[%s; %d]" % (self.data_type, self.array_length)
        else:
            return self.data_type


def entry_from_list(f: TupleEntry) -> Entry:
    offset = int(f[0], base=16)
    data_type = f[1].replace("__le", "u").replace("__u", "u")
    if data_type == "char":
        data_type = "u8"
    name = f[2]
    is_array = p.search(name)
    if is_array:
        (start, _) = is_array.span()
        array_length = int(name[start + 1 : -1])
        name = name[:start]
    else:
        array_length = 1
    desc = f[3]
    return Entry(offset, data_type, name, desc, array_length)


entries: List[Entry] = []

p = re.compile("\[.*\]")  # pylint: disable=W1401

print("#[repr(C)]")
print("#[derive(Debug)]")
print("pub(crate) struct %s {" % struct_name)

for f in fields:
    f = f.split("\t")
    f = [e.strip().replace("\n", " ") for e in f]
    try:
        if f[2] == "":
            if f[3] != "":
                print("\n" + INDENT * " " + "// %s\n" % f[3])
            continue
    except IndexError:
        if f[0] != "":
            print("\n" + INDENT * " " + "// %s\n" % f[0])
        continue

    entry = entry_from_list(f)

    if entry.desc != "":
        print(INDENT * " " + "/// %s" % entry.desc)
    print(
        INDENT * " "
        + "pub(crate) %s: %s, // %d - %d"
        % (
            entry.name,
            entry.get_data_type(),
            entry.offset,
            entry.offset + entry.get_byte_length(),
        )
    )

    entries.append(entry)

total_byte_length = sum([e.get_byte_length() for e in entries])

print("}")

print()

print("impl %s {" % struct_name)
print(" " * INDENT + "pub(crate) const WIDTH: usize = %d;" % total_byte_length)
print("}")

print()

print("impl From<&[u8; Self::WIDTH]> for %s {" % struct_name)

print(" " * INDENT + '#[cfg(target_endian = "little")]')
print(" " * INDENT + "fn from(block: &[u8; Self::WIDTH]) -> Self {")

print(" " * INDENT * 2 + "unsafe { std::mem::transmute(*block) }")

print(" " * INDENT + "}")

print()

print(" " * INDENT + '#[cfg(target_endian = "big")]')
print(" " * INDENT + "fn from(block: &[u8; Self::WIDTH]) -> Self {")

print(" " * INDENT * 2 + "let mut raw = unsafe { std::mem::transmute(*block) };")

for e in entries:
    if e.data_type == "u8":
        continue
    print(" " * INDENT * 2 + "raw.%s = " % e.name, end="")
    if e.array_length > 1:
        print("[")
        for i in range(e.array_length):
            print(
                " " * INDENT * 3 + "%s::from_le(raw.%s[%d])," % (e.data_type, e.name, i)
            )
        print(" " * INDENT * 2 + "]", end="")
    else:
        print("%s::from_le(raw.%s)" % (e.data_type, e.name), end="")
    print(";")

print(" " * INDENT + "}")

print("}")
