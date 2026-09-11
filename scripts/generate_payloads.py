#!/usr/bin/env python3
import sys
import os

def bin_to_c_header(source_path, var_name, out_path):
    with open(source_path, "rb") as f:
        data = f.read()
    
    with open(out_path, "w") as f:
        f.write(f"// Auto-generated payload from {os.path.basename(source_path)}\n")
        f.write(f"#ifndef {var_name.upper()}_H\n")
        f.write(f"#define {var_name.upper()}_H\n\n")
        f.write(f"#include <stddef.h>\n\n")
        f.write(f"static const size_t {var_name}_len = {len(data)};\n")
        f.write(f"static const unsigned char {var_name}_data[] = {{\n")
        
        # Write bytes in chunks of 16 for readable C code
        for i in range(0, len(data), 16):
            chunk = data[i:i+16]
            hex_str = ", ".join(f"0x{b:02x}" for b in chunk)
            f.write(f"    {hex_str},\n")
            
        f.write("};\n\n")
        f.write(f"#endif // {var_name.upper()}_H\n")
    print(f"Generated {out_path} ({len(data)} bytes)")

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: generate_payloads.py <binary_path> <var_name> <output_header>")
        sys.exit(1)
    bin_to_c_header(sys.argv[1], sys.argv[2], sys.argv[3])
