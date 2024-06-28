union pte {
    uint64_t value;
    struct {
        uint64_t V : 1;
        uint64_t R : 1;
        uint64_t W : 1;
        uint64_t X : 1;
        uint64_t U : 1;
        uint64_t G : 1;
        uint64_t A : 1;
        uint64_t D : 1;
        uint64_t RSW : 2;
        union {
            uint64_t ppn : 44;
            uint64_t PPN0 : 9;
            uint64_t PPN1 : 9;
            uint64_t PPN2 : 26;
        };
        uint64_t Reserved : 10;
    };
};