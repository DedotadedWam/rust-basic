Elements in a linked list are known as nodes.
Each node contains a key and a pointer to its successor node, known as next.
The attribute named head points to the first element of the linked list.
The last element of the linked list is known as the tail.

Singly linked list — Traversal of items can be done in the forward direction only.
Doubly linked list — Traversal of items can be done in both forward and backward directions. Nodes consist of an additional pointer known as prev, pointing to the previous node.
Circular linked lists — Linked lists where the prev pointer of the head points to the tail and the next pointer of the tail points to the head.

Search: Find the first element with the key k in the given linked list by a simple linear search and returns a pointer to this element
Insert: Insert a key to the linked list. An insertion can be done in 3 different ways; insert at the beginning of the list, insert at the end of the list and insert in the middle of the list.
Delete: Removes an element x from a given linked list. You cannot delete a node by a single step. A deletion can be done in 3 different ways; delete from the beginning of the list, delete from the end of the list and delete from the middle of the list.
