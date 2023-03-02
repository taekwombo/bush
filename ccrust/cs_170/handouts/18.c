// vim: set fdm=marker:

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "vector.h"
#include "hashset.h"

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

#ifdef P3
// Problem 3: multitable
typedef int (*MultiTableHashFunction)(const void *keyAddr, int numBuckets);
typedef int (*MultiTableCompareFunction)(const void *keyAddr1, const void *keyAddr2);
typedef void (*MultiTableMapFunction)(void *keyAddr, void *valueAddr, void *auxData);

/**
 * mappings item:
 * [<keySize bytes>, vector]
 */
typedef struct {
    hashset mappings;
    int keySize;
    int valueSize;
} multitable;

/**
 * Function: MultiTableNew
 * -----------------------
 * Initializes the raw space addressed by mt to be an empty
 * multitable otherwise capable of storing keys and values of
 * the specified sizes. The numBuckets, hash, and compare parameters
 * are supplied with the understanding that they will simply be passed to HashSetNew,
 * as the interface clearly advertises that a hashset is used.
 * You should otherwise interact with the hashset (and any vectors) using only
 * functions which have the authority to manipulate them.
 */
void MultiTableNew(multitable *mt, int keySizeInBytes, int valueSizeInBytes,
    int numBuckets, MultiTableHashFunction hashfn, MultiTableCompareFunction cmpfn)
{
    assert(hashfn != NULL);
    assert(cmpfn != NULL);

    mt->keySize = keySizeInBytes;
    mt->valueSize = valueSizeInBytes;

    HashSetNew(&mt->mappings, keySizeInBytes + sizeof(vector), numBuckets, hashfn, cmpfn, NULL);
}

/**
 * Function: MultiTableMap
 * -----------------------
 * Applies the specified MultiTableMapFunction to each key/value pair
 * stored inside the specified multitable. The auxData parameter
 * is ultimately channeled in as the third parameter to every single
 * invocation of the MultiTableMapFunction. Just to be clear, a
 * multitable with seven keys, where each key is associated with
 * three different values, would prompt MultiTableMap to invoke the
 * specified MultiTableMapFunction twenty-one times.
 */
void MultiTableEnter(multitable *mt, const void *keyAddr, const void *valueAddr)
{
    void *res = HashSetLookup(&mt->mappings, keyAddr);
    if (res == NULL) {
        void *kv = alloca(mt->keySize + sizeof(vector));
        vector *vec = (vector*)((char*)kv + mt->keySize);
        memcpy(kv, keyAddr, mt->keySize);
        VectorNew(vec, mt->valueSize, NULL, 4);
        VectorAppend(vec, valueAddr);
        HashSetEnter(&mt->mappings, kv);
    } else {
        // Add to vector.
        vector *vec = (vector*)((char*)res + mt->keySize);
        VectorAppend(vec, valueAddr);
    }
}

typedef struct {
    void *auxData;
    multitable *mt;
    MultiTableMapFunction mapfn;
} mt_map;

void MultitableHashsetMap(void *elemAddr, void *auxData)
{
    mt_map *data = auxData;
    void *keyAddr = elemAddr;
    vector *vec = (vector*)((char*)elemAddr + data->mt->keySize);
    int len = VectorLength(vec);
    int i = 0;
    while (i < len) {
        data->mapfn(keyAddr, VectorNth(vec, i), data->auxData);
        i++;
    }
}

/**
 * Function: MultiTableMap
 * -----------------------
 * Applies the specified MultiTableMapFunction to each key/value pair
 * stored inside the specified multitable. The auxData parameter
 * is ultimately channeled in as the third parameter to every single
 * invocation of the MultiTableMapFunction. Just to be clear, a
 * multitable with seven keys, where each key is associated with
 * three different values, would prompt MultiTableMap to invoke the
 * specified MultiTableMapFunction twenty-one times.
 */
void MultiTableMap(multitable *mt, MultiTableMapFunction map, void *auxData)
{
    mt_map data = {
        .auxData = auxData,
        .mt = mt,
        .mapfn = map,
    };
    HashSetMap(&mt->mappings, MultitableHashsetMap, &data);
}

int IntHash(const void *value, int numBuckets)
{
    assert(*(int*)value >= 0);
    return *(int*)value % numBuckets;
}

int IntCmp(const void *left, const void *right)
{
    return left - right;
}

// Map Function that increments `auxData` by 1 on each invocation.
// Expects auxData to be *int.
void IntMap(void *key, void *value, void *auxData)
{
    *(int*)auxData += 1;
}

int main()
{
    multitable mt;
    MultiTableNew(&mt, sizeof(int), sizeof(int), 10, IntHash, IntCmp);

    // Insert 20 keys.
    for (int i = 0; i < 20; i++) {
        MultiTableEnter(&mt, &i, &i);
    }

    int aux = 0;

    MultiTableMap(&mt, IntMap, &aux);

    printf("Map function called: %d times.\n", aux);

    assert(aux == 20);

    return 0;
}
#endif
