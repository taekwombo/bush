#include <stdio.h>
#include <sys/mman.h>
#include "./scip.pb-c.h"

const char* USAGE = ""
"USAGE                  \n"
"  tagscip <file>       \n"
"                       \n"
"EXAMPLE                \n"
"  tagscip ./index.scip \n"
;

#define LN(i, mak) printf("%*s" #mak "\n", i, "")
#define LNF(i, mak, ...) printf("%*s" #mak "\n", i, "", __VA_ARGS__)

void print_desc(ProtobufCMessage *m, char print_enc) {
    if (print_enc) {
        LNF(0, %s {, m->descriptor->name);
    }
    LNF(2, magic: %d, m->descriptor->magic);
    LNF(2, short_name: %s, m->descriptor->short_name);
    LNF(2, package_name: %s, m->descriptor->package_name);
    if (print_enc) {
        LN(0, });
    }
}

void print_index(Scip__Index *b) {
    LNF(0, %s {, b->base.descriptor->name);
    LNF(2, meta.project_root: %s, b->metadata->project_root);
    LNF(2, meta.text_encoding: %d, b->metadata->text_document_encoding);
    LNF(2, unknown_fields: %d, b->base.n_unknown_fields);
    LNF(2, documents: %d, b->n_documents);
    LNF(2, ext_sym: %d, b->n_external_symbols);
    print_desc(&b->base, 0);
    LN(0, });
}
int main(int argc, char **argv) {
    if (argc != 2 || !argv[1]) {
        fprintf(stderr, "%s", USAGE);
        return 1;
    }

    char* path = argv[1];

    FILE *f = fopen(path, "r");

    if (!f) {
        fprintf(stderr, "Failed to open file: %s", path);
        return 1;
    }

    fseek(f, 0, SEEK_END);
    size_t size = ftell(f);
    rewind(f);

    void *data = mmap(NULL, size, PROT_READ, MAP_PRIVATE, fileno(f), 0);

    if (data == MAP_FAILED) {
        fprintf(stderr, "Failed to read contents of file: %s", path);
        return 1;
    }

    printf("Parsing %ld bytes of %s.\n", size, path);

    Scip__Index *index = scip__index__unpack(NULL, size, data);

    if (!index) {
        fprintf(stderr, "Failed to parse file: %s", path);
        return 1;
    }

    for (size_t d = 0; d < index->n_documents; d++) {
        Scip__Document *doc = index->documents[d];

        printf("doc: %s\n", doc->relative_path);

        for (size_t s = 0; s < doc->n_symbols; s++) {
            Scip__SymbolInformation *sym = doc->symbols[s];
            if (sym->kind != SCIP__SYMBOL_INFORMATION__KIND__UnspecifiedKind) {
                printf("sym: %d - %s\n", sym->kind, sym->symbol);
            }
        }
    }

    print_index(index);

    return 0;
}

