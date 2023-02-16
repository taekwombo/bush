#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

typedef enum {
    List,
    Integer,
    String,
} NodeType;

typedef struct Node {
    NodeType type;
    struct Node* next;  // Next Node in the list.
    size_t data;        // Pointer or data.
} Node;

static Node* createIntNode(int value) {
    Node* node = malloc(sizeof(Node));
    assert(node);

    node->type = Integer;
    node->next = NULL;
    node->data = value;

    return node;
}

static Node* createStringNode(const char* value) {
    char* str = malloc(sizeof(char) * strlen(value) + 1);
    strcpy(str, value);

    Node* node = malloc(sizeof(Node));
    assert(node);

    node->type = String;
    node->next = NULL;
    node->data = (size_t)str;

    return node;
}

static void linkNodes(Node* node, Node* next_node) {
    assert(node->next == NULL);
    node->next = next_node;
}

static void destroyNode(Node* node) {
    switch (node->type) {
        case Integer:
            free(node);
            break;
        case String:
            free((char*)node->data);
            free(node);
            break;
        default:
            assert(1 == 0);
            break;
    }
}

void printNode(Node* node) {
    switch (node->type) {
        case Integer:
            printf(
                "Node {\n  type: %d,\n  next: %p,\n  data: %d\n}\n",
                node->type,
                node->next,
                (int)node->data
            );
            break;
        case String:
            printf(
                "Node {\n  type: %d,\n  next: %p,\n  data: \"%s\"\n}\n",
                node->type,
                node->next,
                (char*)node->data
            );
            break;
        default:
            assert(1 == 0);
            break;
    }
}

static char* concatStrings(const char* prefix, const char* suffix) {
    char* res = malloc(sizeof(char) * (strlen(prefix) + strlen(suffix)));

    strcpy(res, prefix);
    strcat(res, suffix);

    return res;
}

static char* concatAllStringNodes(Node* node) {
    Node* str_node = node;

    // Find first String Node.
    while (str_node != NULL && str_node->type != String) {
        switch (str_node->type) {
            case String:
                break;
            default:
                str_node = str_node->next;
        }
    }

    assert(str_node != NULL);

    int is_node_owned_str = 1;
    char* concat_result = (char*)str_node->data;
    assert(concat_result != NULL);

    str_node = str_node->next;

    while (str_node != NULL) {
        if (str_node->type == String) {
            char* tmp = concatStrings(concat_result, (char*)str_node->data);

            if (is_node_owned_str != 1) {
                free(concat_result);
            } else {
                is_node_owned_str = 0;
            }

            concat_result = tmp;
        }

        str_node = str_node->next;
    }

    return concat_result;
}

int main() {
    Node* nodes[] = {
        createIntNode(0),
        createStringNode("Purple"),
        createStringNode("is my"),
        createStringNode("favourite"),
        createStringNode("colour"),
    };

    // List should look like this:
    // (0 -> Purple ->
    //                (is my -> favourite -> colour))
    // is my -> favourite
    linkNodes(nodes[2], nodes[3]);
    // favourite -> colour
    linkNodes(nodes[3], nodes[4]);
    // 0 -> Purple
    linkNodes(nodes[0], nodes[1]);
    // Purple -> (is my ...)
    linkNodes(nodes[1], nodes[2]);

    printf("List says: \"%s\"\n", concatAllStringNodes(nodes[0]));
}
