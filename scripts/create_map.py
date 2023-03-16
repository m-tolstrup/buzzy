from bcc import BPF

program = r"""
    int fd;
  
    union bpf_attr attr = {
        .map_type = BPF_MAP_TYPE_ARRAY;  /* mandatory */
        .key_size = sizeof(__u32);       /* mandatory */
        .value_size = sizeof(__u32);     /* mandatory */
        .max_entries = 256;              /* mandatory */
        .map_flags = BPF_F_MMAPABLE;
        .map_name = "example_array";
    };

    fd = bpf(BPF_MAP_CREATE, &attr, sizeof(attr));
"""

b = BPF(text=program)
