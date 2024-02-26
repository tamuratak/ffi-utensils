struct mach_port_options {
    int a;
    union {
        int b;
        int c;
    };
};

union mach_port_options_union {
    int a;
    int b;
};