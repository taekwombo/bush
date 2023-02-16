#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

typedef enum { false, true } bool;

typedef int (*SetCompareFunction)(const void *left, const void* right);

typedef struct {
    size_t elemSize;
    size_t nodeSize;
    size_t length;
    size_t capacity;
    SetCompareFunction cmpFn;
    void *data;
} sortedset;

static const int kInitialCapacity = 8;
static const int kNotFound = -1;

static void Grow(sortedset *set)
{
    assert(set->length == set->capacity);
    size_t ncap = set->capacity * 2;
    set->data = realloc(set->data, set->nodeSize * ncap);
}

/*
 * Returns pointer to -1 if the element was not found in the set.
 *    Pointer is a location in memory of lt or gt field for a node that is a parent of the searched element.
 *    In other words, if element is added to the set -1 should be replaced with the index to new element.
 * 
 * Otherwise, returns pointer to the index of matchin element. 
 */
static int *FindNode(sortedset *set, void* elemAddr)
{
    void* current;
    int *index = set->data;

    // While node index is not -1 go deeper.
    while (*index != kNotFound) {
        // Get address of currenlty checked element and compare it with elemAddr.
        void *current = (char*)set->data + sizeof(int) + (*index * set->nodeSize);
        int cmp = set->cmpFn(current, elemAddr);

        // If element was found do nothing more.
        if (cmp == 0) {
            break;
        }

        // Get the lt index.
        index = (int*)((char*)current + set->elemSize);
        // Or gt index if current element was smaller than element at elemAddr.
        if (cmp < 0) {
            index++;
        }
    }

    return index;
}

/*
* Function: SetNew
* Usage: SetNew(&stringSet, sizeof(char *), StringPtrCompare);
* SetNew(&constellations, sizeof(pointT), DistanceCompare);
* ----------------
* SetNew allocates the requisite space needed to manage what
* will initially be an empty sorted set. More specifically, the
* routine allocates space to hold up to 'kInitialCapacity' (currently 4)
* client elements.
*/
void SetNew(sortedset *set, size_t elemSize, SetCompareFunction cmpFn)
{
    assert(elemSize > 0);
    assert(cmpFn != NULL);
    assert(set != NULL);

    set->elemSize = elemSize;
    set->length = 0;
    set->capacity = kInitialCapacity;
    set->cmpFn = cmpFn;
    set->nodeSize = sizeof(int) * 2 + elemSize;
    // Data: [int first_index, ...Node]
    // Node: [elem, padding when?, int lt, int gt]
    set->data = malloc(set->nodeSize * kInitialCapacity + sizeof(int));
    *(int*)set->data = kNotFound;
}

/*
* Function: SetAdd
* Usage: if (!SetAdd(&friendsSet, &name)) free(name);
* ----------------
* Adds the specified element to the set if not already present. If
* present, the client element is not copied into the set. true
* is returned if and only if the element at address elemPtr
* was copied into the set.
*/
bool SetAdd(sortedset *set, void* elemAddr)
{
    assert(set != NULL);
    assert(elemAddr != NULL);

    // Grow set if needed before retrieving pointers to the allocation.
    if (set->length == set->capacity) {
        Grow(set);
    }

    // If element is not present in the tree grow the memory for elements.
    int *nodeIndex = FindNode(set, elemAddr);

    // Return if element is already present in the set.
    if (*nodeIndex != kNotFound) {
        return false;
    }

    // Append new element to the end of allocated memory.
    void *dest = (char*)set->data + sizeof(int) + set->length * set->nodeSize;
    memcpy(dest, elemAddr, set->elemSize);
    *(int*)((char*)dest + set->elemSize) = kNotFound;
    *(int*)((char*)dest + set->elemSize + sizeof(int)) = kNotFound;

    // Set the index for the leaf node.
    *nodeIndex = set->length;
    // Increase the number of elements in the set.
    set->length++;

    return true;
}

/*
* Function: SetSearch
* Usage: if (SetSearch(&staffSet, &lecturer) == NULL)
* printf("musta been fired");
* -------------------
* SetSearch searches for the specified client element according
* the whatever comparison function was provided at the time the
* set was created. A pointer to the matching element is returned
* for successful searches, and NULL is returned to denote failure.
*/
void *SetSearch(sortedset *set, void *elemAddr)
{
    assert(set != NULL);
    assert(elemAddr != NULL);

    int *index = FindNode(set, elemAddr);

    // No element was found.
    if (*index == kNotFound) {
        return NULL;
    }

    return (char*)set->data + sizeof(int) + (*index * set->nodeSize);
}

// --------------------------------------------------------------------

// Comparator function for sortedset with int elements.
static int intcmp(const void* left, const void* right)
{
    return *(int*)left - *(int*)right;
}

// Debug function for sortedset with int elements.
static void DebugIntSet(sortedset *set)
{
    assert(set->elemSize == 4);
    printf("Set(start_index: %d):\n", *(int*)set->data);
    for (size_t i = 0; i < set->length; i++) {
        printf("    %lu -> data: %d lt: %d, gt: %d\n",
                i,
                *(int*)((char*)set->data + sizeof(int) + (i * set->nodeSize)),
                *(int*)((char*)set->data + sizeof(int) + (i * set->nodeSize) + sizeof(int)),
                *(int*)((char*)set->data + sizeof(int) + (i * set->nodeSize) + sizeof(int) + sizeof(int))
              );
    }
}

int main() {
    sortedset set;
    SetNew(&set, sizeof(int), intcmp);

    int items[6] = { 10, 8, 15, 3, 16, 9 };
    for (int i = 0; i < 6; i++) {
        int el = items[i];
        bool r = SetAdd(&set, &el);
    }

    DebugIntSet(&set);

    printf("memory:\n");
    printf("    [%p]      : %d\n", set.data, *(int*)set.data);
    for (int i = 0; i < set.length; i++) {
        int *el = (int*)((char*)set.data + sizeof(int) + (i * set.nodeSize));
        int *lt = el + 1;
        int *gt = el + 2;

        printf("    [%p]    %d : %d\n", el, i, *el);
        printf("    [%p]   lt : %d\n", lt, *lt);
        printf("    [%p]   gt : %d\n", gt, *gt);
    }

    int *fifteen = SetSearch(&set, &items[2]);

    for (int i = 0; i < 6; i++) {
        int el = items[i];
        printf("Searching for %d\n", el);
        int *r = SetSearch(&set, &el);
        assert(r != NULL);
        printf("    [%p]\n", r);
    }

    return 0;
}
