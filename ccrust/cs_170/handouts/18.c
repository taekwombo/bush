// vim: set fdm=marker:

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "vector.h" // Implementation from assignment 3.

#ifdef P1
// Problem 1: sparsestringarray {{{
// A sparsestringarray is an array-like data structure that provides constant
// time access to its elements, and constant time insertion and deletion.
// It layers array semantics over an ordered collection of C strings,
// with the understanding that most of the strings are empty.

typedef struct {
    /** Set to be of size 'groupSize'. */
    bool *bitmap;
    /** Vector of dynamically allocated, nonempty C strings. */
    vector strings;
} group;

typedef struct {
    /** Dynamically allocated array of structs. */
    group *groups;
    /** Number of groups in the sparsestringarray. */
    int numGroups;
    /** Logical length of the full sparsestringarray. */
    int arrayLength;
    /** Number of strings managed by each group. */
    int groupSize;
} sparsestringarray;

typedef void SSAMapFunction(int index, const char *str, void *auxData);

static const char* kEmptyString = "";

static void StringFree(void *elem)
{
    char *str = *(char**)elem;
    if (str != kEmptyString) {
        free(str);
    }
}

/**
 * Requirements:
 * - arrayLength > 0
 * - groupSize > 0
 */
void SSANew(sparsestringarray *ssa, int arrayLength, int groupSize)
{
    assert(arrayLength > 0);
    assert(groupSize > 0);

    ssa->arrayLength = arrayLength;
    ssa->groupSize = groupSize;
    ssa->numGroups = ((arrayLength - 1) / groupSize) + 1;
    ssa->groups = malloc(sizeof(group) * ssa->numGroups);

    for (int i = 0; i < ssa->numGroups; i++) {
        group *g = &(ssa->groups[i]);
        g->bitmap = malloc(sizeof(bool) * groupSize);
        memset(g->bitmap, 0, sizeof(bool) * groupSize);
        VectorNew(&g->strings, sizeof(char*), StringFree, 1);
    }
}

bool SSAInsert(sparsestringarray *ssa, int index, const char *str)
{
    int groupIndex = index / ssa->groupSize;
    assert(groupIndex <= ssa->numGroups && groupIndex >= 0);

    group *g = &ssa->groups[groupIndex];
    int bitmapIndex = index % ssa->groupSize;

    int vecIndex = 0;
    for (int i = 0; i < bitmapIndex; i++) {
        if (g->bitmap[i] == true) {
            vecIndex++;
        }
    }

    const char *s = strdup(str);
    if (g->bitmap[bitmapIndex] == true) {
        // Free currently stored string and then replace.
        StringFree(VectorNth(&g->strings, vecIndex));
        VectorReplace(&g->strings, &s, vecIndex);
        return true;
    }

    g->bitmap[bitmapIndex] = true;
    VectorInsert(&g->strings, &s, vecIndex);
    return false;
};

void SSAMap(sparsestringarray *ssa, SSAMapFunction mapfn, void *auxData)
{
    for (int g = 0; g < ssa->numGroups; g++) {
        const group *gr = &ssa->groups[g];

        // Current group max index.
        int groupSize = ssa->groupSize;
        // If it is the last group it may not have the full size.
        if (g + 1 == ssa->numGroups && ssa->arrayLength % ssa->groupSize > 0) {
            groupSize = ssa->arrayLength % ssa->groupSize;
        }
        int idx = g * ssa->groupSize;
        int vecIdx = 0;

        for (int i = 0; i < groupSize; i++) {
            if (gr->bitmap[i] == true) {
                mapfn(idx + i, *(char**)VectorNth(&gr->strings, vecIdx++), auxData);
            } else {
                mapfn(idx + 1, kEmptyString, auxData);
            }
        }
    }
}

void SSADispose(sparsestringarray *ssa)
{
    for (int g = 0; g < ssa->numGroups; g++) {
        group *gr = &ssa->groups[g];
        VectorDispose(&gr->strings);
        free((void*)gr->bitmap);
    }
    free((void*)ssa->groups);
}

static void CountEmptyPrintNonEmpty(int index, const char *str, void *auxData) {
    if (str != kEmptyString) {
        printf("Oooo! Nonempty string at index %d: \"%s\"\n", index, str);
    } else {
        (*(int *)auxData)++;
    }
}

int main(int argc, char **argv) {
    sparsestringarray ssa;
    SSANew(&ssa, 70000, 35);

    SSAInsert(&ssa, 33001, "need");
    SSAInsert(&ssa, 58291, "more");
    SSAInsert(&ssa, 33000, "Eye");
    SSAInsert(&ssa, 33000, "I");
    SSAInsert(&ssa, 67899, "cowbell");

    int numEmptyStrings = 0;
    SSAMap(&ssa, CountEmptyPrintNonEmpty, &numEmptyStrings);
    printf("%d of the strings were empty strings.\n", numEmptyStrings);
    SSADispose(&ssa);
    return 0;
}

// Problem1: }}}
#endif

#ifdef P2
// Problem 2: serializeList {{{

size_t *serializeList(size_t *node)
{
    // Total length of serialized text including \0.
    size_t strLen = 0;
    // Number of text chunks.
    size_t nodes = 0;
    // Result: [size_t (number of chunks)][text][text]
    size_t *result = malloc(sizeof(size_t));

    while (node != NULL) {
        char *text = (char*)(&node[1]);
        size_t len = strlen(text);

        if (len > 0) {
            result = realloc(result, sizeof(size_t) + strLen + len + 1);
            nodes++;
            strcpy((char*)result + sizeof(size_t) + strLen, text);
            strLen += len + 1;
        }

        // Deref next node address.
        node = (size_t*)*node;
    }

    *result = nodes;
    return result;
}

#define NODE(name, text) size_t *name = malloc(sizeof(size_t) + sizeof(text) + 1);\
                         strcpy((char*)name + sizeof(size_t), text);\
                         *name = (size_t)NULL;
#define NODE_WCHILD(name, text, child) NODE(name, text)\
                                       *name = (size_t)child;
int main(int argc, char **argv) {
    assert(*serializeList(NULL) == 0);

    // Single item in the list.
    NODE(single, "Lonely...");
    assert(*serializeList(single) == 1);
    free(single);

    // Multiple items int the list.
    NODE(last, "Last one.");
    NODE_WCHILD(lsnd, "Almost over.", last);
    NODE_WCHILD(second, "I am second.", lsnd);
    NODE_WCHILD(root, "First!", second);

    size_t *res = serializeList(root);
    assert(*res == 4);

    char *text = (char*)res + sizeof(size_t);
    for (size_t i = 0; i < *res; i++) {
        printf("%s\n", text);
        text += strlen(text) + 1;
    }

    free(last);
    free(lsnd);
    free(second);
    free(root);

    return 0;
}
// Problem: 2 }}}
#endif
